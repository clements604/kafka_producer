# Kafka Producer
A basic Kafka producer to aid in development activities written in Rust.

# Features

- Send messages to Kafka topics with custom headers and keys
- Supports configuration via command-line or JSON settings file
- SSL and SASL authentication options
- Logging level configuration
- Loads message body from file
- Flexible header and key specification

# Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/clements604/kafka_producer.git
cd kafka_producer
cargo build --release
```

# Testing

Run the test suite with:

```bash
cargo test
```

Tests cover argument parsing and settings deserialization. No Kafka connectivity is required for tests.

## Building

### Debug binary
```bash
cargo build
```
### Release binary
```bash
cargo build --release
```

### Sample settings.json
```json
{
    "bootstrap_servers": "localhost:9092",
    "topic": "test-topic",
    "group": "test-group",
    "broker_timeout_milis": "10000",
    "headers": "header1:value1,header2:value2",
    "key": "test key 1",
    "body_file": "test_message.txt",
    "logging_level": "DEBUG",
    "ssl_enabled": false,
    "ssl_protocol": "ssl",
    "ssl_ca_file": "",
    "ssl_client_cert_file": "",
    "ssl_private_key_location": "",
    "ssl_key_password": "",
    "sasl_mechanisms": "SCRAM-SHA-512",
    "sasl_username": "",
    "sasl_password": ""
}
```

## Usage

### Command-line Usage

You can run the Kafka producer with command-line arguments or a settings file. Command-line arguments override settings from the file.

#### Basic Example

```bash
./target/debug/kafka_producer \
    -b localhost:9092 \
    -t test-topic \
    -g test-group \
    -z 10000 \
    -H "header1:value1,header2:value2" \
    -k "test key 1" \
    -f test_message.txt
```

#### Arguments

| Short | Long           | Description                                 |
|-------|----------------|---------------------------------------------|
| -b    | --bootstrap    | Kafka bootstrap servers (host:port)         |
| -t    | --topic        | Kafka topic to send to                      |
| -g    | --group        | Consumer group (for config compatibility)   |
| -z    | --timeout      | Broker timeout in milliseconds              |
| -s    | --settings     | Path to settings JSON file                  |
| -H    | --headers      | Comma-separated headers (key:value,...)     |
| -k    | --key          | Message key                                 |
| -f    | --body_file    | Path to file containing message body        |

#### Example settings file
See the [Sample settings.json](#sample-settingsjson) section above for all available options.

#### Notes
- If both a command-line argument and a settings file value are provided, the command-line argument takes precedence.
- The message body is loaded from the file specified by `-f`/`--body_file`.
- Headers should be provided as a comma-separated list, e.g. `-H "foo:bar,baz:qux"`.
- SSL and SASL options are supported via the settings file.

## Local Kafka
### Confluent documentation
- https://docs.confluent.io/platform/current/get-started/platform-quickstart.html+
#### Commands
```bash
# Clone the Confluent Platform all-in-one example repository
git clone https://github.com/confluentinc/cp-all-in-one.git
# Change to the cloned repository’s root directory:
cd cp-all-in-one
# The default branch may not be the latest. Check out the branch for the version you want to run, for example, 8.1.0-post
git checkout 8.1.0-post
# The docker-compose.yml file is located in a nested directory. Navigate into the following directory:
cd cp-all-in-one
# Start the Confluent Platform stack with the -d option to run in detached mode:
docker compose up -d
# After a few minutes, if the state of any component isn’t Up, run the docker compose up -d command again, or try docker compose restart <image-name>, for example:
docker compose restart control-center

docker compose up -d # Start local cluster
docker compose down # Delete local cluster
```
### Local cluster link
http://localhost:9021/clusters/