use crate::Error;
use kube::config::Kubeconfig;
use std::{fs::File, path::Path, path::PathBuf};
use url::Url;

mod import;
mod list;

pub use import::import;
pub use list::list;

#[cfg(test)]
mod tests;

pub fn get_from_file(file: &Path) -> Result<Kubeconfig, Error> {
    let kubeconfig_file = File::open(file)?;

    let kubeconfig = match serde_yaml::from_reader::<File, Kubeconfig>(kubeconfig_file) {
        Ok(kubeconfig) => kubeconfig,
        Err(err) => return Err(Error::YAML(err)),
    };

    Ok(kubeconfig)
}

pub fn get(config_dir: &Path, name: &str) -> Result<(PathBuf, Kubeconfig), Error> {
    let kubeconfig_path = config_dir.join(format!("{name}.kubeconfig"));
    let kubeconfig = get_from_file(&kubeconfig_path)?;
    Ok((kubeconfig_path, kubeconfig))
}

pub fn get_hostname(kubeconfig: &Kubeconfig) -> Result<String, Error> {
    let mut urls: Vec<String> = vec![];
    for cluster in kubeconfig.clusters.iter() {
        let url = cluster
            .cluster
            .as_ref()
            .ok_or(Error::Message(
                "could not find cluster field in kubeconfig".to_string(),
            ))?
            .server
            .as_ref()
            .ok_or(Error::Message(
                "could not find server field in kubeconfig".to_string(),
            ))?
            .to_string();
        let url = Url::parse(&url)?;
        let host = url
            .host_str()
            .ok_or_else(|| Error::Message("failed to parse host from server URL".to_string()))?;
        urls.push(host.to_string());
    }

    urls.dedup();

    match urls.len() {
        0 => Err(Error::Message(
            "could not find any server URL in kubeconfig".to_string(),
        )),
        1 => urls.first().ok_or(Error::Message("".to_string())).cloned(),
        _ => Err(Error::Message(
            "kubeconfig has more than one server defined".to_string(),
        )),
    }
}

pub fn rename_context(kubeconfig: &Kubeconfig, context_name: &str) -> Result<Kubeconfig, Error> {
    let mut new_kubeconfig = kubeconfig.clone();

    let current_context = kubeconfig
        .current_context
        .as_ref()
        .ok_or_else(|| Error::Message("cannot get current context".to_string()))?;

    let mut contexts = kubeconfig.contexts.clone();
    for context in &mut contexts {
        if context.name.eq(current_context) {
            context.name = context_name.to_string();
        }
    }

    new_kubeconfig.current_context = Some(context_name.to_string());
    new_kubeconfig.contexts = contexts;

    Ok(new_kubeconfig)
}
