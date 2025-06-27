//! Tests for Kusto configuration parsing from TOML

use codex_core::config_types::KustoConfig;

#[test]
fn test_kusto_config_from_toml() {
    let toml_content = r#"
        cluster_url = "https://help.kusto.windows.net"
        database = "Samples"
    "#;
    
    let config: KustoConfig = toml::from_str(toml_content).unwrap();
    
    assert_eq!(config.cluster_url, "https://help.kusto.windows.net");
    assert_eq!(config.database, "Samples");
}

#[test]
fn test_minimal_kusto_config() {
    let toml_content = r#"
        cluster_url = "https://help.kusto.windows.net"
        database = "Samples"
    "#;
    
    let config: KustoConfig = toml::from_str(toml_content).unwrap();
    
    assert_eq!(config.cluster_url, "https://help.kusto.windows.net");
    assert_eq!(config.database, "Samples");
}

#[test]
fn test_kusto_config_default() {
    let config = KustoConfig::default();
    
    assert_eq!(config.cluster_url, "");
    assert_eq!(config.database, "");
}