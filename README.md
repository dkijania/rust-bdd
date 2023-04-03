# rust-bdd
Example of bdd in rust-develop and rampup

## Master current build status

| CI | Status | Description |
|---:|:------:|:------------|
| CircleCI | ![CircleCI](https://circleci.com/gh/dkijania/rust-bdd.svg?style=shield) | Master |

# setup

## build

`cargo build`

## test

Before running tests below env variables needs to be defined:

1. User api key
```
set API_KEY="68m51U..."
```
2. Secret key
```
set SECRET_KEY="b67VivGPzyumN4Rj52h..."
```
3. Api endpoint
```
set API_ENDPOINT=https://api.sample.com
```

Optionally debug environment variable can be defined to allow steps and api debug printout
```
set DEBUG=true
```

### to see test output

`cargo test -- --nocapture`

# docker

For demo purposed a Dockerfile is provided

## build

`docker build . -t rust-bdd`

## run

```
docker run -e API_ENDPOINT="..." -e API_KEY="..." -e SECRET_KEY="..." -e DEBUG="..." rust-bdd
```
