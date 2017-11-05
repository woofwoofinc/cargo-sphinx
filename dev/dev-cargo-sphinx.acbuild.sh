#!/usr/bin/env bash

set -xe


################################################################################
# Setup
################################################################################

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

TMP_DIR="$(mktemp -d -p "$DIR" dev-cargo-sphinx.XXXXXX)"
pushd "$TMP_DIR" > /dev/null


################################################################################
# Download Base Image
################################################################################

wget http://cdimage.ubuntu.com/ubuntu-base/releases/17.10/release/ubuntu-base-17.10-base-amd64.tar.gz


################################################################################
# Start Image Build
################################################################################

acbuild begin --build-mode=oci ./ubuntu-base-17.10-base-amd64.tar.gz


################################################################################
# Basic Development Tools
################################################################################

acbuild run -- apt-get update -qq
acbuild run -- apt-get upgrade -qq

acbuild run -- apt-get install -qq wget
acbuild run -- apt-get install -qq build-essential
acbuild run -- apt-get install -qq git


################################################################################
# Sphinx
################################################################################

# Python pip is in Ubuntu universe.
acbuild run -- apt-get install -qq software-properties-common
acbuild run -- apt-add-repository universe
acbuild run -- apt-get update -qq

acbuild run -- apt-get install -qq python2.7
acbuild run -- apt-get install -qq python-pip
acbuild run -- pip install -q --upgrade pip

acbuild run -- pip install -q Sphinx
acbuild run -- pip install -q sphinx_bootstrap_theme


################################################################################
# Rust
################################################################################

acbuild run -- apt-get install -qq curl graphviz cmake libssl-dev pkg-config

acbuild run -- curl -sSf https://sh.rustup.rs -o rustup.sh
acbuild run -- sh rustup.sh -y
acbuild run -- rm rustup.sh

# The PATH locations for the Rust binaries are automatically added to .profile
# but this isn't read when the Bash entry point is executed in the container.
acbuild run -- bash -c 'echo "export PATH=\"\$HOME/.cargo/bin:\$PATH\"" >> /root/.bashrc'

acbuild run -- /root/.cargo/bin/cargo install rustfmt
acbuild run -- /root/.cargo/bin/cargo install cargo-outdated
acbuild run -- /root/.cargo/bin/cargo install cargo-sphinx

acbuild run -- /root/.cargo/bin/rustup install nightly
acbuild run -- /root/.cargo/bin/rustup run nightly cargo install clippy


################################################################################
# Set Image Executable
################################################################################

acbuild set-exec -- /bin/bash


################################################################################
# Finalise Image
################################################################################

acbuild run -- apt-get -qq autoremove
acbuild run -- apt-get -qq clean

acbuild write --overwrite ../dev-cargo-sphinx.oci

acbuild end


################################################################################
# Teardown
################################################################################

popd > /dev/null
rm -fr "$TMP_DIR"
