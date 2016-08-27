FROM        ubuntu:xenial


################################################################################
# Basic Development Tools
################################################################################

RUN     apt-get update -qq
RUN     apt-get upgrade -qq

RUN     apt-get install -qq wget
RUN     apt-get install -qq build-essential gcc


################################################################################
# Travis
################################################################################

RUN     apt-get install -qq ruby ruby-dev
RUN     gem install --no-ri --no-rdoc travis travis-lint


################################################################################
# git
################################################################################

RUN     apt-get install -qq git


################################################################################
# Rust
################################################################################

RUN    apt-get install -qq curl graphviz

RUN    curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV    CARGO_TARGET_DIR targetdocker
ENV    PATH /root/.cargo/bin:$PATH

RUN    cargo install rustfmt
RUN    cargo install cargo-watch
RUN    cargo install cargo-outdated
RUN    cargo install cargo-graph
RUN    cargo install cargo-modules
RUN    cargo install cargo-count

RUN    rustup install nightly
RUN    rustup run nightly cargo install clippy
