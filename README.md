# RustBSD

Exactly what the title says: OpenBSD, with some parts rewritten (and some written from scratch) in Rust.

**Disclaimer:** It's just for fun, obviously it's not a great idea to [take a stable set of commonly used userspace tools are rewrite it in a new programming language with no evident benefit](https://discourse.ubuntu.com/t/an-update-on-rust-coreutils/80773)...

**Disclaimer:** This is fully untested software, and we expect for it not to work: nevertheless, open an issue if it doesn't work correctly (or better yet, try to fix it and send a PR)

## Why is it interesting?

- *Fun*: Mostly, just beacuse it's a fun experiment.

- *Safe*: Some high-safety components such as `init` and `doas` benefit from an high-quality Rust reimplementation.

- *Updated*:
  - rustBSD is synced daily with the upstream OpenBSD source code for all non-Rust components.
  - rustBSD is compatible with OpenBSD's ports and xenocara package repositories.

- *Rolling-release*: Everytime a major feature is reached in any of the packages developed by rustBSD and the entire system is tested, a new stable release is published.

- *Declarative*: *fdecs* is a tool for flexible declarative system for rustBSD, managing packages, configurations, scripts, daemons and databases. *(not yet implemented)*

- *Data-driven*:
  - *Datad* is our data layer daemon, built on top of SQLite, inspired by the IBM's AS/400 design. *(not yet implemented)*
  - *Kvdb* is our in-memory lightweight key-value database, compatible with JSON and inspired by Redis. *(not yet implemented)*

- *Reliable*:
  - Built on top of OpenBSD, famous for its reliability
  - *supervise* is our metrics-aware service supervisor, capable of managing safe reboots of critical processes *(not yet implemented)*

- *Scalable*:
  - *terascale* is a Linux server software for spawning and managing multiple rustBSD VMs *(not yet implemented)*
  - *polyserverd* is a lightweight daemon for managing multiple rustBSD nodes, integrated with `terascale` *(not yet implemented)*
  - *polyclientd* is a lightweight daemon for rustBSD nodes that allows to communicate with `polyserverd` *(not yet implemented)*
  - *lbalanced* is a lightweight load balancer, integrated with `polyserverd` for splitting packets across multiple rustBSD nodes *(not yet implemented)*

## How to build and install

- Install OpenBSD (most likely in a VM)
- Install the Rust toolchain
- Clone this repository into `/usr/src`
- Run `make obj && make build`
- Run `cd /usr/src && sysmerge`
- Run `cd /dev && ./MAKEDEV all`
- Run `reboot` (just to manage running processes)
- You are now running on the latest version of RustBSD!

## How to create disk images

- (Follow `How to build and install`)
- Run `export RELDIR=your-releasedir`
- Run `cd /usr/src/distrib/$(machine)/iso && make`
- Run `make install`

## How to run the testing suite
- Run `./run-tests.csh` or `csh run-tests.csh`

---

## What was rewritten?

- `echo`
- `chroot`
- `cat`

## Will be rewritten next...

- `cp`
- `mv`
- `dd`
- `kill`
- `ls`
- `rm`
- `rmdir`
- `sleep`
- `ps`

## To be rewritten in the future...

- `md5`
- `sha*`
- `sync`
- `tar`

## Major future rewrites...

- `init`
- `doas`

## Original tools incomings...

- `datad`: sqlite-based background data layer
- `kvdb`: in-memory key-value database
- `fdecs`: flexible declarative system for rustBSD installations
- `supervise`: reliable service supervisor
- `polyclientd`: platform for managing multiple rustBSD nodes; *polyclientd* runs on the nodes
- `polyserverd`: platform for managing multiple rustBSD nodes; *polyserverd* runs on the managers
- `lbalanced`: lightweight load balancer daemon
- `terascale`: Linux tool for managing rustBSD VMs


---

## How can I contribute?

Just take any userspace program that you are interested in, implement it in Rust and send a PR.

(Note: before writing a userspace program, read `bin/echo` as it's the simplest example)

## How important is backwards compatibility?

Not too much:

- As long as amd64 is supported, support for other architectures can be broken (with `TODO` notes for later support of other architectures)
- As long as it is widely unused, particular CLI flags and features can be left out in the reimplementation process
- If there is a better way to do it, that way should be followed

## Can we add new userspace programs?

Yes, but only if we think that they can be widely used on most RustBSD installations.

## What is PuffyRS?

PuffyRS (found at `puffyrs/`) is a Rust library designed to help us write better userspace tooling using idiomatic Rust.

## How to add a Rust project to the userland

1. Create a directory for your program under the appropriate location (e.g. `bin/myprog/` for a utility that belongs in `/bin`).

2. Initialize a Rust project inside it:
   ```
   cargo init --name myprog bin/myprog
   ```

3. Ensure the `Cargo.toml` declares a binary matching your program name:
   ```toml
   [package]
   name = "myprogrs"
   version = "0.1.0"
   edition = "2021"

   [[bin]]
   name = "myprog"
   path = "src/main.rs"
   ```

4. Create a `Makefile` in your program's directory that invokes `cargo` and integrates with the BSD build system:
   ```makefile
   PROG=	myprog
   CLEANFILES+=	target

   ${PROG}:
   	cargo build --release
   	ln -sf target/release/${PROG} ${PROG}

   .include <bsd.prog.mk>
   ```

5. Add your program's directory name to the `SUBDIR` list in the parent `Makefile` (e.g. `bin/Makefile`), so it gets built alongside the rest of the userland.

6. Rebuild and install as usual:
   ```
   make obj && make build
   cd /usr/src && sysmerge
   ```
