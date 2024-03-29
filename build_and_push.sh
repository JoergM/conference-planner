#! /bin/sh 

function error() {
    echo $1
    exit
}

function build_and_push_module() {
DOCKER_PREFIX=$1
MODULE_NAME=$2

cd ${MODULE_NAME}
MODULE_VERSION=$(cargo pkgid | cut -d# -f2 | cut -d: -f2)
cd ..

docker build . -t ${DOCKER_PREFIX}/${MODULE_NAME}:${MODULE_VERSION} -f Dockerfile-${MODULE_NAME} || error "Error building Image"
docker tag ${DOCKER_PREFIX}/${MODULE_NAME}:${MODULE_VERSION} ${DOCKER_PREFIX}/${MODULE_NAME}:latest
docker push ${DOCKER_PREFIX}/${MODULE_NAME}:${MODULE_VERSION} || error "Error pushing new version"
docker push ${DOCKER_PREFIX}/${MODULE_NAME} || error "Error pushing latest"

}

#main

DOCKER_PREFIX=$1 
[ -z "$DOCKER_PREFIX" ] && error "Please enter Docker Prefix as first argument (See README.md)"

build_and_push_module ${DOCKER_PREFIX} schedule
build_and_push_module ${DOCKER_PREFIX} frontend
build_and_push_module ${DOCKER_PREFIX} speaker
build_and_push_module ${DOCKER_PREFIX} session
build_and_push_module ${DOCKER_PREFIX} loadgenerator


