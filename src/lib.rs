#![doc = include_str!("../README.md")]

use std::{
    fs,
    io::{self, ErrorKind},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

const DEFAULT_FILE_TYPE: &'static str = "persist";

/// A struct for managing persistent data on disk
pub struct DiskPersist<D> {
    path: PathBuf,
    _type: PhantomData<D>,
}

impl<D: Serialize + DeserializeOwned> DiskPersist<D> {
    /// Initialize a new [`DiskPersist`].
    pub fn init(name: impl AsRef<str>) -> Result<Self, DataError> {
        let mut path: PathBuf = match dirs::cache_dir() {
            Some(base) => base,
            None => return Err(DataError::NoCacheDir),
        };

        path.push(&format!("{}.{}", name.as_ref(), DEFAULT_FILE_TYPE));

        Ok(Self {
            _type: PhantomData::default(),
            path,
        })
    }

    /// Initialize a new [`DiskPersist`] with a custom path.
    pub fn init_with_path(path: impl AsRef<Path>) -> Result<Self, DataError> {
        let path = path.as_ref().to_path_buf();

        if path.is_dir() {
            return Err(DataError::FoundDirectory);
        }

        Ok(Self {
            _type: PhantomData::default(),
            path,
        })
    }

    /// Update data to disk.
    pub fn write(&self, data: &D) -> Result<(), DataError> {
        let bytes = match bincode::serialize(data) {
            Ok(bytes) => bytes,
            Err(error) => return Err(DataError::Serde(error)),
        };

        match fs::write(&self.path, bytes) {
            Ok(_) => Ok(()),
            Err(error) => return Err(DataError::Io(error)),
        }
    }

    /// Read data from disk.
    pub fn read(&self) -> Result<Option<D>, DataError> {
        let bytes = match fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(error) => {
                return match error.kind() {
                    ErrorKind::NotFound => return Ok(None),
                    _ => Err(DataError::Io(error)),
                }
            }
        };

        let deserialized: D = match bincode::deserialize(&bytes) {
            Ok(deserialized) => deserialized,
            Err(error) => return Err(DataError::Serde(error)),
        };

        Ok(Some(deserialized))
    }

    /// Return a reference to the internal path where data is written.
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

/// Errors returned by [`DiskPersist`].
#[derive(Error, Debug)]
pub enum DataError {
    /// Error reading or writing to your file.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Couldn't serialize or deserialize your data, this could mean that the data is corrupted.
    #[error(transparent)]
    Serde(#[from] bincode::Error),
    /// Couldn't find a cache directory on the system.
    #[error("couldn't find cache directory")]
    NoCacheDir,
    /// The path optional save path you used is a directory, and not a file.
    #[error("optional save path must be a file, not a directory")]
    FoundDirectory,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde::{Deserialize, Serialize};

    use crate::DiskPersist;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Data {
        name: String,
        age: u8,
        location: (f64, f64),
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                name: "Jane Doe".to_string(),
                age: 45,
                location: (49.24565431256531, 111.35598566896671),
            }
        }
    }

    #[test]
    fn full_test() {
        let name = "disk-persist-test";

        // write to disk
        let write: DiskPersist<Data> = DiskPersist::init(name).unwrap();
        let write_data = Data::default();
        write.write(&write_data).unwrap();

        // read from disk
        let read: DiskPersist<Data> = DiskPersist::init(name).unwrap();
        let read_data = read.read().unwrap().unwrap();

        // compare what we got (in rust structs)
        assert_eq!(write_data, read_data);

        // clean up after ourselves
        fs::remove_file(write.path()).unwrap();
    }
}
