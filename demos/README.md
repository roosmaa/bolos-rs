# bolos-rs demo applications

This folder contains the following applications that demonstrate usage of _bolos-rs_:

- [hello-world](https://github.com/roosmaa/bolos-rs/tree/master/demos/hello-world) - The most basic user interface rendering.
- [ui-patterns](https://github.com/roosmaa/bolos-rs/tree/master/demos/ui-patterns) - Various UI patterns that Ledger apps usually use (menus, confirmation screen).

## Setting up the build environment

To build and install the applications to your Ledger Nano S, you need to install the following software: Rust, GCC ARM cross-compiler, Ledger Python tools.

### Step 1: GCC ARM cross-compiler

We need to install the ARM cross-compiler toolchain to gain access to the linker which the Rust compiler relies upon. Most distributions have a ready to use package available, look for "arm-non-eabi-gcc" in your package manager.

If you're running Fedora, you can install it using the following command:

```
$ dnf install arm-none-eabi-gcc-cs
```

### Step 2: Ledger Python tools

You need to have Python 3 installed on your system. It is advisable to install the Ledger tools inside a python virtual environment. These steps should work for most Linux systems:

```bash
$ python3 -m venv --prompt bolos ~/.bolos-virtualenv
$ source ~/.bolos-virtualenv/bin/activate
(bolos) $ pip install ledgerblue
```

At the time of writing, the ledgerblue (0.1.19) package needs some manual patching to make it work on Python 3 reliably:

```
(bolos) $ cd ~/.bolos-virtualenv/lib/python3*/
(bolos) $ patch -p0 < path/to/bolos-rs/demos/ledgerblue-0.1.19.patch
```

### Step 3: Rust

Follow the official Rust install instructions [here](https://www.rust-lang.org/en-US/install.html). Afterwards you should have `rustup` command at your disposal. You need to install the nightly toolchain and the correct target for Ledger hardware as follows:

```
(bolos) $ rustup install nightly
(bolos) $ cd path/to/bolos-rs/demos/
(bolos) $ rustup override set nightly
(bolos) $ rustup target add thumbv6m-none-eabi
```

## Running the demos

Each demo has a _Makefile_, that runs the correct commands to build and upload the app to Ledger Nano S. If you installed the build environment correctly, it's just a matter of running the following in the desired demo folder:

```
(bolos) $ make load
```

If you wish to delete the demo app from your device, run the following:

```
(bolos) $ make delete
```