# fan

Command-line tool for controlling ThinkPad fan speed through `/proc/acpi/ibm/fan`.

## Requirements

- ThinkPad with `thinkpad_acpi` kernel module loaded

## Install

```bash
cargo build --release
sudo cp target/release/fan /usr/local/bin/
```

## Usage

```
fan <value>
```

| Value | Effect |
|-------|--------|
| stat, info | show current status |
| w | watch status, updates every 1s |
| on | enable fan |
| off | disable fan |
| auto | hand control back to firmware |
| max | full speed |
| min | level 1 |
| 1-7 | set specific level |

Commands that write to `/proc/acpi/ibm/fan` require root. If not run as root, the tool re-executes itself with `sudo` automatically.
