# findorad

## Install Dependencies

Install [Cap’n Proto](https://capnproto.org/index.html) on Ubuntu platform :

```
sudo apt-get install capnproto
```
Install `libssl-dev`
```
sudo apt-get install libssl-dev
```

## Start Node

### Single Node

Run single node development environment:

``` bash
# Using source code.
$ cargo run --bin findorad -- --dev --enable-web3

# Or using precompile binary.
$ findorad --dev --enable-web3
```

### Local Cluster for Staking

Start multiple nodes development environment:

``` bash
# Using source code.
$ cargo run --bin findorad -- --dev-staking --enable-web3

# Or using precompile binary.
$ findorad --dev-staking --enable-web3
```

## Features

### Native Token

There is currently only one UTXO-based ledger in the network,
All native assets support `Confidential` transaction or `NonConfidential` transaction.
Native assets use a 32-byte identifier to identify the asset type, and the attributes of asset can be customized.

- transferable: Is this type of token transferable?
- owner: The owner of the asset can issue additional assets.
- splitable:
  - If the asset is splitable, it can transfer like ERC20; Can be transfered in `Confidential` or `NonConfidential`.
  - If the asset isn't splitable, it can transfer like ERC721; Can only be transfered in `NonConfidential`.

#### Hash Address (HA)

Like other blockchains like Ethereum or Bitcoin, Hash Address (HA) is a basic type of address.
The generation rule of the hash addressed is same as the Ethereum, and the address length is 20 bytes.
But due to zei requirements, the algorithm used here is `ED25519`. The HA can generate from the public key.

The bech32 format of this HA's prefix is `fraha`, At the same time, the length is the same as the Ethereum,
so the `0x` format can be generated. **But the HA can only be used in `NonConfidential` transactions**

Special, all bytes of the HA are `0` is a black hole address, which means that this output will be burned.

#### Public Key Address (PKA)

Based on zei's requirements, the recipient address must use the Public Key Address(PKA)
when using private transactions. The bech32 format of the PKA starts with `fra`, which is an ED25519 public key.

#### Transaction

To ensure the atomicity of the transaction, The UTXO-based transaction format is extended to support multiple operations.
Based on the privacy transaction requirement field of zei, an additional `Operation` field is added to both of inputs and outputs
of the transaction. Use `Operation` to mark defining asset, issuing assets or other operations.

Like bitcoin, `txid` is the hash of transaction body. Use the index of the output in transaction and the `txid` to mark the output.
When the input txid byte is all `0`, it means that this `Input`'s' `txid` is the `txid` of the current transaction.
This design is mainly to support aggregating multiple operations.

### Staking && Governance

#### Staking

In native assets, all asset type bytes are 0 are FRA. Staking uses FRA for staking and `Confidential` transactions are not allowed in it.

> Here is detail rules of Staking.

#### Dynamic Rewards Rule based on WASM

Due to tokenomics, the reward rules may need to be adjusted. in order to support this, the reward rules calculate by WASM.
A proposer can propose a transaction for updating rewards rule on the network. More than 2/3 of the validators need to submit a confirmation transaction.
After confirmation passed, the rewards rule will take effect on the specified height.

### EVM && Account Model

EVM is a popular blockchain smart contract platform, but it works on the `Account` model.

#### Simulated Account Model based on UTXO (SAMU)

There is only one UTXO ledger in the network. Due to the execution of EVM requires the Account model, the Account model is simulated using UTXO.
An EVM call transaction will trigger 0 or more sub-transactions. these sub-transactions, and the EVM call transaction transaction will
be packaged as a UTXO transaction.

#### Transfer from Native Token

The length of the Hash Address(HA) is the same as the EVM address, so FRA can directly transfer to an Ethereum address through native UTXO.
At the same time, the 0x format of FRA's HA can be used directly as an Ethereum address in tools such as Metamask.
So you can directly use the HA address as the recipient in the EVM transaction.

#### Web3 Compatibility

By calling the tendermint RPC to compact web3 interface, Web3 can run as a standalone service, or embedded in findorad.
Even embedding findorad is still calling through tendermint RPC without accessing application data.

#### Precompile Contracts

Expect the precompiled contracts defined by the EIP standard. The 20-byte data generated by hashing native asset types will be accessed
as precompiled contract address. Based on different asset properties, it is expressed as IERC20 or IERC721.

## Construction and crates

- libfindora: Common data struct for findora node, client and wallet.
- libfn: Functions for wallet and command line client.
- cli: Provide command line tool called `fn`.
- findorad: Findora core node called `findorad`.
- web3(web3-server): Standalone web3 server.
- modules:
  - asset(fm-asset): Asset & user-define asset management module.
  - utxo(fm-utxo): Core logic for UTXO.
  - coinbase(fm-coinbase): Generate UTXO Outputs based on special rule.
  - fee(fm-fee): Process and verify transaction fee.
  - staking(fm-staking): Staking based on tendermint.
  - rewards(fm-rewards): Reward based on your stake.
  - evm(fm-evm): EVM based on SAMU.

