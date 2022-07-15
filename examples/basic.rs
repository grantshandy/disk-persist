use disk_persist::DiskPersist;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    age: u8,
}

fn main() {
    write();
    read();
}

fn write() {
    let persist: DiskPersist<Data> = DiskPersist::init("disk-persist-example").unwrap();

    let data = Data {
        name: "John Doe".to_string(),
        age: 45,
    };

    persist.write(&data).unwrap();
}

fn read() {
    let persist: DiskPersist<Data> = DiskPersist::init("disk-persist-example").unwrap();

    println!("{:#?}", persist.read().unwrap());
}
