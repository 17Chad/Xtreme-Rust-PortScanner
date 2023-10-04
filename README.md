**XTREME Rust Port Scanner**

Made with the intention to be as small and fast as possible. 
Intended for embedded devices where storage is a concern and speed is needed to get on/off target quickly.  

**How to run or build** 
```
"cargo run -- -a 192.168.1.1 -p 22,80,443,8080" to run normally.
"cargo run -- -a 192.168.1.1 -p 1-1024" 
"cargp run -- -a 192.168.1.1 -p 22

"cargo build --release" to build and optimize size of the executable.

Usage: -a Address -p Port

Available options:
    -a, --address <Address>  The address that you want to scan. Must be a valid IPv4 address.
    -p, --ports <Port>  The port(s) to scan. Specify a single port (e.g., 80), a list of ports (e.g.,
                        80,22,443,8080), or a range of ports (e.g., 1-1024).
    -h, --help          Prints help information

```

** To cross-compile in Rust, please see this guide: https://github.com/rust-cross/rust-musl-cross/tree/main **
