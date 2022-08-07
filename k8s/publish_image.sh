#!/bin/bash
set -euo pipefail
IFS=$'\n\t'
set -x

DOCKER_IMAGE=server
DOCKER_TAG=localhost:8050/$DOCKER_IMAGE

SCRIPT_DIR=$(dirname -- "$( readlink -f -- "$0"; )")

docker build -t $DOCKER_TAG -f server/Dockerfile ./server
docker push $DOCKER_TAG
