# Ferrix - Modern System Information Tool for Linux

A lightweight, fast and modern system information tool for Linux, built with Rust and Iced.

![](assets/main_win.png)

## Features

### System Overview

- **Dashboard** with key system metrics at a glance;
- **Real time monitoring** of CPU, memory and others PC's components;
- **System health scoring** based on multiple factors.

### Hardware Information

- Processor details;
- Memory specifications;
- Storage information;
- Battery status;
- Data from DMI tables;

### Software Information

- Installed system;
- Kernel information;
- Users and groups;
- `systemd` services;

### User Experience

- **Modern, clean interface** based on [iced](https://iced.rs);
- **Minimal dependencies** (`systemd`, `glibc`);
- **Fast startup** and low memory footprint.

## Installation

```bash
cargo install ferrix-app
```

... or download pre-built binaries from [GitHub Releases](https://github.com/mskrasnov/ferrix/releases/latest)

... or build Ferrix from sources:

```bash
git clone https://github.com/mskrasnov/ferrix
cd ferrix

cargo build --release
./target/release/ferrix-app
```

### Build dependencies

- **Rust** 2025;
- Modern Linux system with `systemd` (tested on Debian 12.1);

## Technology stack

- **OS:** Linux with `glibc` and `systemd`;
- **Multilanguage:** coming soon...
- **Programming lang.:** [Rust](https://rust-lang.org);
- **GUI:** [iced](https://iced.rs);

<a href="https://iced.rs">
  <img alt="iced" title="iced" src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="350px">
</a>
    
## License

Ferrix (`ferrix-lib`, `ferrix-app`) is distributed under the [GNU GPLv3](LICENSE) license.
