# Goal

TODO

# Architecture

TODO

# Running the cluster

## Requirements

- docker
- k3d
- kubectl
- helm

## Set up cluster

Create a local cluster and container registry:

```bash
./k8s/create_cluster.sh
```

Build the containers and publish it to the registry:

```bash
./k8s/publish_images.sh
```

Install the helm chart:

```bash
helm install chat-cluster ./k8s/helm/chat-cluster
```

## "Send" a chat message

For now, sending has to be done by accessing the redis-based message broker
manually:

```bash
kubectl exec -it service/message-broker-service -- redis-cli
127.0.0.1:6379> PUBLISH default-channel "Hello everyone!"
```

To check that it was received, use the `messages` endpoint:

```bash
curl localhost:8081/chat-server/messages
```

## Inspect the replication log

TODO

## Confirm that rolling upgrades work

TODO: Add a manual or automatic test to demonstrate that scaling/rolling upgrades work, more specifically, that new chat servers retrieve chat messages from the replication log.

## Delete the cluster after use

```bash
k3d cluster delete mycluster
```
