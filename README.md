# RustBSD

Exactly what the title says: OpenBSD, with some parts rewritten in Rust.

**Disclaimer:** It's just for fun, obviously it's not a great idea to [take a stable set of commonly used userspace tools are rewrite it in a new programming language with no evident benefit](https://discourse.ubuntu.com/t/an-update-on-rust-coreutils/80773)...

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

---

## What was rewritten?
- `echo`

## How can I contribute?

Just take any userspace program that you are interested in, implement it in Rust and send a PR.

## How important is backwards compatibility?

Not too much:

- As long as amd64 is supported, support for other architectures can be broken (with `TODO` notes for later support of other architectures)
- As long as it is widely unused, particular CLI flags and features can be left out in the reimplementation process
- If there is a better way to do it, that way should be followed

## Can we add new userspace programs?

Yes, but only if we think that they can be widely used on most RustBSD installations.
