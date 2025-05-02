# README.md

# Cyber GPU Stress Test

**Cyber GPU Stress Test** is a compact visual stress testing utility designed for devices like **Anbernic RG35XX Plus** running **Knulli Custom Firmware**.

This tool renders hundreds of animated boxes bouncing on the screen to measure rendering performance and observe FPS behavior under heavy graphical load.

![Screenshot](./images/preview.png)

---

## 🚀 Features

- Realtime FPS display as a visual bar
- randomized objects with color and motion
- micro UI lib
- 2 modes (Basic stress test, relax mode ... +2 more in future)
- Fullscreen rendering for performance testing
- Monitor values: FPS, CPU/GPU/DDR temperature
- mini rect benchmark as RECT

---

## 🛠 Requirements

- Rust 1.76 or newer
- SDL2 and SDL2_ttf libraries
- Font path (customizable):
  `/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf`

---

## 🧪 Run Locally

```shell
cargo run --release
```

To cross-compile for Anbernic (aarch64):

```shell
make build
make deploy
```

---

## 🐳 Run via Docker (for Anbernic cross-build)

```shell
make docker
```

after make run docker
```shell
# cpu-stess-test project builder:
docker run -it -v "$PWD":/cyber-gpu-test cyberfps-debug
```

build only in docker:
```shell
make build
```

Then deploy (you can deploy without docker, as option use ssh or smb):
```shell
make deploy
```

Create Script `cyber-gpu-test.sh` and put in anbernic dir `roms/tools/`
```shell
#!/bin/sh

rm -f /tmp/cyb83rdo6/cyber-gpu-test
mkdir -p /tmp/cyb83rdo6/

if [ -f /userdata/bin/cyber-gpu-test ]; then
  cp /userdata/bin/cyber-gpu-test /tmp/cyb83rdo6/
  chmod +x /tmp/cyb83rdo6/cyber-gpu-test
  /tmp/cyb83rdo6/cyber-gpu-test
else
  echo "ERROR: cyber-gpu-test not found!"
fi
```

---

## 📂 Project Structure
- `src/main.rs` — main stress tests implementation
- `src/stress/*` — files for custom stress tests
- `src/ui/*` — self-made mini UI (menu, label, rgba background) lib (for next light projects)
- `src/ui/colors.rs` — all colors projects store here
- `src/ui/menu.rs` — all enums for menu items store here
## 📂Project on anbernic
- `roms/tools/` — launcher script on device `cyber-gpu-test.sh`
- `roms/tools/images` — put `cyber-gpu-test.png` in folder
- `userdata/bin/` — where binaries are placed
- `images/` — optional assets (logo, screenshots)

---

## 🎮 Verified On

- Anbernic RG35XX Plus
- Knulli CFW v0.3+
- SDL2 framebuffer (no X11)

## For Build on different platform:
- `.cargo/config.toml` change `build` and `target` as option
- current architecture (aarch64): `"aarch64-unknown-linux-gnu"`
- current (on docker) linker: `/usr/bin/aarch64-linux-gnu-gcc`

