#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde_json;

pub mod dota;

use dota::*;
use hyper::status::StatusCode;
use hyper::Client;
use std::io::Read;

macro_rules! language {
    () => {
        pub fn language(&mut self, param_value: &str) -> &mut Self {
            self.url.push_str(&*format!("language={}&", param_value));
            self
        }
    };
}

macro_rules! set {
    ($func: ident, $builder: ident, $build: ident) => {
        pub fn $func(&mut self) -> &mut $build {
            self.$builder = $build::build(&*self.key);
            &mut self.$builder
        }
    };
}

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

#[derive(Debug, Default)]
pub struct Dota2Api {
    http_client: Client,
    pub key: String,
    get_heroes_builder: GetHeroesBuilder,
    get_game_items_builder: GetGameItemsBuilder,
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
    get!(get_heroes, GetHeroes, get_heroes_builder, GetHeroesResult);

    set!(set_game_items, get_game_items_builder, GetGameItemsBuilder);
    get!(
        get_game_items,
        GetGameItems,
        get_game_items_builder,
        GetGameItemsResult
    );

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

#[derive(Default, Debug)]
pub struct GetHeroesBuilder {
    url: String,
}

impl GetHeroesBuilder {
    fn build(key: &str) -> Self {
        Self {
            url: format!(
                "http://api.steampowered.com/IEconDOTA2_570/GetHeroes/v1/?key={}&",
                key
            ),
        }
    }

    pub fn itemized_only(&mut self, param_value: bool) -> &mut Self {
        self.url
            .push_str(&*format!("itemizedonly={}&", param_value));
        self
    }
    language!();
}

#[derive(Debug, Default)]
pub struct GetGameItemsBuilder {
    url: String,
}

impl GetGameItemsBuilder {
    fn build(key: &str) -> Self {
        Self {
            url: format!(
                "http://api.steampowered.com/IEconDOTA2_570/GetGameItems/v1/?key={}&",
                key
            ),
        }
    }
    language!();
}
