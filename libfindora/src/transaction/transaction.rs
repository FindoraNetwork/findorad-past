use abcf::ToBytes;
use capnp::{message::ReaderOptions, serialize_packed};
use primitive_types::H512;
use zei::xfr::{sig::XfrKeyPair, structs::AssetTypeAndAmountProof};

use crate::{transaction_capnp, Result};

use super::{
    bytes::{deserialize, serialize},
    signature::Signature,
    Input, Output,
};

#[derive(Debug)]
pub struct Transaction {
    pub txid: H512,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
    pub signatures: Vec<Signature>,
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
        }
    }
}

impl abcf::module::FromBytes for Transaction {
    fn from_bytes(bytes: &[u8]) -> abcf::Result<Self>
    where
        Self: Sized,
    {
        fn inner(bytes: &[u8]) -> Result<Transaction> {
            let reader = serialize_packed::read_message(bytes, ReaderOptions::new())?;
            let root = reader.get_root::<transaction_capnp::transaction::Reader>()?;

            deserialize::from_root(root)
        }

        Ok(inner(bytes)?)
    }
}

impl Transaction {
    pub fn signature(&mut self, _keypairs: Vec<XfrKeyPair>) -> Result<()> {
        //         if self.signatures.len() != 0 {
        //     return Err(eg!("this tx is signed."));
        // }
        //
        // if self.inputs.len() != keypairs.len() {
        //     return Err(eg!("please give right keypair for inputs."));
        // }
        //
        // let bytes = self.to_bytes().map_err(|e| eg!(format!("{:?}", e)))?;
        //
        // for i in 0..keypairs.len() {
        //     let keypair = &keypairs[i];
        //
        //     let signature = keypair.sign(&bytes);
        //
        //     self.signatures.push(signature);
        // }
        //
        // let bytes = self.to_bytes().map_err(|e| eg!(format!("{:?}", e)))?;
        //
        // let tx_hash = Sha3_512::digest(&bytes).as_slice();
        //
        // self.txid = H512(Sha3_512::digest(&bytes).try_into()?);
        //
        Ok(())
    }
}

impl ToBytes for Transaction {
    fn to_bytes(&self) -> abcf::Result<Vec<u8>> {
        fn inner(t: &Transaction) -> Result<Vec<u8>> {
            let mut result = Vec::new();

            let mut message = capnp::message::Builder::new_default();
            let transaction = message.init_root::<transaction_capnp::transaction::Builder>();

            serialize::build_transaction(t, transaction)?;
            serialize_packed::write_message(&mut result, &message)?;

            Ok(result)
        }

        Ok(inner(self)?)
    }
}
