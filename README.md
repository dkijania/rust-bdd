# rust-bdd
Example of bdd in rust

## Master current build status

| CI | Status | Description |
|---:|:------:|:------------|
| CircleCI | ![CircleCI](https://circleci.com/gh/dkijania/rust-bdd.svg?style=shield) | Master |

# setup

Before running tests below env variables needs to be defined:

1. User api key
```
API_KEY="68m51U..."
```
2. Secret key
```
SECRET_KEY="b67VivGPzyumN4Rj52h..."
```
3. Api endpoint
```
API_ENDPOINT=https://api.sample.com
```

Optionally debug environment variable can be defined to allow steps and api debug printout
```
DEBUG=true
```

# docker

For demo purposed a Dockerfile is provided

## build

`docker build . -t rust-bdd`

## run

```
docker run -e API_ENDPOINT="..." -e API_KEY="..." -e SECRET_KEY="..." -e DEBUG="..." rust-bdd
```