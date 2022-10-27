# Rust Network Monitor

## Installation

https://www.rust-lang.org/learn/get-started

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    Rustup metadata and toolchains will be installed into the Rustup
    home directory, located at:

      /home/ant/.rustup

    This can be modified with the RUSTUP_HOME environment variable.

    The Cargo home directory is located at:

      /home/ant/.cargo

    This can be modified with the CARGO_HOME environment variable.

    The cargo, rustc, rustup and other commands will be added to
    Cargo's bin directory, located at:

      /home/ant/.cargo/bin

    This path will then be added to your PATH environment variable by
    modifying the profile files located at:

      /home/ant/.profile
      /home/ant/.bashrc

    You can uninstall at any time with rustup self uninstall and
    these changes will be reverted.

    Current installation options:


       default host triple: x86_64-unknown-linux-gnu
         default toolchain: stable (default)
                   profile: default
      modify PATH variable: yes

    ...
      stable-x86_64-unknown-linux-gnu installed - rustc 1.64.0 (a55dd71d5 2022-09-19)

restart shell

    cargo --version
    cargo 1.64.0 (387270bc7 2022-09-16)


Rust updates very frequently... Get the latest version of Rust by running `rustup update`.


- build your project with `cargo build`
- run your project with cargo run`
- test your project with `cargo test`
- build documentation for your project with `cargo doc`
- publish a library to crates.io with `cargo publish`

## Creation

    cargo new network-monitor
    cd network-monitor
    cargo run

## Debugging with VS Code

- installed extension CodeLLDB


## Links

- cargo manifest (Cargo.toml file in root of project) https://doc.rust-lang.org/cargo/reference/manifest.html
- crates: https://crates.io/crates/
- language reference: https://doc.rust-lang.org/rust-by-example/flow_control/for.html
- language nursery: https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
- rust book: https://doc.rust-lang.org/book/
- cargo book: https://doc.rust-lang.org/cargo/

