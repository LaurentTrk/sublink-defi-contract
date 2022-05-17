FROM gitpod/workspace-full

RUN sudo apt update && \
    sudo apt install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev curl && \
    curl https://sh.rustup.rs -sSf | sh && \
    source ~/.cargo/env && \
    rustup default stable && \
    rustup update nightly && \
    rustup update stable && \
    rustup target add wasm32-unknown-unknown --toolchain nightly
