# image name & version
WRAPPER_NAME = ipfs_storage_cruster/ipfs_node_wrapper
MANAGER_NAME = ipfs_storage_cruster/ipfs_storage_cruster_manager
VERSION = latest

.PHONY: build-app1 build-app2 build-all up down

# build docker image of ipfs_node_wrapper_app
build-wrapper:
	docker build --build-arg APP_NAME=ipfs_node_wrapper_app -t $(WRAPPER_NAME):$(VERSION) .

# build docker image of ipfs_storage_cruster_app
build-manager:
	docker build --build-arg APP_NAME=ipfs_storage_cruster_app -t $(MANAGER_NAME):$(VERSION) .

build-all: build-wrapper build-manager

# run by docker compose
up:
	docker compose up -d
#	docker-compose up -d

# stop docker compose
down:
	docker compose down
#	docker-compose down

logs:
	docker compose logs -f
#	docker-compose logs -f
