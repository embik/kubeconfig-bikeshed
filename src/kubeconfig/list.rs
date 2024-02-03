use crate::metadata::{self, Metadata};
use crate::Error;
use std::collections::btree_map::BTreeMap;
use std::{fs, path::Path};

pub struct ListEntry {
    pub name: String,
    pub labels: Option<BTreeMap<String, String>>,
}

pub fn list(
    config_dir: &Path,
    metadata: &Metadata,
    selectors: Option<Vec<metadata::Selector>>,
) -> Result<Vec<ListEntry>, Error> {
    let mut kubeconfigs: Vec<ListEntry> = vec![];

    let mut files: Vec<fs::DirEntry> = fs::read_dir(config_dir)?.map(|r| r.unwrap()).collect();
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

        let mut labels: Option<BTreeMap<String, String>> = None;

        if let Some(ref selectors) = selectors {
            labels = match metadata.get(name) {
                Some(m) => {
                    if !metadata::selectors::matches(
                        selectors,
                        &m.labels.clone().unwrap_or_default(),
                    ) {
                        continue;
                    }

                    Some(m.labels.clone().unwrap_or_default())
                }
                None => None,
            };
        }

        kubeconfigs.push(ListEntry {
            name: name.to_string(),
            labels,
        });
    }

    Ok(kubeconfigs)
}

fn is_kubeconfig(file: &Path) -> bool {
    if !file.is_file() {
        return false;
    }

    matches!(file.extension(), Some(extension) if extension == "kubeconfig")
}
