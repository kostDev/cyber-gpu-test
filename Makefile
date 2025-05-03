PROJECT_NAME := cyber-gpu-test

# Цільова архітектура build only inside docker
TARGET := aarch64-unknown-linux-gnu

# IP адреса твоєї консолі root@your.address bind in system with auto ssh
DEVICE := cyberdog

# Куди копіювати на консолі
REMOTE_PATH := /userdata/bin

# Компільований бінарник
BINARY := target/$(TARGET)/release/$(PROJECT_NAME)

RG35 := ssh cyberdog

# =========================

.PHONY: all build deploy ssh clean docker run

all: build deploy
# build only inside docker
build:
	cargo build --release --target=$(TARGET)
	aarch64-linux-gnu-strip $(BINARY)
	@ls -lh $(BINARY)

debug:
	cargo build --target=$(TARGET)

deploy:
	$(RG35) "mkdir -p $(REMOTE_PATH)"
	scp -q $(BINARY) $(DEVICE):$(REMOTE_PATH)/$(PROJECT_NAME)
	$(RG35) "chmod +x $(REMOTE_PATH)/$(PROJECT_NAME)"

ssh:
	ssh $(DEVICE)

clean:
	cargo clean

docker:
	docker build -t cyberfps-builder .

run:
	docker rm -f cybergpu-dev 2>/dev/null || true
	docker run --rm --name cybergpu-dev -it \
      -v "$PWD":/cyber-gpu-test \
      -v cyber-cargo-cache:/root/.cargo \
      -v cyber-target-cache:/cyber-gpu-test/target \
      cyberfps-debug

strip:
	aarch64-linux-gnu-strip $(BINARY)

size:
	@ls -lh $(BINARY)

