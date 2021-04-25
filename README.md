# Conference planner

A demo application used to demonstrate features of service meshes. It mimics a simple planning tool for conferences where speakers and sessions can be maintained and a resulting schedule can be displayed. 

It consists of four services:

* Speaker
* Session
* Schedule
* Frontend

It is prepared to demo routing features like A/B Testing, observability and resilience features by configureable delays and failures. 

It also includes a reverse proxy called frontend-proxy that allows testing multiple versions of the frontend-service without installing an ingress-controller. 

**Notice:** This is not an example of a good microservice architecture. It is for demonstration purposes of service mesh functionality only. In a real world application the functionality would probably be in a single service and would not need a service mesh.

## Features for building service mesh scenarios

### Loadgenerator

The Demo includes a container that generates load for the application. This is especially useful, when testing monitoring capabilities. You can find the configuration in the kubernetes folder. To activate the loadgenerator use:

`kubectl apply -f loadgenerator.yaml`

The loadbalancer will generate a random call on one of the urls of the frontend-proxy. There will be two calls per second. 

### Alternate Design

The frontend container reacts on setting an environment variable `ALTERNATE_DESIGN`. If that is set, some colors will change so that the alternate version is visible. Possible values are TRUE or FALSE. If not set it defaults to false. 

This is especially useful when demonstrating A/B Testing or other scenarios with multiple active versions.

### Random Errors

All services including Frontend can react on an environment variable `FAILURE_RATE`.

This can be set to a value between 0 and 100. This is understood as a percentage of calls that should fail. The default is 0 which means no failures. Setting the variable to 100 means all calls will fail with HTTP Code 503 - Service Unavailable. 

This is useful to demonstrate monitoring and retry mechanisms of Service Meshes.

### Random Delays

All services including Frontend can react on an environment variable `RANDOM_DELAY_MAX`.

This can be set to a value of zero or larger. The number is interpreted as milliseconds. If set the service will delay the answer for a random time between 0 and the set value of milliseconds. The default value is 0.

This is useful to demonstrate monitoring features and timeout features of service meshes.

### Tracing

All Services can create Tracing information and report this to a Collector that understands the Jaeger format. To activate the reporting set the environment variable `OTEL_EXPORTER_JAEGER_ENDPOINT` to an url where a collector is listening. 

This can be used to demonstrate tracing features in service meshes. 

## Installation

### Building and pushing the images from source

To build all images and push them to a registry use:

```
./build_and_push.sh <registry_prefix>
```
The argument is a prefix used for tagging and specifying the registry where the images will be pushed. If you are pushing to Docker Hub this would be your username. 

Make sure you logged in to the registry before starting the script. 

#### Open Issues

Currently this will only build single architecture images corresponding to the architecture used to build them. Please be aware when building on ARM CPUs.  

### Installing and testing in Kubernetes 

In the folder kubernetes is a number of basic configurations. There is also a folder with linkerd examples, that contains configurations for specific demonstrations. 

#### basic.yaml

This is the full setup of all services without any additional features like Traffic split or failures. 
Apply using:

```
kubectl apply -f basic.yaml
```

To view the application from your local host use port-forward:

```
kubectl port-forward service/frontend-proxy 8080:8080
```

Then open http://localhost:8080 on your local browser.

## Running locally for development

### Install Rust and Cargo

You will need to have Rust and it's build-tool cargo installed. The recommended way to install them is rustup which can be found under https://rustup.rs/ 

### Add services to /etc/hosts

Add the following to your local /etc/hosts:

```
# for demo app
127.0.0.1       speakers
127.0.0.1       schedule
127.0.0.1       sessions
127.0.0.1       frontend
```

This simulates the DNS Discovery usually done by Kubernetes and the service mesh.

### Starting all services together

There is a command to start all services at once. Just call:

```
./run-all.sh
```

This will use cargo run on all submodules. When you interrupt the command with ctrl-c you will kill all services together.

### Testing tracing locally

Run a local jaeger instance:
`docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest`

Start the demo app using:
`./run-all.sh`

The environment variable for the jaeger collector defaults to localhost:14268. So it is not necessary to set it for the tracing to work in local tests.