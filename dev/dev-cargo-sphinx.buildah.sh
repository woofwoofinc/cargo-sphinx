#!/usr/bin/env bash

set -xe


################################################################################
# Setup
################################################################################

IMAGE=dev-cargo-sphinx
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

TMP_DIR="$(mktemp -d -p "$DIR" $IMAGE.XXXXXX)"
pushd "$TMP_DIR" > /dev/null


################################################################################
# Start Image Build
################################################################################

buildah from scratch --name $IMAGE


################################################################################
# Base Image
################################################################################

wget http://cdimage.ubuntu.com/ubuntu-base/releases/17.10/release/ubuntu-base-17.10-base-amd64.tar.gz

MOUNT=$(buildah mount $IMAGE)
tar xzf ubuntu-base-17.10-base-amd64.tar.gz -C "$MOUNT"
buildah umount $IMAGE


################################################################################
# Basic Development Tools
################################################################################

buildah run $IMAGE -- apt-get update -qq
buildah run $IMAGE -- apt-get upgrade -qq

buildah run $IMAGE -- apt-get install -qq wget
buildah run $IMAGE -- apt-get install -qq build-essential
buildah run $IMAGE -- apt-get install -qq git


################################################################################
# Sphinx
################################################################################

# Python pip is in Ubuntu universe.
buildah run $IMAGE -- apt-get install -qq software-properties-common
buildah run $IMAGE -- apt-add-repository universe
buildah run $IMAGE -- apt-get update -qq

buildah run $IMAGE -- apt-get install -qq python2.7
buildah run $IMAGE -- apt-get install -qq python-pip
buildah run $IMAGE -- pip install -q --upgrade pip

buildah run $IMAGE -- pip install -q Sphinx
buildah run $IMAGE -- pip install -q sphinx_bootstrap_theme


################################################################################
# Rust
################################################################################

buildah run $IMAGE -- apt-get install -qq curl graphviz cmake libssl-dev
buildah run $IMAGE -- apt-get install -qq pkg-config

buildah run $IMAGE -- curl -sSf https://sh.rustup.rs -o rustup.sh
buildah run $IMAGE -- sh rustup.sh -y
buildah run $IMAGE -- rm rustup.sh

# The PATH locations for the Rust binaries are automatically added to .profile
# but this isn't read when the Bash entry point is executed in the container.
buildah run $IMAGE -- bash -c 'echo "export PATH=\"\$HOME/.cargo/bin:\$PATH\"" >> /root/.bashrc'

buildah run $IMAGE -- /root/.cargo/bin/cargo install cargo-outdated
buildah run $IMAGE -- /root/.cargo/bin/cargo install cargo-sphinx

buildah run $IMAGE -- /root/.cargo/bin/rustup install nightly
buildah run $IMAGE -- /root/.cargo/bin/cargo +nightly install rustfmt-nightly --force
buildah run $IMAGE -- /root/.cargo/bin/cargo +nightly install clippy


################################################################################
# Finalise Image
################################################################################

buildah run $IMAGE -- apt-get -qq autoremove
buildah run $IMAGE -- apt-get -qq clean

echo "nameserver 8.8.8.8" > resolv.conf
buildah copy $IMAGE resolv.conf /etc/resolv.conf

buildah config $IMAGE --entrypoint /bin/bash

buildah commit -rm $IMAGE $IMAGE


################################################################################
# Output Image
################################################################################

buildah push $IMAGE oci:$IMAGE:latest
tar cf ../$IMAGE.oci -C $IMAGE .


################################################################################
# Teardown
################################################################################

buildah rmi $IMAGE

popd > /dev/null
rm -fr "$TMP_DIR"
