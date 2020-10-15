#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde_json;

pub mod dota;

use dota::*;
use hyper::status::StatusCode;
use hyper::Client;
use std::io::Read;

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
}
impl Dota2Api {
    pub fn new(key: String) -> Self {
        Dota2Api {
            http_client: Client::new(),
            key,
            ..Default::default()
        }
    }

    pub fn set_heroes(&mut self) -> &mut GetHeroesBuilder {
        self.get_heroes_builder = GetHeroesBuilder::build(&*self.key);
        &mut self.get_heroes_builder
    }

    pub fn get_heroes(&mut self) -> Result<GetHeroes, Error> {
        let response = self.get(&*self.get_heroes_builder.url.clone())?;
        let data_result: GetHeroesResult = serde_json::from_str(response.as_str())?;
        let data = data_result.result;
        Ok(data)
    }

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

    pub fn itemized_only(&mut self, param_value: bool) {
        self.url
            .push_str(&*format!("itemizedonly={}&", param_value));
    }
}
