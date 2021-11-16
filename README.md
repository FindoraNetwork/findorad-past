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

## Construction and crates.

- libfindora: Common data struct for findora node, client and wallet.
- libfn: Functions for wallet and command line client.
- cli: Provide command line tool called `fn`.
- findorad: Findora core node called `findorad`.
- modules:
  - utxo: Core logic for UTXO.
  - coinbase: Generate UTXO Outputs based on special rule.
