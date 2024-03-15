# image name & version
WRAPPER_NAME = ipfs_storage_cruster/ipfs_node_wrapper
MANAGER_NAME = ipfs_storage_cruster/ipfs_storage_cruster_manager
VERSION = latest

.PHONY: build-wrapper build-manager build-all up down

# build docker image of ipfs_node_wrapper_app
build-wrapper:
	docker build --build-arg APP_NAME=ipfs_node_wrapper_app -t $(WRAPPER_NAME):$(VERSION) .

# build docker image of ipfs_storage_cruster_manager_app
build-manager:
	docker build --build-arg APP_NAME=ipfs_storage_cruster_manager_app -t $(MANAGER_NAME):$(VERSION) .

build-all: build-wrapper build-manager

# might have to use `sudo`
compose-clean:
	rm -rf ./compose

# run by docker compose
up:
	docker compose up -d
#	docker-compose up -d

# clear all data and start containers
pure-up: compose-clean up

# stop containers
stop:
	docker compose stop
#	docker-compose down

# stop and delete containers
down:
	docker compose down
#	docker-compose down

logs:
	docker compose logs -f
#	docker-compose logs -f
