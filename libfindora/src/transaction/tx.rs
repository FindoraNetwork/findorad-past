use abcf::ToBytes;
use capnp::{message::ReaderOptions, serialize_packed};
use digest::Digest;
use primitive_types::H512;
use sha3::Sha3_512;
use zei::xfr::{sig::XfrKeyPair, structs::AssetTypeAndAmountProof};

use crate::{transaction_capnp, Address, Error, Result};

use super::{
    bytes::{deserialize, serialize},
    signature::Signature,
    FraSignature, Input, Memo, Output,
};

#[derive(Debug)]
pub struct Transaction {
    pub txid: H512,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
    pub signatures: Vec<Signature>,
    pub memos: Vec<Memo>,
}

impl abcf::Transaction for Transaction {}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            txid: H512::default(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: AssetTypeAndAmountProof::NoProof,
            signatures: Vec::new(),
            memos: Vec::new(),
        }
    }
}

impl abcf::module::FromBytes for Transaction {
    fn from_bytes(bytes: &[u8]) -> abcf::Result<Self>
    where
        Self: Sized,
    {
        Ok(Transaction::deserialize(bytes)?)
    }
}

impl Transaction {
    pub fn deserialize(bytes: &[u8]) -> Result<Self> {
        let reader = serialize_packed::read_message(bytes, ReaderOptions::new())?;
        let root = reader.get_root::<transaction_capnp::transaction::Reader>()?;

        deserialize::from_root(root)
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut result = Vec::new();

        let mut message = capnp::message::Builder::new_default();
        let transaction = message.init_root::<transaction_capnp::transaction::Builder>();

        serialize::build_transaction(self, transaction)?;
        serialize_packed::write_message(&mut result, &message)?;

        Ok(result)
    }

    pub fn signature(&mut self, keypairs: &[XfrKeyPair]) -> Result<()> {
        if !self.signatures.is_empty() {
            return Err(Error::AlreadySign);
        }

        let bytes = self.serialize()?;

        for keypair in keypairs {
            let address = Address::from(keypair.get_pk());

            let public_key = keypair.get_pk();

            let signature = keypair.sign(&bytes);

            self.signatures.push(Signature::Fra(FraSignature {
                address,
                public_key,
                signature,
            }))
        }

        let bytes = self.serialize()?;

        let txid = Sha3_512::digest(&bytes);

        self.txid = H512::from_slice(txid.as_slice());

        Ok(())
    }

    pub fn verify(&self, _bytes: &[u8]) -> Result<bool> {
        // verify tx signature using  here.

        // skip memo for ethereum.

        Ok(true)
    }
}

impl ToBytes for Transaction {
    fn to_bytes(&self) -> abcf::Result<Vec<u8>> {
        Ok(self.serialize()?)
    }
}
