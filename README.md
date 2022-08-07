# Requirements

- docker
- k3d
- kubectl

# Set up cluster

Create and configure a local cluster:

```bash
./k8s/setup_cluster.sh
```

Test that it works:

```bash
curl localhost:8081/health
```

Delete it again after use:

```bash
k3d cluster delete mycluster
```
