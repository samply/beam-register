# beam-register
Register new users in Beam

## Requirements

### Users environment variables file
Please create an environment file and add the path to BEAM_FILE_PATH

### Minimal environment variables
API_KEY=XXXXX
BEAM_FILE_PATH=users.env
RUST_LOG=info
BEAM_PROXY_CONTAINER_NAME=dummy-container

## Test without beam proxy container
Suggestion:

```
docker pull hello-world
docker run --name dummy-container -d alpine sleep 3600
export BEAM_PROXY_CONTAINER_NAME=dummy-container
```
