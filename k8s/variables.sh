INGRESS_PORT=8081

K3D_REGISTRY=mycluster-registry
DOCKER_REGISTRY_PORT=8050

DOCKER_IMAGE=server
DOCKER_TAG=localhost:$DOCKER_REGISTRY_PORT/$DOCKER_IMAGE

