use crate::metadata::ConfigMetadata;
use crate::Error;
use clap::ArgMatches;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Label {
    pub key: String,
    pub value: Option<String>,
}

pub fn to_map(vec: &Vec<Label>) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    vec.iter().for_each(|label| {
        if let Some(value) = &label.value {
            map.insert(label.key.clone(), value.clone());
        }
    });

    map
}

/// Parse a single label (key=value or key-) from string
pub fn parse(s: &str) -> Result<Label, Error> {
    // if the equal sign is in the string, we can parse key=val
    if let Some(pos) = s.find('=') {
        let key = &s[..pos];
        let value = &s[pos + 1..];

        if !is_valid_label_key(key) || !is_valid_label_value(value) {
            return Err(Error::Message(
                "key or value are not valid RFC 1123 dns-style".to_string(),
            ));
        }

        return Ok(Label {
            key: key.to_string(),
            value: Some(value.to_string()),
        });
    } else if s.ends_with('-') {
        // if the equal sign is NOT in here, we have to look for '-' because this might be
        // setting the "remove label with this key" type of label string
        let key = &s[..s.len() - 1];

        return Ok(Label {
            key: key.to_string(),
            value: None,
        });
    }

    Err(Error::Message(format!("could not parse `{s}` as label")))
}

pub fn from_args(matches: &ArgMatches, id: &str) -> Result<Vec<Label>, Error> {
    if !matches.contains_id(id) {
        return Ok(vec![]);
    }

    let labels = matches
        .get_many::<Label>(id)
        .ok_or_else(|| Error::Message("failed to parse labels from argument".to_string()))?
        .map(|l| l.to_owned())
        .collect::<Vec<Label>>();

    Ok(labels)
}

pub fn merge(
    metadata: &ConfigMetadata,
    new_labels: &Vec<Label>,
    overwrite: bool,
) -> Result<BTreeMap<String, String>, Error> {
    match &metadata.labels {
        Some(existing_labels) => {
            let mut merged_labels = existing_labels.clone();

            for label in new_labels.iter() {
                if let Some(new_val) = &label.value {
                    if let Some(old_val) =
                        merged_labels.insert(label.key.clone(), new_val.to_owned())
                    {
                        if !old_val.eq(new_val) && !overwrite {
                            return Err(Error::Message(format!(
                                "cannot set key '{}' to value '{}', is '{}' already",
                                label.key, new_val, old_val
                            )));
                        }
                    }
                } else {
                    // the label had value set to None, which means
                    // we want to remove the label from the merged
                    // map.
                    merged_labels.remove(&label.key);
                }
            }

            Ok(merged_labels)
        }
        None => Ok(to_map(&new_labels)),
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

pub fn is_valid_label_key(label: &str) -> bool {
    let (prefix, name) = label.split_at(label.find('/').unwrap_or_else(|| 0) + 1);
    let prefix = prefix.strip_suffix("/").unwrap_or_else(|| prefix);

    is_valid_rfc_1123_subdomain(prefix) && is_valid_rfc_1123_subdomain(name)
}

pub fn is_valid_label_value(value: &str) -> bool {
    value.len() < 64
        && value.chars().all(|b| {
            (b.is_alphabetic() && b.is_lowercase())
                || b.is_numeric()
                || b == '.'
                || b == '-'
                || b == '_'
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rfc_1123_subdomains() {
        for name in &["test", "test.com", "test-domain"] {
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
        for key in &["test", "test.com", "test.com/key", "test-key"] {
            assert!(is_valid_label_key(key), "{key} is not valid");
        }
    }

    #[test]
    fn test_invalid_label_keys() {
        for key in &["tesT", "test@com", "test+com/key", "1234?"] {
            assert!(!is_valid_label_key(key), "{key} should not be valid");
        }
    }

    #[test]
    fn test_valid_label_values() {
        for value in &["test", "test-value", "test_value"] {
            assert!(is_valid_label_value(value), "{value} is not valid");
        }
    }

    #[test]
    fn test_invalid_label_values() {
        for value in &["tesT", "test$value", "test_Value", "test/value"] {
            assert!(!is_valid_label_value(value), "{value} should not be valid");
        }
    }
}
