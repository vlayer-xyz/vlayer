This Rust crate is designed to regenerate web proof fixtures for testing and development purposes.

# Prerequisites:

Ensure the required Docker services are up and running before executing the fixture generator.

```
docker compose -f ../../docker/web-proof/docker-compose-notary-v8.yaml up
```

# Usage:
Once the Docker services are active, execute the fixture generation process by running:
```
cargo run
```