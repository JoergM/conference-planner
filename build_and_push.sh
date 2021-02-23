#! /bin/sh

function error() {
    echo $1
    exit
}

DOCKER_PREFIX=$1 
[ -z "$DOCKER_PREFIX" ] && error "Please enter Docker Prefix as first argument (See README.md)"

cd frontend
docker build -t ${DOCKER_PREFIX}/frontend .
docker push ${DOCKER_PREFIX}/frontend 

cd ../schedule
docker build -t ${DOCKER_PREFIX}/schedule .
docker push ${DOCKER_PREFIX}/schedule 

cd ../speaker
docker build -t ${DOCKER_PREFIX}/speaker .
docker push ${DOCKER_PREFIX}/speaker 

cd ../session
docker build -t ${DOCKER_PREFIX}/session .
docker push ${DOCKER_PREFIX}/session 


