FROM ubuntu:22.04

# 🧰 Ставимо всі потрібні пакети
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libsdl2-dev \
    libsdl2-ttf-dev \
    gcc-aarch64-linux-gnu \
    ca-certificates

# ⛓️ Встановлюємо Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# 🎯 Додаємо ARM64 ціль
RUN rustup target add aarch64-unknown-linux-gnu

# 🧱 Створюємо правильну конфігурацію з лінкером
WORKDIR /cyber-gpu-test
COPY . .

RUN mkdir -p .cargo
RUN echo '[target.aarch64-unknown-linux-gnu]\nlinker = "aarch64-linux-gnu-gcc"' > .cargo/config.toml

# 🚀 Компілюємо
RUN cargo build --release --target=aarch64-unknown-linux-gnu