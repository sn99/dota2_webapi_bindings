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
    localized_name: Option<String>,
}

//==============================================================================
#[derive(Deserialize, Debug)]
pub struct GetGameItemsResult {
    pub(crate) result: GetGameItems,
}

#[derive(Deserialize, Debug)]
pub struct GetGameItems {
    items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    id: usize,
    name: String,
    cost: usize,
    secret_shop: usize,
    side_shop: usize,
    recipe: usize,
    localized_name: Option<String>,
}
