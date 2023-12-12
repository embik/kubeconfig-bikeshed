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

fn kubeconfig_multiple_cluster_same_host() -> Kubeconfig {
    Kubeconfig {
        api_version: Some("v1".to_string()),
        kind: Some("Config".to_string()),
        preferences: None,
        clusters: vec![
            NamedCluster {
                name: "test-cluster".to_string(),
                cluster: Some(kube::config::Cluster {
                    server: Some("https://test.embik.me:6443/some/suffix".to_string()),
                    insecure_skip_tls_verify: None,
                    certificate_authority: Some("/tmp/ca.crt".to_string()),
                    certificate_authority_data: None,
                    proxy_url: None,
                    tls_server_name: None,
                    extensions: None,
                }),
            },
            NamedCluster {
                name: "test-cluster-2".to_string(),
                cluster: Some(kube::config::Cluster {
                    server: Some("https://test.embik.me:6443".to_string()),
                    insecure_skip_tls_verify: None,
                    certificate_authority: Some("/tmp/ca.crt".to_string()),
                    certificate_authority_data: None,
                    proxy_url: None,
                    tls_server_name: None,
                    extensions: None,
                }),
            },
        ],
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

fn kubeconfig_multiple_cluster_different_hosts() -> Kubeconfig {
    Kubeconfig {
        api_version: Some("v1".to_string()),
        kind: Some("Config".to_string()),
        preferences: None,
        clusters: vec![
            NamedCluster {
                name: "test-cluster".to_string(),
                cluster: Some(kube::config::Cluster {
                    server: Some("https://kubernetes.embik.me:6443/some/suffix".to_string()),
                    insecure_skip_tls_verify: None,
                    certificate_authority: Some("/tmp/ca.crt".to_string()),
                    certificate_authority_data: None,
                    proxy_url: None,
                    tls_server_name: None,
                    extensions: None,
                }),
            },
            NamedCluster {
                name: "test-cluster-2".to_string(),
                cluster: Some(kube::config::Cluster {
                    server: Some("https://test.embik.me:6443".to_string()),
                    insecure_skip_tls_verify: None,
                    certificate_authority: Some("/tmp/ca.crt".to_string()),
                    certificate_authority_data: None,
                    proxy_url: None,
                    tls_server_name: None,
                    extensions: None,
                }),
            },
        ],
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
    assert_eq!("test.embik.me", get_hostname(&kubeconfig)?);

    Ok(())
}

#[test]
fn test_get_hostname_multiple_clusters() -> Result<()> {
    let kubeconfig = kubeconfig_multiple_cluster_same_host();
    assert_eq!("test.embik.me", get_hostname(&kubeconfig)?);

    let kubeconfig = kubeconfig_multiple_cluster_different_hosts();
    assert_eq!(true, get_hostname(&kubeconfig).is_err());

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
