# Назва твого проєкту (можна змінити)
PROJECT_NAME := cyber-gpu-test

# Цільова архітектура
TARGET := aarch64-unknown-linux-gnu

# IP адреса твоєї консолі
DEVICE := cyberdog

# Куди копіювати на консолі
REMOTE_PATH := /userdata/bin

# Компільований бінарник
BINARY := target/$(TARGET)/release/$(PROJECT_NAME)

RG35 := ssh cyberdog

# =========================

.PHONY: all build deploy ssh clean docker run

all: build deploy

build:
	cargo build --release --target=$(TARGET)

deploy:
	$(RG35) "mkdir -p $(REMOTE_PATH)"
	scp $(BINARY) $(DEVICE):$(REMOTE_PATH)/$(PROJECT_NAME)
	$(RG35) "chmod +x $(REMOTE_PATH)/$(PROJECT_NAME)"

ssh:
	ssh $(DEVICE)

clean:
	cargo clean

docker:
	docker build -t cyberfps-builder .

run:
	docker run -it -v "$PWD":/cyber-gpu-test cyberfps-debug

