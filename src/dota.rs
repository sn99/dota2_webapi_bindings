//==============================================================================
//IEconDOTA2_<ID>
//==============================================================================

//! I have by default use Json instead of XML cause it is more popular and easy to work with, and you do not need ti use
//! them directly!!
//!
//! Almost all of the structs have `localized_name : Option<String>` as parameter, this will always
//! return `None` unless you use `language()` parameter
//! **Note**: I recommend using `language` cause it gives names like "clarity" instead of "item_clarity"

use serde_with;

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

//==============================================================================
//IDOTA2Match_<ID>
//==============================================================================

#[derive(Deserialize, Debug)]
pub struct GetLeagueListingResult {
    pub(crate) result: GetLeagueListing,
}

#[derive(Deserialize, Debug)]
pub struct GetLeagueListing {
    leagues: Vec<League>,
}

#[derive(Deserialize, Debug)]
pub struct League {
    name: String,
    #[serde(rename = "leagueid")]
    league_id: usize,
    description: Option<String>,
    tournament_url: String,
    #[serde(rename = "itemdef")]
    item_def: usize,
}

//==============================================================================

#[derive(Deserialize, Debug)]
pub struct GetLiveLeagueGamesResult {
    pub(crate) result: GetLiveLeagueGames,
}

#[derive(Deserialize, Debug)]
pub struct GetLiveLeagueGames {
    games: Vec<Game>,
    status: usize,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    players: Vec<Player>,
    radiant_team: Option<RadiantTeam>,
    dire_team: Option<DireTeam>,
    lobby_id: usize,
    match_id: usize,
    spectators: usize,
    league_id: usize,
    league_node_id: usize,
    stream_delay_s: usize,
    radiant_series_wins: usize,
    dire_series_wins: usize,
    series_type: usize,
    scoreboard: Option<Scoreboard>,
}

#[derive(Deserialize, Debug)]
pub struct Player {
    account_id: usize,
    name: String,
    hero_id: usize,
    team: usize,
}

#[derive(Deserialize, Debug)]
pub struct RadiantTeam {
    team_name: String,
    team_id: usize,
    team_logo: usize,
    complete: bool,
}

#[derive(Deserialize, Debug)]
pub struct DireTeam {
    team_name: String,
    team_id: usize,
    team_logo: usize,
    complete: bool,
}

#[derive(Deserialize, Debug)]
pub struct Scoreboard {
    duration: f64,
    roshan_respawn_timer: usize,
    radiant: Ancient,
    dire: Ancient,
}

#[derive(Deserialize, Debug)]
pub struct Ancient {
    score: usize,
    tower_state: usize,
    barracks_state: usize,
    picks: Option<Vec<HeroId>>,
    bans: Option<Vec<HeroId>>,
    players: Vec<PlayerDetailed>,
    /// I hate steam for just giving just one ability in json format while in fact there are 5 in raw
    /// and for mozilla to not point in out bobo
    //    #[serde(with = "serde_with::rust::tuple_list_as_map")]
    abilities: Option<Vec<Ability>>,
}

#[derive(Deserialize, Debug)]
pub struct HeroId {
    hero_id: usize,
}

#[derive(Deserialize, Debug)]
pub struct PlayerDetailed {
    player_slot: usize,
    account_id: usize,
    hero_id: usize,
    kills: usize,
    death: usize,
    assists: usize,
    last_hits: usize,
    denies: usize,
    gold: usize,
    level: usize,
    gold_per_min: usize,
    xp_per_min: usize,
    ultimate_state: usize,
    ultimate_cooldown: usize,
    item0: usize,
    item1: usize,
    item2: usize,
    item3: usize,
    item4: usize,
    item5: usize,
    respawn_timer: usize,
    position_x: f64,
    position_y: f64,
    net_worth: usize,
}

#[derive(Deserialize, Debug)]
pub struct Ability {
    ability_id: usize,
    ability_level: usize,
}
