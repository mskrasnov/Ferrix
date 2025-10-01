<div align="center">
  <img src="ferrix-app/data/icons/hicolor/scalable/apps/com.mskrasnov.Ferrix.svg" width="200">
  <h1>Ferrix — Swiss Knife for Linux Hardware Diagnostics</h1>
  <p><b>A simple program for getting information about computer hardware and installed software.</b></p>
  <!-- <h4> -->
    <!-- <a href="" -->
  <!-- </h4> -->
  <img src="assets/main_win.png" width="45%">

  [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![GitHub Release](https://img.shields.io/github/v/release/mskrasnov/ferrix?logo=github)](https://github.com/mskrasnov/ferrix/releases)
</div>

A lightweight, fast and modern system information tool for Linux, built with Rust and Iced.

## What is Ferrix?

Ferrix is a Rust-crate and program for obtaining information about computer hardware and software. It is designed to work in modern Linux OS distributions.

## Motivation

For Linux, there are a bunch of different console programs for getting information about various PC and OS components. In addition, there is an amazing graphics program called Hardinfo (and its sequel, Hardinfo2). However, I wanted to write their simple analog with the following differences:

1. More complete support for modern Linux distributions: displaying information about systemd services, OS boot time, information about installed Flatpak packages, and the ability to reset GNOME environment settings (**NOTE:** I haven't implemented exactly these functions yet, but I plan to implement them in the future. They will be included in Ferrix v1.0);
2. I needed experience in developing a similar class of programs for Linux. Don't think that Ferrix is any kind of serious professional program — it's just a student's "DIY", nothing more. Use it at your own risk.

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

## ❤️ Support Ferrix

Developing Ferrix takes time and passion. If you find it useful, please consider supporting its development:

- **Star ⭐ this repo!** It helps others discover Ferrix;
- **Write comments, questions, bug reports, or suggestions** for new functionality in [issues](https://github.com/mskrasnov/Ferrix/issues/new).
- If you are from Russia, **send me a donation 💰** to the card: `2202 2062 5233 5406` (Sberbank). This will help me keep my enthusiasm alive, as well as pay my mobile internet bills so that I can continue working on Ferrix.
- **Spread the world!** Tell friends, post on forums.
    
## License

Ferrix (`ferrix-lib`, `ferrix-app`) is distributed under the [GNU GPLv3](../LICENSE) license.
