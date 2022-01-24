fn main() {
    let db: sled::Db = sled::open("./target/utxo").unwrap();
    let tree: sled::Tree = db.open_tree("owned_outputs").unwrap();

    for it in tree.iter() {
        let (k, v) = it.unwrap();
        println!("key: {:?}, valve: {:?}", k, v);
    }

}
