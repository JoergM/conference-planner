# Example Configurations for Linkerd

### Alternate Design

Inside the folder kubernetes is a configuration that includes a Traffic split for e.g. A/B Testing the frontend. It also installs two frontend versions. They use different environment variables to show different designs. 
Apply using:

```
kubectl apply -f a_b.yaml
```

The Split is set to 50%. To test it just reload the webapplication or start the load-generator and observe the result in the service-mesh dashboard.

The frontend container reacts on setting an environment variable `ALTERNATE_DESIGN`. If that is set, some colors will change so that the alternate version is visible.

### Blue Green Release

TBD

### Canary Release

TBD

### Delay-injection

TBD

### Failure injection

TBD

### Tracing with Jaeger

Testing locally:

Run a local jaeger instance:
`docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest`

Start the demo app using:
`./run-all.sh`