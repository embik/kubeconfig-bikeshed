use crate::{error, Error};
use serde::{Deserialize, Serialize};
use std::collections::btree_map::BTreeMap;
use std::{fs::File, path::Path, path::PathBuf};

pub mod labels;
pub mod selectors;

pub use selectors::Selector;

pub const FILE: &str = "metadata.json";

const VERSION: &str = "0.1";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub version: String,
    pub kubeconfigs: BTreeMap<String, ConfigMetadata>,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            version: VERSION.to_string(),
            kubeconfigs: BTreeMap::new(),
        }
    }

    pub fn from_file(file: &Path) -> Result<Metadata, Error> {
        let metadata_file = match File::open(file) {
            Ok(file) => file,
            Err(err) => return Err(Error::IO(err)),
        };

        let metadata = match serde_json::from_reader::<File, Metadata>(metadata_file) {
            Ok(metadata) => metadata,
            Err(err) => return Err(Error::JSON(err)),
        };

        if metadata.version != VERSION {
            return Err(Error::Message(format!(
                "unknown metadata version: {}",
                metadata.version
            )));
        }

        Ok(metadata)
    }

    pub fn write(&self, file: &Path) -> Result<(), Error> {
        let metadata_file = match File::create(file) {
            Ok(file) => file,
            Err(_) => std::fs::OpenOptions::new().write(true).open(file)?,
        };

        match serde_json::to_writer::<File, Metadata>(metadata_file, self) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::JSON(err)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&ConfigMetadata> {
        let map = &self.kubeconfigs;
        map.get(name)
    }

    pub fn set(mut self, name: String, metadata: ConfigMetadata) -> Self {
        let map = &mut self.kubeconfigs;
        map.insert(name, metadata);
        self
    }

    pub fn remove(mut self, name: &str) -> Self {
        let map = &mut self.kubeconfigs;
        map.remove(name);
        self
    }

    pub fn rename(mut self, source: &str, destination: &str) -> Result<Self, error::Error> {
        let map = &mut self.kubeconfigs;
        if let Some(val) = map.get(source) {
            if map.get(destination).is_some() {
                return Err(error::Error::Message(format!(
                    "{destination} already exists in metadata store"
                )));
            }

            map.insert(destination.to_string(), val.clone());
            map.remove(source);
        }

        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigMetadata {
    pub labels: Option<BTreeMap<String, String>>,
}

pub fn file_path(config_dir: &Path) -> PathBuf {
    config_dir.join(FILE)
}
