// Unit tests for argument parsing and config loading in main.rs
// These tests do not require Kafka connectivity.

use clap::Parser;
use super::*;

#[test]
fn test_args_parsing() {
    let args = Args::parse_from([
        "prog",
        "-b", "localhost:9092",
        "-t", "test-topic",
        "-g", "test-group",
        "-z", "1000",
        "-s", "settings.json",
        "-H", "key1:val1,key2:val2",
        "-k", "test-key",
        "-f", "body.txt"
    ]);
    assert_eq!(args.bootstrap_servers.as_deref(), Some("localhost:9092"));
    assert_eq!(args.topic.as_deref(), Some("test-topic"));
    assert_eq!(args.group.as_deref(), Some("test-group"));
    assert_eq!(args.broker_timeout_milis.as_deref(), Some("1000"));
    assert_eq!(args.settings.as_deref(), Some("settings.json"));
    assert_eq!(args.headers.as_deref(), Some("key1:val1,key2:val2"));
    assert_eq!(args.key.as_deref(), Some("test-key"));
    assert_eq!(args.body_file.as_deref(), Some("body.txt"));
}

#[test]
fn test_settings_deserialize() {
    let json = r#"{
        "bootstrap_servers": "localhost:9092",
        "topic": "test-topic",
        "group": "test-group",
        "broker_timeout_milis": "1000",
        "headers": "key1:val1,key2:val2",
        "key": "test-key",
        "body_file": "body.txt",
        "logging_level": "DEBUG",
        "ssl_enabled": false,
        "ssl_protocol": "TLSv1.2",
        "ssl_ca_file": "ca.pem",
        "ssl_client_cert_file": "client.pem",
        "ssl_private_key_location": "client.key",
        "ssl_key_password": "password",
        "sasl_mechanisms": "PLAIN",
        "sasl_username": "user",
        "sasl_password": "pass"
    }"#;
    let settings: Settings = serde_json::from_str(json).unwrap();
    assert_eq!(settings.bootstrap_servers, "localhost:9092");
    assert_eq!(settings.topic, "test-topic");
    assert_eq!(settings.group, "test-group");
    assert_eq!(settings.broker_timeout_milis, "1000");
    assert_eq!(settings.headers.as_deref(), Some("key1:val1,key2:val2"));
    assert_eq!(settings.key, "test-key");
    assert_eq!(settings.body_file, "body.txt");
}
