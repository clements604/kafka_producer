use std::fs;

use log::{debug, error, info};

use atty::Stream;
use clap::Parser;
use std::io::{self, Read};

use config::Config;
use rdkafka::config::ClientConfig;
use rdkafka::message::Header;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Deserialize;

mod main_test;

static DEFAULT_SETTINGS_FILE: &str = "settings.json";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'b', long = "bootstrap")]
    bootstrap_servers: Option<String>,
    #[arg(short = 't', long = "topic")]
    topic: Option<String>,
    #[arg(short = 'g', long = "group")]
    group: Option<String>,
    #[arg(short = 'z', long = "timeout")]
    broker_timeout_milis: Option<String>,
    #[arg(short = 's', long = "settings")]
    settings: Option<String>,
    #[arg(short = 'H', long = "headers")]
    headers: Option<String>,
    #[arg(short = 'k', long = "key")]
    key: Option<String>,
    #[arg(short = 'f', long = "body_file")]
    body_file: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    bootstrap_servers: String,
    topic: String,
    group: String,
    broker_timeout_milis: String,
    headers: Option<String>,
    key: String,
    body_file: String,
    logging_level: LoggingLevel,
    ssl_enabled: bool,
    ssl_protocol: String,
    ssl_ca_file: String,
    ssl_client_cert_file: String,
    ssl_private_key_location: String,
    ssl_key_password: String,
    sasl_mechanisms: String,
    sasl_username: String,
    sasl_password: String,
}

#[derive(Debug, Deserialize, Clone)]
enum LoggingLevel {
    ERROR,
    INFO,
    DEBUG,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let settings_file_path = match args.settings {
        Some(settings) => settings,
        None => String::from(DEFAULT_SETTINGS_FILE),
    };
    let settings: Settings = Config::builder()
        .add_source(config::File::with_name(&settings_file_path))
        .build()?
        .try_deserialize()?;

    let logging_level = match settings.logging_level {
        LoggingLevel::DEBUG => log::LevelFilter::Debug,
        LoggingLevel::ERROR => log::LevelFilter::Error,
        LoggingLevel::INFO => log::LevelFilter::Info,
    };
    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(logging_level)
        .is_test(false)
        .try_init();

    let key = match args.key {
        Some(key) => key,
        None => settings.key,
    };

    let body = read_body(args.body_file, settings.body_file)?;

    let bootstrap_servers = match args.bootstrap_servers {
        Some(bootstrap_servers) => bootstrap_servers,
        None => settings.bootstrap_servers,
    };

    let topic = match args.topic {
        Some(topic) => topic,
        None => settings.topic,
    };

    let group = match args.group {
        Some(group) => group,
        None => settings.group,
    };

    let broker_timeout_milis: String = match args.broker_timeout_milis {
        Some(broker_timeout_milis) => broker_timeout_milis,
        None => settings.broker_timeout_milis,
    };

    let headers_str = match args.headers {
        Some(h) => Some(h),
        None => settings.headers,
    };
    let mut kafka_headers = OwnedHeaders::new();
    if let Some(headers_str) = headers_str {
        for header in headers_str.split(',') {
            let mut kv = header.splitn(2, ':');
            if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                kafka_headers = kafka_headers.insert(Header {
                    key,
                    value: Some(value),
                });
            }
        }
    }

    let ssl_enabled = settings.ssl_enabled;

    debug!("Bootstrap servers: {}", bootstrap_servers);
    debug!("Topic: {}", topic);
    debug!("Group: {}", group);

    let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &bootstrap_servers);
        client_config.set("socket.timeout.ms", &broker_timeout_milis);
        client_config.set("message.timeout.ms", &broker_timeout_milis);
        client_config.set("request.timeout.ms", &broker_timeout_milis);
        client_config.set("retries", "0");
        client_config.set("socket.keepalive.enable", "true");

    if ssl_enabled {
        debug!("SSL enabled, setting client config values");
        let ssl_protocol: String = settings.ssl_protocol.clone();
        match ssl_protocol.as_str() {
            "ssl" => {
                debug!("SSL protocol");
                info!("SSL protocol functionality is untested!");
                client_config.set("security.protocol", ssl_protocol);
                client_config.set("ssl.ca.location", settings.ssl_ca_file);
                client_config.set("ssl.certificate.location", settings.ssl_client_cert_file);
                client_config.set("ssl.key.location", settings.ssl_private_key_location);
                client_config.set("ssl.key.password", settings.ssl_key_password);
            },
            "sasl_ssl" => {
                debug!("SA_SSL protocol");
                info!("SA_SSL protocol functionality is untested!");
                client_config.set("security.protocol", ssl_protocol);
                client_config.set("sasl.mechanisms", settings.sasl_mechanisms);
                client_config.set("sasl.username", settings.sasl_username);
                client_config.set("sasl.password", settings.sasl_password);
                client_config.set("ssl.ca.location", settings.ssl_ca_file);
            },
            _ => {
                panic!("Unknown SSL protocol");
            }
        }
    }

    let producer: FutureProducer = client_config.create()?;

    let delivery_status = producer.send(
        FutureRecord::to(&topic)
            .payload(&body)
            .key(&key)
            .headers(kafka_headers),
        std::time::Duration::from_secs(5),
    );

    match futures::executor::block_on(delivery_status) {
        Ok(delivery) => {
            info!(
                "Message delivered to topic {} partition {} at offset {}",
                topic, delivery.partition, delivery.offset
            );
            Ok(())
        }
        Err((err, _)) => {
            error!("Delivery failed: {}", err);
            Err(err.into())
        }
    }
}

/*
 * Read body to send to Kafka topic.
 * This body can be provided via stdin, or from file.
*/
fn read_body(args_body: Option<String>, settings_body: String) -> io::Result<String> {
    if !atty::is(Stream::Stdin) {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        return Ok(buf);
    }

    // Not stdin, try getting file from args, otherwise default to body file defined in settings
    let body_file = args_body.unwrap_or(settings_body);
    fs::read_to_string(body_file)
}
