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

Build the containers and publish them to the registry:

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

To check that it was received, access the `chat-server` service:

```bash
curl localhost:8081/chat-server/messages
```

The message should also be stored and accessible through the `replication-log` service:

```bash
curl localhost:8081/replication-log/messages/default-channel
```

## Confirm that replication log is correctly being used

When a new `chat-server` instance starts up, it should retrieve the list of already sent messages from the replication log. To test this, force a re-deployment:

```bash
kubectl rollout restart deploy chat-server-deployment
```

Once the `chat-server` instances are back up, we can check that they retrieved the previously sent messages:

```bash
curl localhost:8081/chat-server/messages
```

# Delete the cluster after use

```bash
k3d cluster delete mycluster
```
