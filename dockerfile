FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libsdl2-dev \
    libsdl2-ttf-dev \
    gcc-aarch64-linux-gnu \
    ca-certificates

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup target add aarch64-unknown-linux-gnu

WORKDIR /cyber-gpu-test
COPY . .

RUN mkdir -p .cargo
RUN echo '[target.aarch64-unknown-linux-gnu]' > .cargo/config.toml
RUN echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml

RUN cargo build --release --target=aarch64-unknown-linux-gnu