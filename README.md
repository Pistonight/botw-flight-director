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
### Requirements

You need the following tools:
- A Rust Toolchain
- (Windows Only)`vcpkg` in your path (https://github.com/microsoft/vcpkg)
- `cmake` in your path (https://cmake.org/download/)
- `task` in your path (https://taskfile.dev/#/installation)

First run `task install` to setup the tools.

If you are on Windows, run `task win:vcpkg` to install `llvm` and `opencv` through `vcpkg`. After that, you need to add `clang.exe` to path (located at `$VCPKG_ROOT\installed\x64-windows\tools\llvm` on windows)

For other platforms, please refer to https://github.com/twistedfall/opencv-rust to see how to setup OpenCV

### Building