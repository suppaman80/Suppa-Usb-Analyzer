# Suppa-Usb-Analyzer

![Suppa-Usb-Analyzer running on Linux with connected USB devices](docs/suppa-usb-analyzer-screenshot.png)

A native Rust desktop prototype exploring a lightweight alternative to the
React/Tauri interface. It does not modify the `Gestore_USB` project.

The application is built with `egui`/`eframe` and does not use a WebView.
Scanning uses `sysfs` and `lsusb` to display USB devices, `ttyUSB`/`ttyACM`
ports, VID:PID identifiers, and recognized serial bridges.

The application version is defined in `Cargo.toml` and displayed in the About
window. Release tags use the matching `v` prefix (for example, `0.1.0` becomes
`v0.1.0`).

## Run

```sh
cargo run
```

To build and measure the optimized executable:

```sh
cargo build --release
du -h target/release/suppa-usb-analyzer
```

## AppImage

```sh
./build-appimage.sh
```

Builds are saved with unique filenames in `artifacts/appimage/`.

## Publish a GitHub Release

The `Build and publish AppImage` workflow can be started manually from the
GitHub Actions tab. Enter a tag matching the package version in `Cargo.toml`
(for example, `v0.1.0`). The workflow runs the test suite, builds an x86_64
AppImage, creates the GitHub Release, and attaches the package.

## Suppaworld Website

The static landing page is in `docs/` and is published with GitHub Actions.
After uploading this project to the `Suppaman80/Suppa-Usb-Analyzer` repository,
set GitHub Pages to **GitHub Actions**:

`https://suppaman80.github.io/Suppa-Usb-Analyzer/`

The workflow runs when website files are updated, or it can be started manually
from the Actions tab. To preview the site locally:

```sh
python3 -m http.server 8000 --directory docs
```

## Notes

- Scanning runs in the background so the interface remains responsive.
- Search, serial-port filtering, manual refresh, and a device summary are included.
- This is a functional prototype; it does not yet include every refinement or
  distribution workflow from the Tauri application.

## License

The original code and assets in this project are distributed under the terms of
the GNU General Public License, version 3 or later (GPL-3.0-or-later). See
`LICENSE`.

When distributing a binary build, also make the corresponding source code and
the scripts required to generate it available.
