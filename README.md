```bash
$ k3d cluster create -p "8081:80@loadbalancer" --agents 2 testcluster
$ kubectl apply -f cluster-config.yaml
$ curl localhost:8081
```

