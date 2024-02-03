use super::labels;
use crate::Error;
use clap::ArgMatches;
use std::collections::btree_map::BTreeMap;

#[derive(Clone, PartialEq)]
pub enum Operation {
    Equal,
    NotEqual,
}

#[derive(Clone)]
pub struct Selector {
    pub key: String,
    pub value: String,
    pub op: Operation,
}

pub fn matches(selectors: &Vec<Selector>, labels: &BTreeMap<String, String>) -> bool {
    for selector in selectors.iter() {
        let opt = labels.get(&selector.key);
        if let Some(value) = opt {
            if (selector.op == Operation::Equal && &selector.value != value)
                || (selector.op == Operation::NotEqual && &selector.value == value)
            {
                return false;
            }
        } else {
            return false;
        }
    }

    return true;
}

pub fn from_args(matches: &ArgMatches, id: &str) -> Result<Vec<Selector>, Error> {
    if !matches.contains_id(id) {
        return Ok(vec![]);
    }

    let labels = matches
        .get_many::<Selector>(id)
        .ok_or_else(|| Error::Message("failed to parse labels from argument".to_string()))?
        .map(|l| l.to_owned())
        .collect::<Vec<Selector>>();

    Ok(labels)
}

// Parse a single selector from string
pub fn parse(s: &str) -> Result<Selector, Error> {
    let pos = s
        .find('=')
        .ok_or_else(|| Error::Message(format!("invalid selector: no `=` found in `{s}`")))?;
    let mut key_end = pos.clone();
    let mut op = Operation::Equal;

    // check if '!' is in the string and right before the '='
    if let Some(pos_bang) = s.find('!') {
        if pos_bang == pos - 1 {
            op = Operation::NotEqual;
            key_end = pos - 1;
        }
    }

    let key = &s[..key_end];
    let value = &s[pos + 1..];

    if !labels::is_valid_label_key(key) || !labels::is_valid_label_value(value) {
        return Err(Error::Message(
            "key or value are not valid RFC 1123 dns-style".to_string(),
        ));
    }

    Ok(Selector {
        key: key.to_string(),
        value: value.to_string(),
        op,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata;
    use crate::metadata::labels::Label;

    #[test]
    fn test_parse() {
        for selector in &["test=test", "test!=test.com", "test=test.com"] {
            let s = parse(selector);
            assert!(s.is_ok());
            assert_eq!(s.unwrap().key, "test");
        }
    }

    #[test]
    fn test_matches() {
        let tests: Vec<(Vec<Selector>, Vec<Label>, bool)> = vec![
            (
                vec![Selector {
                    key: "key".to_string(),
                    value: "val".to_string(),
                    op: Operation::Equal,
                }],
                vec![],
                false,
            ),
            (
                vec![Selector {
                    key: "key".to_string(),
                    value: "val".to_string(),
                    op: Operation::Equal,
                }],
                vec![Label {
                    key: "key".to_string(),
                    value: Some("val".to_string()),
                }],
                true,
            ),
            (
                vec![Selector {
                    key: "key".to_string(),
                    value: "val".to_string(),
                    op: Operation::NotEqual,
                }],
                vec![Label {
                    key: "key".to_string(),
                    value: Some("val".to_string()),
                }],
                false,
            ),
        ];

        for test in tests.iter() {
            let (selectors, labels, expected) = test;
            let labels_map = metadata::labels::to_map(&labels);
            assert_eq!(matches(&selectors, &labels_map), expected.to_owned());
        }
    }
}
