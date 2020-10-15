#[derive(Deserialize, Debug)]
pub struct GetHeroesResult {
    pub(crate) result: GetHeroes,
}

#[derive(Deserialize, Debug)]
pub struct GetHeroes {
    heroes: Vec<Hero>,
    count: usize,
    status: usize,
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
    status: usize,
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

//==============================================================================

#[derive(Deserialize, Debug)]
pub struct GetRaritiesResult {
    pub(crate) result: GetRarities,
}

#[derive(Deserialize, Debug)]
pub struct GetRarities {
    count: usize,
    rarities: Vec<Rarity>,
    status: usize,
}

#[derive(Deserialize, Debug)]
pub struct Rarity {
    name: String,
    id: usize,
    order: usize,
    color: String,
    localized_name: Option<String>,
}

//==============================================================================

#[derive(Deserialize, Debug)]
pub struct GetTournamentPrizePoolResult {
    pub(crate) result: GetTournamentPrizePool,
}

#[derive(Deserialize, Debug)]
pub struct GetTournamentPrizePool {
    prize_pool: usize,
    league_id: usize,
    status: usize,
}
