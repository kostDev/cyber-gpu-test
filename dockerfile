FROM --platform=linux/arm64 ubuntu:22.04

# 🧰 Додаємо підтримку ARM64 архітектури
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    gcc-aarch64-linux-gnu \
    libsdl2-dev:arm64 \
    libsdl2-ttf-dev:arm64 \
    ca-certificates

# ⛓️ Встановлюємо Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# 🎯 Додаємо ARM64 ціль
RUN rustup target add aarch64-unknown-linux-gnu

# 🧱 Переходимо в директорію проєкту
WORKDIR /cyber-gpu-test
COPY . .

# ⚙️ Створюємо `.cargo/config.toml` з вказанням лінкера і шляхом до lib'ів
RUN mkdir -p .cargo && \
    echo '[target.aarch64-unknown-linux-gnu]' > .cargo/config.toml && \
    echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml && \
    echo 'rustflags = ["-C", "link-args=-L/usr/lib/aarch64-linux-gnu"]' >> .cargo/config.toml

# 🚀 Компілюємо
RUN cargo build --release --target=aarch64-unknown-linux-gnu