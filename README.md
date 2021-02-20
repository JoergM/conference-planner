# Conference planner

A demo application used to demonstrate features of service meshes. It mimics a simple planning tool for conferences where speakers and sessions can be maintained and a resulting schedule can be displayed. 

It consists of four services:

* Speaker
* Session
* Schedule
* Frontend

It is prepared to demo routing features like A/B Testing, observability and resilience features by configureable delays and failures. 

## Installation

### Building the images from source

### Pushing the images

### Installing in Kubernetes 


## Usage

### A/B Testing

### Blue Green Release

### Canary Release

### Delay-injection

### Failure injection

### Open Telemetry

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