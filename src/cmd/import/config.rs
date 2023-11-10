use kube::config::Kubeconfig;
use serde_yaml;

use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

pub fn get_kubeconfig(file: &OsString) -> Result<Kubeconfig, Box<dyn Error>> {
    let kubeconfig_file = File::open(file)?;

    serde_yaml::from_reader::<File, Kubeconfig>(kubeconfig_file)
        .map_err(|err| Box::new(err) as Box<dyn Error>)
}
