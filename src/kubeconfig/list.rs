use crate::metadata::{self, Metadata};
use crate::Error;
use std::collections::btree_map::BTreeMap;
use std::{fs, path::Path};

pub fn list(
    config_dir: &Path,
    metadata: &Metadata,
    selectors: Option<Vec<&(String, String)>>,
) -> Result<Vec<(String, BTreeMap<String, String>)>, Error> {
    let mut kubeconfigs: Vec<(String, BTreeMap<String, String>)> = vec![];

    let mut files: Vec<_> = fs::read_dir(config_dir)?.map(|r| r.unwrap()).collect();
    files.sort_by_key(|f| f.path());

    for file in files {
        let file = file.path();

        if !is_kubeconfig(&file) {
            continue;
        }

        let name = file
            .file_stem()
            .ok_or_else(|| Error::Message("cannot determine basename".to_string()))?
            .to_str()
            .ok_or_else(|| Error::Message("cannot convert file path to string".to_string()))?;

        let labels = match metadata.get(name) {
            Some(m) => m.labels.clone().unwrap_or_default(),
            None => BTreeMap::new(),
        };

        if !metadata::labels::matches_labels(&labels, &selectors) {
            continue;
        }

        kubeconfigs.push((name.to_string(), labels));
    }

    Ok(kubeconfigs)
}

fn is_kubeconfig(file: &Path) -> bool {
    if !file.is_file() {
        return false;
    }

    matches!(file.extension(), Some(extension) if extension == "kubeconfig")
}
