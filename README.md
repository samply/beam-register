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
docker run --name dummy-container -d --restart always alpine sh -c "echo 'Container started at: $(date)'; sleep 3600"
export BEAM_PROXY_CONTAINER_NAME=dummy-container
```

Or use the test docker-compose.yml

To build it manually:
docker build --build-arg TARGETARCH=amd64 --build-arg COMPONENT=beam-register --build-arg FEATURE=-prod -t beam-register .

For testing, you need to compile it manually with cargo build. 
The content of the target should be in /artifacts/binaries-$TARGETARCH$FEATURE/$COMPONENT
according to the Dockerfile. You can temporally change this line in the Dockerfile if you want to set the target in another place.