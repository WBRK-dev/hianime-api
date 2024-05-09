# HiAnime API

An api that scrapes hianime.to (formerly aniwatch.to or zoro.to).

## Contents
- [Installation](#installation)
- [API Docs](#api-docs)
  - [GET Home Page](#get-home-page)

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

## API Docs
### `GET` Home Page
```
http://127.0.0.1/home
```
Response
```javascript
{
    spotlight: [
        {
            rank: number,
            id: string,
            title: string,
            jtitle: string,
            discription: string,
            poster: string,
            details: string[],
            episodes: {
                sub: number,
                dub: number
            },
        }
        { ... }
    ],
    trending: [
        {
            rank: number,
            id: string,
            title: string,
            jtitle: string,
            poster: string,
        }
        { ... }
    ],
    latest_episodes: [
        {
            id: string,
            title: string,
            jtitle: string,
            discription: string,
            poster: string,
            details: {
                duration: string,
                type: string,
                rating: string
            },
            episodes: {
                sub: number,
                dub: number
            },
        }
        { ... }
    ],
    top_upcoming: [
        {
            id: string,
            title: string,
            jtitle: string,
            poster: string,
            details: {
                duration: string,
                type: string,
                rating: string
            },
            episodes: {
                sub: number,
                dub: number
            },
        }
        { ... }
    ]
}
```