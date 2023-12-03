use crate::metadata::ConfigMetadata;
use anyhow::{bail, Result};
use std::{collections::BTreeMap, error::Error};

use clap::ArgMatches;

/// Parse a single key-value pair
pub fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

pub fn collect_from_args(matches: &ArgMatches, id: &str) -> Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();

    matches
        .get_many::<(String, String)>(id)
        .unwrap_or_default()
        .for_each(|(key, value)| {
            map.insert(key.clone(), value.clone());
        });

    Ok(map)
}

pub fn merge_labels(
    metadata: &ConfigMetadata,
    new_labels: &BTreeMap<String, String>,
    overwrite: bool,
) -> Result<BTreeMap<String, String>> {
    match &metadata.labels {
        Some(existing_labels) => {
            let mut merged_labels = existing_labels.clone();

            for (key, value) in new_labels.iter() {
                if let Some(old_value) = merged_labels.insert(key.to_string(), value.to_string()) {
                    if !old_value.eq(value) && !overwrite {
                        bail!(
                            "cannot set key '{}' to value '{}', is '{}' already",
                            key,
                            value,
                            old_value
                        );
                    }
                }
            }

            Ok(merged_labels)
        }
        None => Ok(new_labels.clone()),
    }
}
