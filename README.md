# Disk Persist

[![CI](https://github.com/grantshandy/disk-persist/actions/workflows/rust.yml/badge.svg)](https://github.com/grantshandy/disk-persist/actions)
[![](https://img.shields.io/crates/v/disk-persist.svg)](https://crates.io/crates/disk-persist)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![](https://img.shields.io/docsrs/disk-persist)](https://docs.rs/disk-persist)

A library that makes it very easy for your application to keep data inbetween executions. It can (very quickly) read and write any data structure that implements [serde](https://serde.rs/)'s `Serialize` and `Deserialize` to disk. It automatically saves the information to either the user's cache folder or any other path that you specify.

Default Location:

|Platform | Value                               | Example                      |
| ------- | ----------------------------------- | ---------------------------- |
| Linux   | `$XDG_CACHE_HOME` or `$HOME`/.cache | /home/user/.cache           |
| macOS   | `$HOME`/Library/Caches              | /Users/User/Library/Caches  |
| Windows | `{FOLDERID_LocalAppData}`           | C:\Users\User\AppData\Local |

## The Basics

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