## fan

CLI tool for ThinkPad fan speed control

## Install

### AUR

```bash
yay -S fan
```

### Manual

```bash
git clone https://github.com/NotiLo-A/fan.git
cd fan
cargo build --release
sudo install -Dm755 target/release/fan /usr/local/bin/fan
```

## Usage

```
fan <value>
```

| Value          | Effect                              |
|----------------|-------------------------------------|
| `help`         | show this help message              |
| `stat`, `info` | show fan stat                       |
| `w`            | watch fan stat (updates every 1s)   |
| `on`           | turn fan on                         |
| `off`          | turn fan off                        |
| `auto`         | set fan auto                        |
| `max`          | set fan full speed                  |
| `min`          | set fan min speed                   |
| `1`–`7`        | set fan speed level 1-7             |
| `fix`          | enable fan_control in thinkpad_acpi |

