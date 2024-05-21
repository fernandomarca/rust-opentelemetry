metadata:
  name: app-dev
  annotations:
    "sidecar.jaegertracing.io/inject": "true"

kubectl --kubeconfig config create namespace observability 
kubectl --kubeconfig config create -f https://github.com/jaegertracing/jaeger-operator/releases/download/v1.49.0/jaeger-operator.yaml -n observability

