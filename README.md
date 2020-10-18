# dota2_webapi_bindings

[![Build Status](https://travis-ci.com/sn99/dota2_webapi_bindings.svg?branch=master)](https://travis-ci.com/sn99/dota2_webapi_bindings)
[![Crates.io Download](https://img.shields.io/crates/d/dota2_webapi_bindings.svg)](https://crates.io/crates/dota2_webapi_bindings)
[![crate](https://img.shields.io/crates/v/dota2_webapi_bindings.svg)](https://crates.io/crates/dota2_webapi_bindings)
[![Documentation](https://docs.rs/dota2_webapi_bindings/badge.svg)](https://docs.rs/dota2_webapi_bindings) 

Dota 2 webapi bindings for rust

You can find the official(outdated) documentation [here](https://wiki.teamfortress.com/wiki/WebAPI#Dota_2)

I am currently using [xpaw](https://steamapi.xpaw.me/#) to get a list of APIs

### How to Use :

In `Cargo.toml` :
```toml
[dependencies]
dota2_webapi_bindings = "*"
```
In `main.rs` :

```rust
//main.rs
use dota2_webapi_bindings::Dota2Api;
static DOTA2_KEY: &str = "0123456789"; //example token

fn main() {
   let mut dota = Dota2Api::new(String::from(DOTA2_KEY));
   // we use `set` to configure the URL first
   dota.set_heroes().itemized_only(true).language("zh_zh");
   // you can also write the above as just `dota.set_heroes();` or `dota.set_heroes().itemized_only(true);`
   // or just `dota.set_heroes().language("zh_zh");` or `dota.set_heroes().language("zh_zh").itemized_only(true);`
   // our builder like function takes care of optional parameters
 
   // and finally `get` to retrieve our struct
   let data = dota.get_heroes().expect("something went wrong, ez mid");
 }
 ```

The webapi terms are same as official except they are all in lowercase, Eg : `GetGameItems` is now `get_game_items()`.

##### Available calls :
* IEconDOTA2_570
    * GetGameItems
    * GetHeroes
    * GetRarities
    * GetTournamentPrizePool
* IDOTA2Match_205790
    * GetLeagueListing
* IDOTA2Match_570
    * GetLiveLeagueGames
    * GetTopLiveGame

See [documentation](https://docs.rs/dota2_webapi_bindings) for more.

**Note:** Try using `language()` with everything, just put in any string, it seems like its gives better readable name
and description for some reason

## License

Licensed under

 * MIT license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/MIT)
