use kube::config::NamedCluster;

use crate::{kubeconfig, Error};
use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use std::{
    fs::File,
    io::{stdin, BufReader, BufWriter},
    path::Path,
};

pub fn import(
    config_dir: &Path,
    kubeconfig_path: &Path,
    name: Option<&String>,
    use_short: bool,
    proxy: Option<&String>,
) -> Result<String, Error> {
    log::debug!(
        "trying to import {}",
        kubeconfig_path.to_str().unwrap_or_default()
    );

    let mut kubeconfig = match kubeconfig_path.to_str().is_some_and(|x| x == "-") {
        false => kubeconfig::get_from_file(kubeconfig_path)?,
        true => {
            let reader = BufReader::new(stdin().lock());
            match serde_yaml::from_reader(reader) {
                Ok(kubeconfig) => kubeconfig,
                Err(err) => return Err(Error::YAML(err)),
            }
        }
    };

    if let Some(proxy_url) = proxy {
        let clusters = kubeconfig
            .clusters
            .iter()
            .map(|cluster| set_proxy(cluster, proxy_url))
            .collect::<Vec<NamedCluster>>();

        kubeconfig.clusters = clusters;
    }

    // read the name from the command line flag; if it's not set,
    // extract the hostname and use that as name.
    let name: String = match name {
        Some(str) => str.clone(),
        None => {
            log::debug!("no name passed via flag, reading it from kubeconfig server URL");
            let host = kubeconfig::get_hostname(&kubeconfig)?;

            match use_short {
                true => host.split_once('.').unwrap_or((&host, "")).0.to_string(),
                false => host.to_string(),
            }
        }
    };

    log::debug!("using {} as name for kubeconfig file and context", name);

    let target_path = kubeconfig::get_path(config_dir, &name);

    // TODO: prompt the user for confirmation to override instead of
    // throwing an error.
    if target_path.exists() {
        return Err(Error::Message(format!(
            "kubeconfig {} already exists at {}",
            name,
            target_path.display()
        )));
    }

    let kubeconfig = kubeconfig::rename_context(&kubeconfig, &name)?;

    let file = File::create(&target_path)?;
    file.set_permissions(fs::Permissions::from_mode(0o600))?;
    let file = BufWriter::new(file);
    serde_yaml::to_writer(file, &kubeconfig)?;

    Ok(name)
}

fn set_proxy(cluster: &NamedCluster, proxy: &str) -> NamedCluster {
    let mut new_cluster = cluster.to_owned();

    if let Some(inner_cluster) = &cluster.cluster {
        let mut inner_cluster = inner_cluster.to_owned();
        inner_cluster.proxy_url = Some(proxy.to_string());
        new_cluster.cluster = Some(inner_cluster);
    }

    new_cluster
}
