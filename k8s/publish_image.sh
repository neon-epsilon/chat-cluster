#!/bin/bash
set -euo pipefail
IFS=$'\n\t'
set -x

DOCKER_IMAGE=chat-client
DOCKER_TAG=localhost:8050/$DOCKER_IMAGE

SCRIPT_DIR=$(dirname -- "$( readlink -f -- "$0"; )")
CHAT_CLIENT_DIR=$SCRIPT_DIR/../chat-client

docker build -t $DOCKER_TAG -f $CHAT_CLIENT_DIR/Dockerfile $CHAT_CLIENT_DIR
docker push $DOCKER_TAG
