use crate::metadata::ConfigMetadata;
use anyhow::{anyhow, bail, Result};
use clap::ArgMatches;
use std::collections::BTreeMap;

/// Parse a single key-value pair
pub fn parse_key_val(s: &str) -> Result<(String, String)> {
    let pos = s
        .find('=')
        .ok_or_else(|| anyhow!("invalid key=value pair: no `=` found in `{s}`"))?;

    let key = &s[..pos];
    let value = &s[pos + 1..];

    if !is_valid_label_key(key) || !is_valid_rfc_1123_label(value) {
        return Err(anyhow!(
            "key or value are not valid RFC 1123 dns-style names"
        ));
    }

    Ok((key.to_string(), value.to_string()))
}

pub fn collect_from_args(matches: &ArgMatches, id: &str) -> Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();

    matches
        .get_many::<(String, String)>(id)
        .ok_or_else(|| anyhow!("failed to parse labels from argument"))?
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

// Ensure that a given label key or value is compliant with RFC 1123
// specifications for DNS subdomains.
//
// Validity is given when the given string is:
// - maximum 253 characters
// - only lowercase alphanumeric characters, '-' or '.'
// - starting and ending with an alphanumeric character
pub fn is_valid_rfc_1123_subdomain(label: &str) -> bool {
    label.len() < 254
        && label.chars().all(|b| {
            (b.is_alphabetic() && b.is_lowercase()) || b.is_numeric() || b == '.' || b == '-'
        })
}

// Ensure that a given label key or value is compliant with RFC 1123
// specifications for DNS labels.
//
// Validity is given when the given string is:
// - maximum 63 characters
// - only lowercase alphanumeric characters
// - starting and ending with an alphanumeric character
pub fn is_valid_rfc_1123_label(label: &str) -> bool {
    label.len() < 64
        && label
            .chars()
            .all(|b| ((b.is_alphabetic() && b.is_lowercase()) || b.is_numeric()))
}

pub fn is_valid_label_key(label: &str) -> bool {
    let (prefix, name) = label.split_at(label.find('/').unwrap_or_else(|| 0) + 1);
    let prefix = prefix.strip_suffix("/").unwrap_or_else(|| prefix);

    is_valid_rfc_1123_subdomain(prefix) && is_valid_rfc_1123_subdomain(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rfc_1123_subdomains() {
        for name in &["test", "test.com"] {
            assert!(is_valid_rfc_1123_subdomain(name), "{name} is not valid");
        }
    }

    #[test]
    fn test_invalid_rfc_1123_subdomains() {
        for name in &["teSt", "tEst.Com"] {
            assert!(
                !is_valid_rfc_1123_subdomain(name),
                "{} should not valid",
                name
            );
        }
    }

    #[test]
    fn test_valid_label_keys() {
        for key in &["test", "test.com", "test.com/key"] {
            assert!(is_valid_label_key(key), "{key} is not valid");
        }
    }

    #[test]
    fn test_invalid_label_keys() {
        for key in &["tesT", "test@com", "test+com/key", "1234?"] {
            assert!(!is_valid_label_key(key), "{key} should not be valid");
        }
    }
}
