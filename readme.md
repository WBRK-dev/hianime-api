# HiAnime API

An api that scrapes hianime.to (formerly aniwatch.to or zoro.to).

> [!IMPORTANT]
>
> 1. This project is mostly based on [aniwatch-api](https://github.com/ghoshRitesh12/aniwatch-api). It is in other words a port from typescript to rust.
> 2. This API is just an unofficial api for [hianime.to](https://hianime.to) and is in no other way officially related to the same.
> 3. The content that this api provides is not mine, nor is it hosted by me. These belong to their respective owners. This api just demonstrates how to build an api that scrapes websites and uses their content.

## Contents
- [Installation](#installation)
- [API Docs](#api-docs)
  - [GET Home Page](#get-home-page)
- [Credits](#credits)

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
    ],
    top10: [
        day: [
            {
                rank: number,
                id: string,
                title: string,
                jtitle: string,
                poster: string,
                episodes: {
                    sub: number,
                    dub: number
                }
            },
            { ... }
        ],
        week: [
            {
                rank: number,
                id: string,
                title: string,
                jtitle: string,
                poster: string,
                episodes: {
                    sub: number,
                    dub: number
                }
            },
            { ... }
        ],
        month: [
            {
                rank: number,
                id: string,
                title: string,
                jtitle: string,
                poster: string,
                episodes: {
                    sub: number,
                    dub: number
                }
            },
            { ... }
        ]
    ]
}
```

## Credits
Big thanks to [Ritesh Ghosh](https://github.com/ghoshRitesh12) for providing an [api](https://github.com/ghoshRitesh12/aniwatch-api) in typescript that this project is based on!