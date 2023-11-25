use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::btree_map::BTreeMap;
use std::{fs::File, path::Path};

pub mod labels;

pub const FILE: &str = "metadata.json";

const VERSION: &str = "0.1";

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn from_file(file: &Path) -> Result<Metadata> {
        let metadata_file = File::open(file)?;

        let metadata = match serde_json::from_reader::<File, Metadata>(metadata_file) {
            Ok(metadata) => metadata,
            Err(err) => return Err(anyhow!(err)),
        };

        if metadata.version != VERSION {
            return Err(anyhow!("unknown metadata version detected"));
        }

        Ok(metadata)
    }

    pub fn write(&self, file: &Path) -> Result<()> {
        let metadata_file = match File::create(file) {
            Ok(file) => file,
            Err(_) => std::fs::OpenOptions::new().write(true).open(file)?,
        };

        match serde_json::to_writer::<File, Metadata>(metadata_file, self) {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow!(err)),
        }
    }

    pub fn get(&self, name: String) -> Option<&ConfigMetadata> {
        let map = &self.kubeconfigs;
        map.get(&name)
    }

    pub fn set(mut self, name: String, metadata: ConfigMetadata) -> Self {
        let map = &mut self.kubeconfigs;
        map.insert(name, metadata);
        self
    }

    pub fn remove(mut self, name: &String) -> Self {
        let map = &mut self.kubeconfigs;
        map.remove(name);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMetadata {
    pub labels: Option<BTreeMap<String, String>>,
}