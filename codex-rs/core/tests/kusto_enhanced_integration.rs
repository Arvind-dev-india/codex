//! Integration tests for enhanced Kusto functionality

use codex_core::config_types::{KustoConfig, KustoDatabaseConfig};
use codex_core::kusto::{KustoKnowledgeBase, ColumnInfo};
use std::collections::HashMap;
use tempfile::TempDir;

#[tokio::test]
async fn test_kusto_config_multiple_databases() {
    let mut config = KustoConfig::default();
    config.cluster_url = "https://help.kusto.windows.net".to_string();
    config.database = "Samples".to_string();
    
    // Add multiple databases
    let mut databases = HashMap::new();
    databases.insert("samples".to_string(), KustoDatabaseConfig {
        name: "Samples".to_string(),
        description: Some("Sample database".to_string()),
        cluster_url: None,
        is_default: true,
    });
    
    databases.insert("production".to_string(), KustoDatabaseConfig {
        name: "ProductionDB".to_string(),
        description: Some("Production database".to_string()),
        cluster_url: Some("https://prod.kusto.windows.net".to_string()),
        is_default: false,
    });
    
    config.databases = databases;
    
    // Verify configuration
    assert_eq!(config.databases.len(), 2);
    assert!(config.databases.contains_key("samples"));
    assert!(config.databases.contains_key("production"));
    
    let samples_db = &config.databases["samples"];
    assert_eq!(samples_db.name, "Samples");
    assert!(samples_db.is_default);
    
    let prod_db = &config.databases["production"];
    assert_eq!(prod_db.name, "ProductionDB");
    assert!(!prod_db.is_default);
    assert_eq!(prod_db.cluster_url.as_ref().unwrap(), "https://prod.kusto.windows.net");
}

#[tokio::test]
async fn test_knowledge_base_creation_and_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let kb_path = temp_dir.path().join("test_kb.json");
    
    // Create a new knowledge base
    let mut kb = KustoKnowledgeBase::new();
    
    // Add database information
    kb.update_database(
        "TestDB".to_string(),
        "https://test.kusto.windows.net".to_string(),
        Some("Test database".to_string())
    );
    
    // Add table information
    let columns = vec![
        ColumnInfo {
            name: "Id".to_string(),
            data_type: "long".to_string(),
            description: Some("Primary key".to_string()),
            is_nullable: Some(false),
            sample_values: vec![serde_json::Value::Number(serde_json::Number::from(1))],
            is_commonly_queried: true,
        },
        ColumnInfo {
            name: "Name".to_string(),
            data_type: "string".to_string(),
            description: Some("Entity name".to_string()),
            is_nullable: Some(true),
            sample_values: vec![serde_json::Value::String("Test".to_string())],
            is_commonly_queried: true,
        },
    ];
    
    let sample_data = vec![{
        let mut row = HashMap::new();
        row.insert("Id".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        row.insert("Name".to_string(), serde_json::Value::String("Test Entity".to_string()));
        row
    }];
    
    kb.update_table("TestDB", "TestTable".to_string(), columns, sample_data);
    
    // Save to file
    kb.save_to_file(&kb_path).await.unwrap();
    
    // Load from file
    let loaded_kb = KustoKnowledgeBase::load_from_file(&kb_path).await.unwrap();
    
    // Verify loaded data
    assert_eq!(loaded_kb.databases.len(), 1);
    assert!(loaded_kb.databases.contains_key("TestDB"));
    
    let db_info = &loaded_kb.databases["TestDB"];
    assert_eq!(db_info.name, "TestDB");
    assert_eq!(db_info.cluster_url, "https://test.kusto.windows.net");
    assert_eq!(db_info.tables.len(), 1);
    
    let table_info = &db_info.tables["TestTable"];
    assert_eq!(table_info.name, "TestTable");
    assert_eq!(table_info.columns.len(), 2);
    assert_eq!(table_info.sample_data.len(), 1);
    
    // Verify columns
    let id_column = &table_info.columns[0];
    assert_eq!(id_column.name, "Id");
    assert_eq!(id_column.data_type, "long");
    assert!(id_column.is_commonly_queried);
    
    let name_column = &table_info.columns[1];
    assert_eq!(name_column.name, "Name");
    assert_eq!(name_column.data_type, "string");
}

#[tokio::test]
async fn test_knowledge_base_search() {
    let mut kb = KustoKnowledgeBase::new();
    
    // Add test data
    kb.update_database(
        "TestDB".to_string(),
        "https://test.kusto.windows.net".to_string(),
        Some("Test database with user data".to_string())
    );
    
    let user_columns = vec![
        ColumnInfo {
            name: "UserId".to_string(),
            data_type: "long".to_string(),
            description: Some("User identifier".to_string()),
            is_nullable: Some(false),
            sample_values: vec![serde_json::Value::Number(serde_json::Number::from(123))],
            is_commonly_queried: true,
        },
        ColumnInfo {
            name: "UserName".to_string(),
            data_type: "string".to_string(),
            description: Some("User display name".to_string()),
            is_nullable: Some(true),
            sample_values: vec![serde_json::Value::String("john_doe".to_string())],
            is_commonly_queried: true,
        },
    ];
    
    kb.update_table("TestDB", "Users".to_string(), user_columns, vec![]);
    
    let event_columns = vec![
        ColumnInfo {
            name: "EventId".to_string(),
            data_type: "long".to_string(),
            description: Some("Event identifier".to_string()),
            is_nullable: Some(false),
            sample_values: vec![serde_json::Value::Number(serde_json::Number::from(456))],
            is_commonly_queried: false,
        },
    ];
    
    kb.update_table("TestDB", "Events".to_string(), event_columns, vec![]);
    
    // Test table search
    let mut table_matches = Vec::new();
    for (db_name, db_info) in &kb.databases {
        for (table_name, table_info) in &db_info.tables {
            if table_name.to_lowercase().contains("user") ||
               table_info.description.as_ref().map_or(false, |d| d.to_lowercase().contains("user")) {
                table_matches.push((db_name, table_name));
            }
        }
    }
    
    assert_eq!(table_matches.len(), 1);
    assert_eq!(table_matches[0], (&"TestDB".to_string(), &"Users".to_string()));
    
    // Test column search
    let mut column_matches = Vec::new();
    for (db_name, db_info) in &kb.databases {
        for (table_name, table_info) in &db_info.tables {
            for column in &table_info.columns {
                if column.name.to_lowercase().contains("user") ||
                   column.description.as_ref().map_or(false, |d| d.to_lowercase().contains("user")) {
                    column_matches.push((db_name, table_name, &column.name));
                }
            }
        }
    }
    
    assert_eq!(column_matches.len(), 2);
    assert!(column_matches.contains(&(&"TestDB".to_string(), &"Users".to_string(), &"UserId".to_string())));
    assert!(column_matches.contains(&(&"TestDB".to_string(), &"Users".to_string(), &"UserName".to_string())));
}

#[test]
fn test_kusto_config_defaults() {
    let config = KustoConfig::default();
    
    assert_eq!(config.cluster_url, "");
    assert_eq!(config.database, "");
    assert_eq!(config.knowledge_base_path, "kusto_knowledge_base.json");
    assert!(config.auto_update_knowledge_base);
    assert_eq!(config.max_knowledge_base_rows, 100);
    assert!(config.databases.is_empty());
}

#[test]
fn test_kusto_config_serialization() {
    let mut config = KustoConfig::default();
    config.cluster_url = "https://help.kusto.windows.net".to_string();
    config.database = "Samples".to_string();
    config.auto_update_knowledge_base = false;
    config.max_knowledge_base_rows = 50;
    
    // Add a database
    let mut databases = HashMap::new();
    databases.insert("test".to_string(), KustoDatabaseConfig {
        name: "TestDB".to_string(),
        description: Some("Test database".to_string()),
        cluster_url: None,
        is_default: false,
    });
    config.databases = databases;
    
    // Serialize to TOML
    let toml_str = toml::to_string(&config).unwrap();
    
    // Deserialize back
    let deserialized_config: KustoConfig = toml::from_str(&toml_str).unwrap();
    
    assert_eq!(deserialized_config.cluster_url, config.cluster_url);
    assert_eq!(deserialized_config.database, config.database);
    assert_eq!(deserialized_config.auto_update_knowledge_base, config.auto_update_knowledge_base);
    assert_eq!(deserialized_config.max_knowledge_base_rows, config.max_knowledge_base_rows);
    assert_eq!(deserialized_config.databases.len(), 1);
    
    let test_db = &deserialized_config.databases["test"];
    assert_eq!(test_db.name, "TestDB");
    assert_eq!(test_db.description.as_ref().unwrap(), "Test database");
    assert!(!test_db.is_default);
}