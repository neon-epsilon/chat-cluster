# Requirements

- docker
- k3d
- kubectl
- helm

# Set up cluster

Create a local cluster and container registry:

```bash
./k8s/create_cluster.sh
```

Build the chat-client container and publish it to the registry:

```bash
./k8s/publish_chat_client_image.sh
```

Install the helm chart:

```bash
helm install chat-cluster ./k8s/helm/chat-cluster
```

# "Send" a chat message

For now, sending has to be done by accessing the redis-based message broker
manually:

```bash
kubectl exec -it service/message-broker-service -- redis-cli
127.0.0.1:6379> PUBLISH default-channel "Hello everyone!"
```

To check that it was received, use the `messages` endpoint:

```bash
curl localhot:8081/chat-client/messagess
```

# Delete the cluster after use

```bash
k3d cluster delete mycluster
```
