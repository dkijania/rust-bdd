# Simple dockerfile example of bdd implementation in rust
FROM ubuntu:latest
LABEL MAINTAINER DKIJANIA
LABEL description="rust bdd example"

ARG PREFIX=/app
ENV ENV_PREFIX=$PREFIX

# prepare the environment
RUN apt-get update && \
    apt-get install -y tzdata && \
    apt-get install -y git curl && \
    mkdir -p ${ENV_PREFIX} && \
    cd ${ENV_PREFIX} && \
    git clone https://github.com/dkijania/rust-bdd src
	
#install rustup
RUN  apt-get install -y build-essential pkg-config libssl-dev && \
    bash -c "curl https://sh.rustup.rs -sSf | bash -s -- -y" && \
    ~/.cargo/bin/rustup install stable && \
    ~/.cargo/bin/rustup default stable


# build project
RUN cd ${ENV_PREFIX}/src && \
    ~/.cargo/bin/cargo build


ARG API_KEY_ARG=""
ARG SECRET_KEY_ARG=""

# Endpoint in format 'https://api.com'
ARG API_ENDPOINT_ARG=""

# Controls debug printout during tests
# Enabled if 'true' 
ARG DEBUG_ARG=""

ENV API_KEY=$API_KEY_ARG
ENV SECRET_KEY=$SECRET_KEY_ARG
ENV API_ENDPOINT=$API_ENDPOINT_ARG

ENV DEBUG=$DEBUG_ARG

# run tests
WORKDIR ${ENV_PREFIX}/src
CMD [ "bash", "-c","~/.cargo/bin/cargo test -- --nocapture"]
