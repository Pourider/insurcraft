# InsurCraft app icon

<img src="insurcraft-small.svg" alt="InsurCraft app icon: a white page with a navy I and three text lines on a blue tile" width="128">

**Design:** a white document page with a folded corner, a navy "I" (for Insur / InsurCraft) and three
text lines, the last one in orange, on a blue gradient tile.

**Palette:**

| Colour | Hex | Used for |
|---|---|---|
| Blue (top) | `#14658F` | tile gradient |
| Navy | `#0A3550` | tile gradient, the "I" |
| Light blue | `#A9D8EE` | folded corner |
| Grey blue | `#9BB3C2` | text lines |
| Orange | `#F29A2E` | accent line |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`. Windows and Linux icons use the
full-bleed tile. macOS icons put it on Apple's grid (an 824 px body centred on a transparent 1024 px
canvas).

**Provenance:** original vector artwork made for InsurCraft. Licence: [LICENSE.txt](LICENSE.txt)
(`MIT OR Apache-2.0`, like the repo).

## Files

| File | What it is |
|---|---|
| `insurcraft.svg` | the master vector; every PNG, `.ico` and `.icns` is rendered from it |
| `insurcraft-small.svg` | copy of the master, used in the app (home screen, About) and this README |
| `insurcraft-1024.png` | 1024 px on Apple's grid; also the runtime Dock icon on macOS |
| `insurcraft.icns` | macOS icon (16–1024 px) |
| `insurcraft.ico` | Windows icon (16–256 px), embedded in `insurcraft.exe` by `apps/insurcraft/build.rs` |
| `hicolor/<n>x<n>/apps/pe.insur.insurcraft.png` | Linux hicolor theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/pe.insur.insurcraft.svg` | Linux scalable icon (copy of the master) |

Where it shows: `apps/insurcraft/src/main.rs` sets the window icon (Dock, taskbar, Alt-Tab, launcher) and the
Wayland app id `pe.insur.insurcraft`; `packaging/linux/pe.insur.insurcraft.desktop` names the
hicolor icon.

## Regenerate

```sh
packaging/icons.sh        # needs resvg and python3; iconutil (macOS) for the .icns
cargo xtask assets        # then update the sha256 values in ATTRIBUTION.toml and run with --write
```
