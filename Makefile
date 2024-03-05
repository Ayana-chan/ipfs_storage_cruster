# image name & version
WRAPPER_NAME=ipfs_storage_cruster/ipfs_node_wrapper
MANAGER_NAME=ipfs_storage_cruster/ipfs_storage_cruster_manager
VERSION=latest

.PHONY: build-app1 build-app2 build-all up down

# build ipfs_node_wrapper_app
build-wrapper:
	docker build --build-arg APP_NAME=ipfs_node_wrapper_app -t $(WRAPPER_NAME):$(VERSION) .

# build ipfs_storage_cruster_app
build-manager:
	docker build --build-arg APP_NAME=ipfs_storage_cruster_app -t $(MANAGER_NAME):$(VERSION) .

build-all: build-app1 build-app2

# 使用Docker Compose启动服务
up:
	docker-compose up -d

# 使用Docker Compose停止服务
down:
	docker-compose down
