# botw-flight-director
A WIP project that does various live video analysis for speedrunning Breath of the Wild. It is installed as an OBS plugin and reads directly from the capture card.

## Features
- Quest Tracker (For All Quests, WIP)
- Minimap Direction Assist (WIP)
- Korok/Shrine Counter (WIP)
- Bloodmoon Time Estimater (WIP)

## Install
Not Available Yet

## Development
### Tools Needed
Make sure these tools are installed before running any scripts from the project
| Binary | Description | Install | Comment |
| ------ | ----------- | ------- | ------- |
| `rustup`, `rustc`, `cargo` | Rust Toolchain | [rust-lang.org](https://www.rust-lang.org/tools/install) |
| `task` | For running scripts | [taskfile.dev](https://taskfile.dev/#/installation) |
| `cmake` | C Build Tool | [cmake.org](https://cmake.org/download/) | OBS Plugin Dev Only |
| `python` | Python Interpreter | [python.org](https://www.python.org/downloads/) | OBS Plugin Dev Only |
| `vcpkg` | C++ Package Manager | [microsoft/vcpkg](https://github.com/microsoft/vcpkg) | OBS Plugin Dev + Windows Only |
| `pwsh` | PowerShell 7 | [microsoft.com](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows?view=powershell-7.4) | OBS Plugin Dev + Windows Only |
| `bun` | JavaScript Tooling | [bun.sh](https://bun.sh/docs/installation) | Client Dev Only |

### Dev Setup - OBS Plugin Dev (Windows)
Run `task tools:obs` to install the necessary tools for OBS Plugin Development on Windows
This will install the tools using `cargo install`, as well as `llvm` and `opencv` through `vcpkg`.

After that, you need to add `clang.exe` to path (located at `$VCPKG_ROOT\installed\x64-windows\tools\llvm` on windows)

### Dev Setup - OBS Plugin Dev (Linux)
(Not Available Yet) please refer to https://github.com/twistedfall/opencv-rust to see how to setup OpenCV

### Dev Setup - Client Dev
Run `task tools:client` to install the necessary tools for Client Development. This will install the tools using `cargo install`, as well as running `bun install`

### Building
TODO