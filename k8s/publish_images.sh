#!/bin/bash
set -euo pipefail
IFS=$'\n\t'
set -x

SCRIPT_DIR=$(dirname -- "$( readlink -f -- "$0"; )")
RUST_WORKSPACE_DIR=$SCRIPT_DIR/../rust-workspace

DOCKER_REPOSITORY=localhost:8050

CURRENT_COMMIT_SHA=$(cd $SCRIPT_DIR && git rev-parse --short HEAD)
TIMESTAMP=$(date +"%s")
DOCKER_TAG_SUFFIX=-$CURRENT_COMMIT_SHA-$TIMESTAMP

# Publish chat-client image.
CHAT_CLIENT_DOCKER_TAG=chat-client$DOCKER_TAG_SUFFIX
docker build --target chat-client -t $DOCKER_REPOSITORY/$CHAT_CLIENT_DOCKER_TAG \
  -f $RUST_WORKSPACE_DIR/Dockerfile $RUST_WORKSPACE_DIR
docker push $DOCKER_REPOSITORY/$CHAT_CLIENT_DOCKER_TAG

# Publish replication-log image.
REPLICATION_LOG_DOCKER_TAG=replication-log$DOCKER_TAG_SUFFIX
docker build --target replication-log -t $DOCKER_REPOSITORY/$REPLICATION_LOG_DOCKER_TAG \
  -f $RUST_WORKSPACE_DIR/Dockerfile $RUST_WORKSPACE_DIR
docker push $DOCKER_REPOSITORY/$REPLICATION_LOG_DOCKER_TAG

# Make images known to helm.
tee $SCRIPT_DIR/helm/chat-cluster/values.yaml << EOF
ChatClientDockerTag: $CHAT_CLIENT_DOCKER_TAG
ReplicationLogDockerTag: $REPLICATION_LOG_DOCKER_TAG
EOF
