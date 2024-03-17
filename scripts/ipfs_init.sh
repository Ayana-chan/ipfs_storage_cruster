#!/bin/sh

set -ex
ipfs bootstrap rm all
# Disable auto IPFS node discovery
ipfs config --bool Discovery.MDNS.Enabled false
