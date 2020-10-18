//! The crate serves as an bindings to the official (outdated)
//! [dota2 webapi](https://dev.dota2.com/forum/dota-2/spectating/replays/webapi/60177-things-you-should-know-before-starting?t=58317)
//! The crate has been made so you can call make calls directly and get a result back in a Struct.
//!
//! Read the full list of api(outdated) calls [here](https://wiki.teamfortress.com/wiki/WebAPI#Dota_2).
//!
//! Use [xpaw](https://steamapi.xpaw.me/#) for latest.
//!
//! The webapi terms are same as official except they are all in lowercase, Eg : `GetGameItems` is now `get_game_items()`.
//!
//! You also need a key that you can get [here](http://steamcommunity.com/dev/apikey).
//! > Originally posted by Zoid at [forum](https://dev.dota2.com/forum/dota-2/spectating/replays/webapi/60177-things-you-should-know-before-starting?t=58317) When you go to http://steamcommunity.com/dev/apikey the "domain"
//! > field is just a note. It's not actually used for anything and is just a helpful field so you can
//! > tell us what your website is. You can just put your name in for now. Once you get a key, its what
//! > uniquely identifies you when accessing our WebAPI calls.
//!
//! In your `main.rs` or anywhere you intend to use the library create a non-mutable string of
//! you token pass first to use the library, there is no calls without the token.
//! ```rust
//! //main.rs
//! use dota2_webapi_bindings::Dota2Api;
//! static DOTA2_KEY: &str = "0123456789"; //example token
//!
//! fn main() {
//!   let mut dota = Dota2Api::new(String::from(DOTA2_KEY));
//!   // we use `set` to configure the URL first
//!   dota.set_heroes().itemized_only(true).language("zh_zh");
//!   // you can also write the above as just `dota.set_heroes();` or `dota.set_heroes().itemized_only(true);`
//!   // or just `dota.set_heroes().language("zh_zh");` or `dota.set_heroes().language("zh_zh").itemized_only(true);`
//!   // our builder like function takes care of optional parameters
//!
//!   // and finally `get` to retrieve our struct
//!   let data = dota.get_heroes().expect("something went wrong, ez mid");
//! }
//!
//! ```
//!
//! ##### Available calls :
//! * IEconDOTA2_570
//!     * GetGameItems
//!     * GetHeroes
//!     * GetRarities
//!     * GetTournamentPrizePool
//! * IDOTA2Match_205790
//!     * GetLeagueListing
//! * IDOTA2Match_570
//!     * GetLiveLeagueGames
//!     * GetTopLiveGame
//!
//! **Note:** Try using `language()` with everything, just put in any string, it seems like its gives better readable name
//! and description for some reason, I have not set-up a default cause sometimes that might not be your intension.

#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde_json;

pub mod dota;

use hyper::status::StatusCode;
use hyper::Client;
use std::io::Read;

use crate::dota::{
    get_game_items::*, get_heroes::*, get_league_listing::*, get_live_league_games::*,
    get_rarities::*, get_top_live_game::*, get_tournament_prize_pool::*,
};

/// language macro for easy implementation in various builder struct
///
/// The language to retrieve results in (default is en_us) (see http://en.wikipedia.org/wiki/ISO_639-1 for
/// the language codes (first two characters) and http://en.wikipedia.org/wiki/List_of_ISO_639-1_codes for
/// the country codes (last two characters))
///
/// language (Optional) (string) : The language to provide output in.
///
/// **Note:** Try using `language()` with everything, just put in any string, it seems like its gives better readable name
/// and description for some reason
macro_rules! language {
    () => {
        pub fn language(&mut self, param_value: &str) -> &mut Self {
            self.url.push_str(&*format!("language={}&", param_value));
            self
        }
    };
}

/// A `set!` macro to get our `set` functions
macro_rules! set {
    ($func: ident, $builder: ident, $build: ident) => {
        pub fn $func(&mut self) -> &mut $build {
            self.$builder = $build::build(&*self.key);
            &mut self.$builder
        }
    };
}

/// A `get!` macro to get our `get` functions
macro_rules! get {
    ($func: ident, $return_type: ident, $builder: ident, $result: ident) => {
        pub fn $func(&mut self) -> Result<$return_type, Error> {
            let response = self.get(&*self.$builder.url.clone())?;
            let data_result: $result = serde_json::from_str(response.as_str())?;
            let data = data_result.result;
            Ok(data)
        }
    };
}

/// builder to reduce boilerplate
macro_rules! builder {
    ($builder: ident, $url: expr) => {
        #[derive(Debug, Default)]
        pub struct $builder {
            url: String,
        }

        impl $builder {
            fn build(key: &str) -> Self {
                Self {
                    url: format!($url, key),
                }
            }
        }
    };
}

/// different type of errors we can receive during either fetching of data or just unpacking JSON
#[derive(Debug)]
pub enum Error {
    Http(hyper::Error),
    Json(serde_json::Error),
    Forbidden(&'static str),
    Message(String),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Error {
        Error::Http(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

/// The main `Dota2Api` of you library works by saving states of all the invoked URLs (you only call the one you need)
/// language macro for easy implementation in various builder struct
///
/// The language to retrieve results in (default is en_us) (see http://en.wikipedia.org/wiki/ISO_639-1 for
/// the language codes (first two characters) and http://en.wikipedia.org/wiki/List_of_ISO_639-1_codes for
/// the country codes (last two characters))
///
/// language (Optional) (string) : The language to provide output in.
#[derive(Debug, Default)]
pub struct Dota2Api {
    http_client: Client,
    pub key: String,
    get_heroes_builder: GetHeroesBuilder,
    get_game_items_builder: GetGameItemsBuilder,
    get_rarities_builder: GetRaritiesBuilder,
    get_tournament_prize_pool_builder: GetTournamentPrizePoolBuilder,
    get_league_listing_builder: GetLeagueListingBuilder,
    get_live_league_games_builder: GetLiveLeagueGamesBuilder,
    get_top_live_game_builder: GetTopLiveGameBuilder,
}

impl Dota2Api {
    pub fn new(key: String) -> Self {
        Dota2Api {
            http_client: Client::new(),
            key,
            ..Default::default()
        }
    }

    set!(set_heroes, get_heroes_builder, GetHeroesBuilder);
    // use `set` before `get`
    get!(get_heroes, GetHeroes, get_heroes_builder, GetHeroesResult);

    set!(set_game_items, get_game_items_builder, GetGameItemsBuilder);
    // use `set` before `get`
    get!(
        get_game_items,
        GetGameItems,
        get_game_items_builder,
        GetGameItemsResult
    );

    set!(set_rarities, get_rarities_builder, GetRaritiesBuilder);
    // use `set` before `get`
    get!(
        get_rarities,
        GetRarities,
        get_rarities_builder,
        GetRaritiesResult
    );

    set!(
        set_tournament_prize_pool,
        get_tournament_prize_pool_builder,
        GetTournamentPrizePoolBuilder
    );
    // use `set` before `get`
    get!(
        get_tournament_prize_pool,
        GetTournamentPrizePool,
        get_tournament_prize_pool_builder,
        GetTournamentPrizePoolResult
    );

    set!(
        set_league_listing,
        get_league_listing_builder,
        GetLeagueListingBuilder
    );
    // use `set` before `get`
    get!(
        get_league_listing,
        GetLeagueListing,
        get_league_listing_builder,
        GetLeagueListingResult
    );

    set!(
        set_live_league_games,
        get_live_league_games_builder,
        GetLiveLeagueGamesBuilder
    );
    // use `set` before `get`
    get!(
        get_live_league_games,
        GetLiveLeagueGames,
        get_live_league_games_builder,
        GetLiveLeagueGamesResult
    );

    set!(
        set_top_live_game,
        get_top_live_game_builder,
        GetTopLiveGameBuilder
    );
    // use `set` before `get`
    pub fn get_top_live_game(&mut self) -> Result<GetTopLiveGame, Error> {
        let response = self.get(&*self.get_top_live_game_builder.url.clone())?;
        let data_result: GetTopLiveGame = serde_json::from_str(response.as_str())?;
        let data = data_result;
        Ok(data)
    }

    /// our get function to actually get the data from the api
    fn get(&mut self, url: &str) -> Result<String, Error> {
        let mut response = self.http_client.get(url).send()?;
        let mut temp = String::new();
        if response.status == StatusCode::Forbidden {
            return Err(Error::Forbidden(
                "Access is denied. Retrying will not help. Please check your API key.",
            ));
        }
        let _ = response.read_to_string(&mut temp);
        Ok(temp)
    }
}

//==============================================================================
//IEconDOTA2_570
//==============================================================================

builder!(
    GetHeroesBuilder,
    "http://api.steampowered.com/IEconDOTA2_570/GetHeroes/v1/?key={}&"
);
impl GetHeroesBuilder {
    /// itemizedonly (Optional) (bool) : Return a list of itemized heroes only.
    pub fn itemized_only(&mut self, param_value: bool) -> &mut Self {
        self.url
            .push_str(&*format!("itemizedonly={}&", param_value));
        self
    }
    language!();
}

builder!(
    GetGameItemsBuilder,
    "http://api.steampowered.com/IEconDOTA2_570/GetGameItems/v1/?key={}&"
);
impl GetGameItemsBuilder {
    language!();
}

builder!(
    GetRaritiesBuilder,
    "http://api.steampowered.com/IEconDOTA2_570/GetRarities/v1/?key={}&"
);
impl GetRaritiesBuilder {
    language!();
}

builder!(
    GetTournamentPrizePoolBuilder,
    "http://api.steampowered.com/IEconDOTA2_570/GetTournamentPrizePool/v1/?key={}&"
);
impl GetTournamentPrizePoolBuilder {
    /// leagueid (Optional) (int) : The ID of the league to get the prize pool of.
    pub fn league_id(&mut self, param_value: usize) -> &mut Self {
        self.url.push_str(&*format!("leagueid={}&", param_value));
        self
    }
    language!();
}

//==============================================================================
//IDOTA2Match_205790
//==============================================================================

builder!(
    GetLeagueListingBuilder,
    "http://api.steampowered.com/IDOTA2Match_205790/GetLeagueListing/v1/?key={}&"
);
impl GetLeagueListingBuilder {
    language!();
}

//==============================================================================
//IDOTA2Match_570
//==============================================================================

builder!(
    GetLiveLeagueGamesBuilder,
    "http://api.steampowered.com/IDOTA2Match_570/GetLiveLeagueGames/v1/?key={}&"
);
impl GetLiveLeagueGamesBuilder {
    language!();
    /// Only show matches of the specified league id
    pub fn league_id(&mut self, param_value: usize) -> &mut Self {
        self.url.push_str(&*format!("league_id={}&", param_value));
        self
    }
    /// Only show matches of the specified match id
    pub fn match_id(&mut self, param_value: usize) -> &mut Self {
        self.url.push_str(&*format!("match_id={}&", param_value));
        self
    }
}

builder!(
    GetTopLiveGameBuilder,
    "http://api.steampowered.com/IDOTA2Match_570/GetTopLiveGame/v1/?key={}&"
);

impl GetTopLiveGameBuilder {
    language!();
    /// Which partner's games to use
    pub fn partner(&mut self, param_value: usize) -> &mut Self {
        self.url.push_str(&*format!("partner={}&", param_value));
        self
    }
}
