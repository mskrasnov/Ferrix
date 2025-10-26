<div align="center">
  <img src="ferrix-app/data/icons/hicolor/scalable/apps/com.mskrasnov.Ferrix.svg" width="200">
  <h1>Ferrix — Swiss Knife for Linux Hardware Diagnostics</h1>
  <p><b>A simple program for getting information about computer hardware and installed software.</b></p>

  [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![GitHub Release](https://img.shields.io/github/v/release/mskrasnov/ferrix?logo=github)](https://github.com/mskrasnov/ferrix/releases)
</div>

<img src="./screens/screen2.png" width="40%"> <img src="./screens/screen5.png" width="40%">

## What is Ferrix?

Ferrix is a Rust-crate and program for obtaining information about computer hardware and software. It is designed to work in modern Linux OS distributions.

## Functions (`ferrix-lib` crate)

1. Get information about:
    - [X] CPU;
    - [X] RAM;
    - [ ] Storage;
    - [X] BIOS and PC Motherboard;
    - [ ] Laptop battery;
    - [X] Installed Linux distribution;
    - [ ] Desktop environment;
    - [X] systemd services;
    - [ ] flatpak packages;
2. Convert collected data into:
    - [X] JSON;
    - [X] XML;
<!-- 3. Reset GNOME Desktop settings; -->

## Build & Install

```bash
git clone https://github.com/mskrasnov/Ferrix
cd Ferrix

make build
```

If you use Debian, perform:

```bash
make deb
```

And install `deb`-package:

```bash
sudo dpkg -i ./target/debian/ferrix-app_${VERSION}-${BUILD_NUM}_${ARCH}.deb
```

If you use other Linux system, perform:

```bash
make run # to run Ferrix...
# ... or
make install # to install Ferrix.
# Perform:
make uninstall # to uninstall Ferrix from your system.
```

## Technology stack

- **OS:** Linux with `glibc`, `dbus` and `systemd`;
- **Programming language:** Rust 1.88+ (2024 edition);
- **GUI:** [`iced`](https://iced.rs);
- **Hardware:** modern PC or laptop;

## ❤️ Support Ferrix

Developing Ferrix takes time and passion. If you find it useful, please consider supporting its development:

- **Star ⭐ this repo!** It helps others discover Ferrix;
- **Write comments, questions, bug reports, or suggestions** for new functionality in [issues](https://github.com/mskrasnov/Ferrix/issues/new).
- If you are from Russia, **send me a donation 💰** to the card: `2202 2062 5233 5406` (Sberbank) or support me in [Boosty](https://boosty.to/mskrasnov). This will help me keep my enthusiasm alive, as well as pay my mobile internet bills so that I can continue working on Ferrix.
- **Spread the world!** Tell friends, post on forums.

## License

Ferrix is free and open-source software distributed under the **GNU General Public License v3.0**. See [LICENSE](LICENSE) file for details.
