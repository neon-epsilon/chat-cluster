#!/bin/bash
set -euo pipefail
IFS=$'\n\t'
set -x

SCRIPT_DIR=$(dirname -- "$( readlink -f -- "$0"; )")

CURRENT_COMMIT_SHA=$(cd $SCRIPT_DIR && git rev-parse --short HEAD)
TIMESTAMP=$(date +"%s")
DOCKER_TAG=chat-client-$CURRENT_COMMIT_SHA-$TIMESTAMP
DOCKER_REPOSITORY=localhost:8050

RUST_WORKSPACE_DIR=$SCRIPT_DIR/../rust-workspace

docker build --target chat-client -t $DOCKER_REPOSITORY/$DOCKER_TAG -f $RUST_WORKSPACE_DIR/Dockerfile $RUST_WORKSPACE_DIR
docker push $DOCKER_REPOSITORY/$DOCKER_TAG

tee $SCRIPT_DIR/helm/chat-cluster/values.yaml << EOF
ChatClientDockerTag: $DOCKER_TAG
EOF
