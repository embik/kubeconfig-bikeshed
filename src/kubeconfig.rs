use kube::config::Kubeconfig;

use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::errors::ImportError;

pub fn get(file: &Path) -> Result<Kubeconfig, Box<dyn Error>> {
    let kubeconfig_file = File::open(file)?;

    let kubeconfig = match serde_yaml::from_reader::<File, Kubeconfig>(kubeconfig_file) {
        Ok(kubeconfig) => kubeconfig,
        Err(err) => return Err(Box::new(err)),
    };

    if !is_valid(&kubeconfig) {
        return Err(Box::new(ImportError::InvalidConfiguration));
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
