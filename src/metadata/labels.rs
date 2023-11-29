use anyhow::{anyhow, Result};
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
        .ok_or_else(|| anyhow!("failed to parse labels"))?
        .for_each(|(key, value)| {
            map.insert(key.clone(), value.clone());
        });

    Ok(map)
}
