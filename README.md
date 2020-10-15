# dota2_webapi_bindings

[![Build Status](https://travis-ci.com/sn99/dota2_webapi_bindings.svg?branch=main)](https://travis-ci.com/sn99/dota2_webapi_bindings)
[![Crates.io Download](https://img.shields.io/crates/d/dota2_webapi_bindings.svg)](https://crates.io/crates/dota2_webapi_bindings)
[![crate](https://img.shields.io/crates/v/dota2_webapi_bindings.svg)](https://crates.io/crates/dota2_webapi_bindings)
[![Documentation](https://docs.rs/dota2_webapi_bindings/badge.svg)](https://docs.rs/dota2_webapi_bindings) 

Dota 2 webapi bindings for rust

You can find the official documentation [here](https://wiki.teamfortress.com/wiki/WebAPI#Dota_2)

### How to Use :

In `Cargo.toml`
```toml
[dependencies]
dota2_webapi_bindings = "*"
```

The webapi terms are same as official except they are all in lowercase, Eg : `GetGameItems` is now `get_game_items()`.

See [documentation](https://docs.rs/dota2_webapi_bindings) for more.

## License

Licensed under

 * MIT license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/MIT)