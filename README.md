# kafka_producer
A basic Kafka producer to aid in development activities written in Rust.

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