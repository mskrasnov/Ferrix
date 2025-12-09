# ferrix-lib

> **NOTE 1:** visit our [GitHub repository](https://github.com/mskrasnov/ferrix) to get more information about this crate.
>
> **NOTE 2:** this crate is a part of [ferrix-app](https://crates.io/crates/ferrix-app) crate.

Crate to get information about PC's hardware and software. Only for Linux. Some features are requires `d-bus`, `systemd` and `flatpak`. Supported features: get information about:

- CPU (`/proc/cpuinfo`);
- RAM (`/proc/meminfo`) and swaps (`/proc/swaps`);
- Linux kernel information (version, architecture, cmdline);
- Users and groups;
- Environment variables for current user;
- `systemd` services;
- Infrormation from DMI tables (BIOS, motherboard, chassis/enclosure, processor, RAM);
- Information from EDID (basic info);
- Supported resolutions for monitor;

TODO:

- [ ] Get more info from EDID;
- [ ] Get information about installed software (`flatpak`, `deb`, `rpm`);
- [X] Get information about notebook battery;
- [ ] Get information about audio;
- [ ] Get information about GUI (desktop environment, session type (Wayland or X.org), etc.);
- [ ] Backup and reset `gsettings` settins;

## License

`ferrix-lib` is distributed under the GNU GPL v3 license.
