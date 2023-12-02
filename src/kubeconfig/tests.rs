use super::*;
use anyhow::Result;
use kube::config::{NamedAuthInfo, NamedCluster, NamedContext};

fn kubeconfig() -> Kubeconfig {
    Kubeconfig {
        api_version: Some("v1".to_string()),
        kind: Some("Config".to_string()),
        preferences: None,
        clusters: vec![NamedCluster {
            name: "test-cluster".to_string(),
            cluster: Some(kube::config::Cluster {
                server: Some("https://test.embik.me:6443".to_string()),
                insecure_skip_tls_verify: None,
                certificate_authority: Some("/tmp/ca.crt".to_string()),
                certificate_authority_data: None,
                proxy_url: None,
                tls_server_name: None,
                extensions: None,
            }),
        }],
        contexts: vec![NamedContext {
            name: "current-context".to_string(),
            context: Some(kube::config::Context {
                cluster: "test-cluster".to_string(),
                user: "user".to_string(),
                namespace: Some("default".to_string()),
                extensions: None,
            }),
        }],
        auth_infos: vec![NamedAuthInfo {
            name: "user".to_string(),
            auth_info: None,
        }],
        extensions: None,
        current_context: Some("current-context".to_string()),
    }
}

#[test]
fn test_get_hostname_single_cluster() -> Result<()> {
    let kubeconfig = kubeconfig();
    assert_eq!("https://test.embik.me:6443", get_hostname(&kubeconfig)?);

    Ok(())
}

#[test]
fn test_rename_context() -> Result<()> {
    let kubeconfig = kubeconfig();

    assert_eq!(
        "test.embik.me",
        rename_context(&kubeconfig, "test.embik.me")
            .unwrap()
            .current_context
            .unwrap()
    );

    Ok(())
}
