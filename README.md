# Disk Persist

A library that makes it very easy for your application to keep data inbetween executions. It can (very quickly) read and write any data structure that implements [serde](https://serde.rs/)'s `Serialize` and `Deserialize` to disk. It automatically saves the information to either the user's cache folder or any other path that you specify.

## Basics

Create our data:
```rust
use serde::{Deserialize, Serialize};

...

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    age: u8,
}
```

Write it to disk:
```rust
let persist: DiskPersist<Data> = DiskPersist::init("disk-persist-example").unwrap();

let data = Data {
    name: "John Doe".to_string(),
    age: 45,
};

persist.write(&data).unwrap();
```

Then read it at any time:
```rust
let persist: DiskPersist<Data> = DiskPersist::init("disk-persist-example").unwrap();

println!("{:#?}", persist.read().unwrap());
```

Outputs:
```
Some(
    Data {
        name: "John Doe",
        age: 45,
    },
)
```