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
# might have to use `sudo`
up:
# Permission denied if not create first.
	mkdir -p ./compose/manager-mysql/init
	cp ./sql/ipfs_storage_cruster_manager.sql ./compose/manager-mysql/init/
	docker compose up -d
#	docker-compose up -d

pure-up: compose-clean up

# stop docker compose
down:
	docker compose down
#	docker-compose down

logs:
	docker compose logs -f
#	docker-compose logs -f
