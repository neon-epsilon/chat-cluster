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
helm install ./k8s/helm/chat-cluster chat-cluster
```

Test that it works:

```bash
curl localhost:8081/health
```

Delete it again after use:

```bash
helm uninstall chat-cluster
```
