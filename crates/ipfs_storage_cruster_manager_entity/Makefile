mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
DIR_ROOT := $(dir $(mkfile_path))

.PHONY: generate_entity

generate_entity:
	sea-orm-cli generate entity \
 	--database-url mysql://root:1234@localhost/ipfs_storage_cruster_manager \
 	--output-dir $(DIR_ROOT)src \
 	--lib \
 	 --with-serde both
