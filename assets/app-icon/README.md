# VideoBox app icon

**Mark:** the Turtle Box turtle, a single-ink smiling turtle, in light ink on a dark tile.

## Palette

Exactly two colours, taken from the High Turtle design system:

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#17181a` | the full-bleed tile behind the turtle |
| On-ink | `#f6f6f5` | the turtle |

## Geometry and style

- A 512-unit viewBox tile, rounded square `rx=112`, ink full bleed (no frame or border).
- The turtle mark (426 x 225 units) is scaled to 372 units wide and centred on the tile.
- macOS renders (`videobox.icns`, `videobox-macos-512.png`) put the tile in Apple's grid (824 of
  1024 px, transparent margin). Windows, Linux and the web use the full-bleed tile.

## Provenance

The turtle mark comes from the High Turtle logo (see `docs/brand/`). Every file here is generated
from `videobox.svg`. Licence: [LICENSE.txt](LICENSE.txt) (Turtle Box brand mark, see
`docs/brand/LICENSE-brand.txt`).

## Files

| File | What |
|---|---|
| `videobox.svg` | canonical artwork |
| `videobox-small.svg` | same artwork (kept as a separate file because `packaging/icons.sh` copies it to the Linux theme) |
| `videobox-1024.png` | 1024 px render |
| `videobox-macos-512.png` | runtime Dock icon on macOS (embedded by `apps/videobox/src/main.rs`) |
| `videobox.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `videobox.ico` | Windows icon, 16–256 px (embedded in `videobox.exe` by `apps/videobox/build.rs`) |
| `hicolor/<size>/apps/studio.turtlebox.videobox.png`, `hicolor/scalable/…svg` | Linux icon theme, used by `packaging/linux/studio.turtlebox.videobox.desktop` |

The web app's `apps/videobox-web/web/favicon.png` (128 px) comes from the same artwork.

## How the icon reaches each OS

- **macOS:** the window icon set at startup (`ViewportBuilder::with_icon`) is the Dock and app-switcher
  icon when the app runs unbundled. A future `.app` bundle points `CFBundleIconFile` at `videobox.icns`.
- **Windows:** `build.rs` embeds `videobox.ico` as the exe's resource icon (Explorer, Start menu,
  pinned taskbar), and the runtime icon covers the window, taskbar and Alt-Tab.
- **Linux:** the runtime icon plus `with_app_id("studio.turtlebox.videobox")`, which Wayland matches to
  the `.desktop` file. Install the `hicolor/` tree under `/usr/share/icons/hicolor/` and the desktop
  file under `/usr/share/applications/`.

## Regenerate

`packaging/icons.sh` (needs `resvg`; `iconutil` on macOS for the `.icns`; the `.ico` is packed by
`cargo xtask ico`). To change the artwork, replace `videobox.svg` and `videobox-small.svg`, then run
the script. Every derived file needs its `.attribution` sidecar and a row in `ATTRIBUTION.md`.
