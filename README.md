---
author: jturnerusa, epenguin
title: RISCV Bare Metal Hello World
---

# Prerequisites

### Crossdev

You will need crossdev to compile the rust-std toolchain. Set up
crossdev here: <https://wiki.gentoo.org/wiki/Crossdev>

### Installing the toolchain

We need to install the toolchain *with* c++, so let's create the
package.use file by asking crossdev to initialize the target first:

``` bash
crossdev --init-target riscv64-unknown-none-elf
```

Make sure we have the cxx use flag enabled:

``` bash
vim /etc/portage/package.use/cross-riscv64-unknown-none-elf
:%s/-cxx//g
:wq
```

Next we will compile the riscv64-unknown-none-elf stage 4. This will
compile the C++ compiler as well.

``` bash
crossdev -s4 -t riscv64-unknown-none-elf
```

Next we need to install the rust toolchain for the riscv64-unknown-none
target:

``` bash
mkdir -p /var/db/repos/crossdev/cross-riscv64-unknown-none-elf/rust-std
cp rust-std-1.77.1.ebuild /var/db/repos/crossdev/cross-riscv64-unknown-none-elf/rust-std
cd /var/db/repos/crossdev/cross-riscv64-unknown-none-elf/rust-std
ebuild rust-std-1.77.1.ebuild manifest
```

We can't compile the cross compiler just yet. We need to first compile
our native toolchain at version 1.77.1 with some use flags:

``` bash
echo "dev-lang/rust llvm_targets_RISCV dist" > /etc/portage/package.use/dev-lang-rust.use
echo "
>=cross-riscv64-unknown-none-elf/rust-std-1.77.1::crossdev **
>=dev-lang/rust-1.77.1::gentoo ~amd64
>=virtual/rust-1.77.1::gentoo ~amd64" >> /etc/portage/package.accept_keywords/cross-rust.accept
```

Now we must recompile our native rust toolchain:

``` bash
emerge -a virtual/rust dev-lang/rust
```

And finally we can compile the cross compiler!

``` bash
emerge -a cross-riscv64-unknown-none-elf/rust-std-1.77.1
```
