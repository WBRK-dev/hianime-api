# HiAnime API

An api that scrapes hianime.to (formerly aniwatch.to or zoro.to).

## Contents
- [Installation](#installation)

## Installation
> [!CAUTION]
>
> The rust compiler and rust package manager `cargo` must be installed!

The only way to use this api is to build the binary from source. The first step is downloading the source code.
```
git clone https://github.com/WBRK-dev/hianime-api.git
```
After it is done downloading, change directory into hianime-api. Execute the build command.
```
cargo build -r
```
After it is done building, there should be a binary called `hianime-api` in `/target/release`.<br>
The binary could also be called `hianime-api.exe` if you are on windows.