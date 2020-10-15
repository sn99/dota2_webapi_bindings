#[derive(Deserialize, Debug)]
pub struct GetHeroesResult {
    pub(crate) result: GetHeroes,
}

#[derive(Deserialize, Debug)]
pub struct GetHeroes {
    heroes: Vec<Hero>,
    count: usize,
}

#[derive(Deserialize, Debug)]
struct Hero {
    name: String,
    id: usize,
}
