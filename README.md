# RustBSD

Exactly what the title says: OpenBSD, with some parts rewritten in Rust.

**Disclaimer:** It's just for fun, obvioulsy it's not a great idea to [take a stable set of commonly used userspace tools are rewrite it in a new programming language with no evident benefit](https://discourse.ubuntu.com/t/an-update-on-rust-coreutils/80773)...

**Disclaimer:** This is fully untested software, and we expect for it not to work: nevertheless, open an issue if it doesn't work correctly (or better yet, try to fix it and send a PR)

## How to build 

- Install OpenBSD (most likely in a VM)
- Install the Rust toolchain
- Clone this repository into `/usr/src`
- Run `make obj && make build`

## How to install after building

- (Follow `How to build`)
- Run `cd /usr/src && sysmerge`
- Run `cd /dev && ./MAKEDEV all`
- Run `reboot` (just to manage running processes)
- You are now running on the latest version of RustBSD!

## How to create disk images

- (Follow `How to build`)
- Run `export RELDIR=your-releasedir`
- Run `cd /usr/src/distrib/$(machine)/iso && make`
- Run `make install`
