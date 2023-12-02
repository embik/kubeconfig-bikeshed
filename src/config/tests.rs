use super::*;
use anyhow::Result;

#[test]
fn test_get_config_dir_with_xdg_config_home() -> Result<()> {
    env::set_var("XDG_CONFIG_HOME", "/xdg/config/home");
    assert_eq!("/xdg/config/home/kbs", get_config_dir()?.to_str().unwrap());
    Ok(())
}
