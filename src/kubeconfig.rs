use crate::errors::ImportError;
use anyhow::{anyhow, bail, Result};
use kube::config::Kubeconfig;
use std::{fs::File, path::Path};

pub fn get(file: &Path) -> Result<Kubeconfig> {
    let kubeconfig_file = File::open(file)?;

    let kubeconfig = match serde_yaml::from_reader::<File, Kubeconfig>(kubeconfig_file) {
        Ok(kubeconfig) => kubeconfig,
        Err(err) => return Err(anyhow!(err)),
    };

    if !is_valid(&kubeconfig) {
        bail!(ImportError::InvalidConfiguration);
    }

    Ok(kubeconfig)
}

fn is_valid(kubeconfig: &Kubeconfig) -> bool {
    kubeconfig.auth_infos.len() == 1 && kubeconfig.clusters.len() == 1
}

pub fn get_hostname(kubeconfig: &Kubeconfig) -> Result<String, ImportError> {
    let named_cluster = kubeconfig
        .clusters
        .first()
        .ok_or(ImportError::InvalidCluster(
            "could not get cluster".to_string(),
        ))?;

    let cluster = named_cluster
        .clone()
        .cluster
        .ok_or(ImportError::InvalidCluster(
            "could not get cluster".to_string(),
        ))?;

    let server = cluster.server.ok_or(ImportError::InvalidCluster(
        "could not get server from cluster".to_string(),
    ))?;

    Ok(server)
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
