use anyhow::{anyhow, bail, Result};
use kube::config::Kubeconfig;
use std::{fs::File, path::Path};
use url::Url;

#[cfg(test)]
mod tests;

pub fn get(file: &Path) -> Result<Kubeconfig> {
    let kubeconfig_file = File::open(file)?;

    let kubeconfig = match serde_yaml::from_reader::<File, Kubeconfig>(kubeconfig_file) {
        Ok(kubeconfig) => kubeconfig,
        Err(err) => bail!(err),
    };

    Ok(kubeconfig)
}

pub fn get_hostname(kubeconfig: &Kubeconfig) -> Result<String> {
    let mut urls: Vec<String> = vec![];
    for cluster in kubeconfig.clusters.iter() {
        let url = cluster
            .cluster
            .as_ref()
            .ok_or(anyhow!("could not find cluster field in kubeconfig"))?
            .server
            .as_ref()
            .ok_or(anyhow!("could not find server field in kubeconfig"))?
            .to_string();
        let url = Url::parse(&url)?;
        let host = url
            .host_str()
            .ok_or_else(|| anyhow!("failed to parse host from server URL"))?;
        urls.push(host.to_string());
    }

    urls.dedup();

    match urls.len() {
        0 => Err(anyhow!("could not find any server URL in kubeconfig")),
        1 => urls.first().ok_or(anyhow!("")).cloned(),
        _ => Err(anyhow!("kubeconfig has more than one server defined")),
    }
}

pub fn rename_context(kubeconfig: &Kubeconfig, context_name: &str) -> Result<Kubeconfig> {
    let mut new_kubeconfig = kubeconfig.clone();

    let current_context = kubeconfig
        .current_context
        .as_ref()
        .ok_or_else(|| anyhow!("cannot get current context"))?;

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
