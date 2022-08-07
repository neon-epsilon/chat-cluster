# Requirements

- docker
- k3d
- kubectl

# Set up cluster

Create a local cluster and container registry:

```bash
./k8s/create_cluster.sh
```

Build the container and publish it:

```bash
./k8s/publish_image.sh
```

Configure the cluster:

```bash
kubectl apply -f ./k8s/cluster_config.yaml
```

Test that it works:

```bash
curl localhost:8081/health
```

Delete it again after use:

```bash
k3d cluster delete mycluster
```
