# 3_softeng_zh_2023

This software project was implemented for the MSE TSM_SoftwEng lecture 2023.
It consists of a microcontroller recording temperature data, which gets sent to a web service.
The data can be viewed with a web-based GUI.

The architecture documentation is deployed on [GitLab Pages](https://hslu.pages.switch.ch/edu/bachelor-engineering-and-architecture/tsm_softweng/tsm_softweng_aut23/3_softeng_zh_2023/3_softeng_zh_2023/overview.html).

## Contributing

Getting setup for development should be as easy as running `./dev/setup.sh`.
If the script doesn't work, read it to figure out what it was supposed to do.
If mostly installs a bunch of system dependencies and development tools.

Although the release build is a single static executable, there are several processes running for development.
This provides auto reloading of backend, frontend, documentation and diagrams whenever something changes.
Run `just zellij` to start everything.
It's recommended to run `zellij` in a standalone terminal emulator to avoid keyboard shortcut conflicts.
You can also figure out how to run everything manually by reading `./dev/zellij.kdl` and the `justfile`.

You might want to quickly familiarize yourself with the following dev tools:
- [just], a simple command runner
- [zellij], a terminal workspace / multiplexer

[just]: https://github.com/casey/just?tab=readme-ov-file#just
[zellij]: https://zellij.dev/about/

## Installation

There are two straight-forward ways to install / run a release build of the application.
There are prebuilt binaries to download for `x86_64-unknown-linux-gnu`.
There is also a Containerfile, but no published image, so you need to build it yourself.

### Download the prebuilt binary

Navigate [here][main-pipelines], where all pipelines for the main branch are listed.
Pick the latest one, click on the build job and browse the artifacts.
The binary should be there for you to download.

[main-pipelines]: https://gitlab.switch.ch/hslu/edu/bachelor-engineering-and-architecture/tsm_softweng/tsm_softweng_aut23/3_softeng_zh_2023/3_softeng_zh_2023/-/pipelines?page=1&scope=all&ref=main

### Build a container using Containerfile

Clone the repository and do a podman or docker build.
Next, run it. The port inside the container is 4000.
Example:

```sh
podman build -t softw-eng .
podman run -it --rm --init -p 4000:4000 softw-eng
```

## Usage

Once you got a hold of a prebuilt binary, everything's in there.
Run the executable to start the server (`--help` works as expected).
Navigate to `localhost:4000` (or the port you configured) to use the GUI.
You can add and delete temperature measurements for demonstration, as well as navigate to the embedded documentation.
The REST API is not documented, you have to check the source if you want to make API requests without the GUI.

## License

The content of this repository is dedicated to the public domain via the [Unlicense](https://spdx.org/licenses/Unlicense.html).
