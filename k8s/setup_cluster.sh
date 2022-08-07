#!/bin/bash
set -euo pipefail
IFS=$'\n\t'
set -x

SCRIPT_DIR=$(dirname -- "$( readlink -f -- "$0"; )")

source $SCRIPT_DIR/variables.sh

k3d cluster create -p "$INGRESS_PORT:80@loadbalancer" --registry-create $K3D_REGISTRY:$DOCKER_REGISTRY_PORT --agents 2 mycluster

docker build -t $DOCKER_TAG -f server/Dockerfile ./server
docker push $DOCKER_TAG

CLUSTER_CONFIG=$(
  cat $SCRIPT_DIR/cluster-config.yaml |
  sed "s/{{K3D_REGISTRY}}/$K3D_REGISTRY/g" |
  sed "s/{{DOCKER_REGISTRY_PORT}}/$DOCKER_REGISTRY_PORT/g" |
  sed "s/{{DOCKER_IMAGE}}/$DOCKER_IMAGE/g"
)
echo "$CLUSTER_CONFIG" | kubectl apply -f -
