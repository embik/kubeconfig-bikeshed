use kube::config::Kubeconfig;
use serde_yaml;

use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

use crate::import::error;

pub fn get(file: &OsString) -> Result<Kubeconfig, Box<dyn Error>> {
    let kubeconfig_file = File::open(file)?;

    let kubeconfig = match serde_yaml::from_reader::<File, Kubeconfig>(kubeconfig_file) {
        Ok(kubeconfig) => kubeconfig,
        Err(err) => return Err(Box::new(err)),
    };

    if !is_valid(&kubeconfig) {
        return Err(Box::new(error::ImportError::InvalidConfiguration));
    }

    Ok(kubeconfig)
}

fn is_valid(kubeconfig: &Kubeconfig) -> bool {
    kubeconfig.auth_infos.len() == 1 && kubeconfig.clusters.len() == 1
}
