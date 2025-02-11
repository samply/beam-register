# Stage 1: Prepare binaries and install required tools
FROM alpine AS chmodder
ARG TARGETARCH
ARG COMPONENT
ARG FEATURE

# Copy the specified binary component first
COPY ./target/ /app/$COMPONENT
#COPY /artifacts/binaries-$TARGETARCH$FEATURE/$COMPONENT /app/$COMPONENT

# Install Docker CLI, Docker Compose, and set executable permissions in one RUN statement
RUN apk add --no-cache docker docker-cli docker-compose && \
    chmod +x /app/*

# Stage 2: Distroless runtime
FROM gcr.io/distroless/cc-debian12
ARG COMPONENT

# Copy the component from the previous stage
COPY --from=chmodder /app/$COMPONENT /usr/local/bin/samply

# Copy Docker CLI and Compose binaries from the chmodder stage
COPY --from=chmodder /usr/bin/docker /usr/bin/docker
COPY --from=chmodder /usr/bin/docker-compose /usr/bin/docker-compose

ENTRYPOINT [ "/usr/local/bin/samply" ]
