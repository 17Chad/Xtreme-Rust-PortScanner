**XTREME Rust Port Scanner**

Made with the intention to be as small and fast as possible. 
Intended for embedded devices where storage is a concern and speed is needed to get on/off target quickly.  

**How to run or build** 
```
"cargo run -- -a 192.168.1.1 -p 22,80,443,8080" to run normally.
"cargo run -- -a 192.168.1.1 -p 1-1024" 
"cargp run -- -a 192.168.1.1 -p 22 
"cargo build --release" to build and optimize size of the executable.
```

**To cross-compile for ARM, ARM64, MIPS (both big-endian and little-endian), TILE, PowerPC, etc you can use Rust's built-in support for cross-compilation targets. Here are the general steps to cross-compile for these architectures:**

```
rustup target add armv7--linux-gnueabihf
rustup target add aarch64--linux-gnu
rustup target add mips--linux-gnu
rustup target add mipsel--linux-gnu
rustup target add tilegx--linux-gnu
rustup target add powerpc64le--linux-gnu
```

**Make sure your build environment is setup properly by makign sure your environmental variables CC and CXX are pointed at your compilers correctly. You can do something like 'which arm-linux-gnueabihf-gcc' to find them in your path.**

```
export CC=/usr/bin/arm-linux-gnueabihf-gcc
export CXX=/usr/bin/arm-linux-gnueabihf-g++
```

**Now you can cross-compile your Rust project for each target architecture using cargo and specifying the target:**

```
cargo build --target=armv7-linux-gnueabihf --release
cargo build --target=aarch64-linux-gnu --release
cargo build --target=mips-linux-gnu --release
cargo build --target=mipsel-linux-gnu --release
cargo build --target=tilegx-linux-gnu --release
cargo build --target=powerpc64le-linux-gnu --release
Now the executables should be in each target specific directory. Always test first before use. 
```
