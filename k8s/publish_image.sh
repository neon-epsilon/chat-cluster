#!/bin/bash
set -euo pipefail
IFS=$'\n\t'
set -x

DOCKER_TAG=chat-client
DOCKER_REPOSITORY=localhost:8050

SCRIPT_DIR=$(dirname -- "$( readlink -f -- "$0"; )")
CHAT_CLIENT_DIR=$SCRIPT_DIR/../chat-client

docker build -t $DOCKER_REPOSITORY/$DOCKER_TAG -f $CHAT_CLIENT_DIR/Dockerfile $CHAT_CLIENT_DIR
docker push $DOCKER_REPOSITORY/$DOCKER_TAG

tee $SCRIPT_DIR/helm/chat-cluster/values.yaml << EOF
ChatClientDockerTag: $DOCKER_TAG
EOF
