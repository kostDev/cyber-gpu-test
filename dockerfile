FROM ubuntu:22.04

RUN dpkg --add-architecture arm64 && \
    apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    gcc-aarch64-linux-gnu \
    ca-certificates \
    libsdl2-dev:arm64 \
    libsdl2-ttf-dev:arm64

# Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup target add aarch64-unknown-linux-gnu

# linker
RUN ln -s /usr/bin/aarch64-linux-gnu-gcc /usr/bin/aarch64-unknown-linux-gnu-gcc
RUN echo "✅ SYMLINK:" && ls -l /usr/bin/aarch64-unknown-linux-gnu-gcc

# pkg-config під ARM
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH="/usr/lib/aarch64-linux-gnu/pkgconfig"

# project
WORKDIR /cyber-gpu-test
COPY . .

# build project
RUN cargo build --release --target=aarch64-unknown-linux-gnu