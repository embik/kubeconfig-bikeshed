use crate::kubeconfig;
use crate::kubeconfig::Error;
use crate::metadata::Metadata;
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
    name: Option<String>,
    use_short: bool,
) -> Result<(String, Metadata), Error> {
    let kubeconfig = match kubeconfig_path.to_str().is_some_and(|x| x == "-") {
        false => kubeconfig::get(kubeconfig_path)?,
        true => {
            let reader = BufReader::new(stdin().lock());
            match serde_yaml::from_reader(reader) {
                Ok(kubeconfig) => kubeconfig,
                Err(err) => return Err(Error::YAML(err)),
            }
        }
    };

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

    let target_path = config_dir.join(format!("{}.kubeconfig", name));

    // TODO: prompt the user for confirmation to override instead of
    // throwing an error.
    if target_path.exists() {
        bail!(
            "kubeconfig {} already exists at {}",
            name,
            target_path.display()
        );
    }

    let kubeconfig = kubeconfig::rename_context(&kubeconfig, &name)?;

    let file = File::create(&target_path)?;
    file.set_permissions(fs::Permissions::from_mode(0o600))?;
    let file = BufWriter::new(file);
    serde_yaml::to_writer(file, &kubeconfig)?;

    Ok("")
}
