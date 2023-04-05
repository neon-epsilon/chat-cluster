#!/bin/bash
set -euo pipefail
IFS=$'\n\t'
set -x

INGRESS_PORT=8081
CLUSTER_NAME=mycluster
REGISTRY_NAME=mycluster-registry
REGISTRY_PORT=8050

k3d cluster create -p "$INGRESS_PORT:80@loadbalancer" --registry-create $REGISTRY_NAME:$REGISTRY_PORT --agents 2 $CLUSTER_NAME
