//==============================================================================
//IEconDOTA2_<ID>
//==============================================================================

//! I have by default use Json instead of XML cause it is more popular and easy to work with, and you do not need ti use
//! them directly!!
//!
//! Almost all of the structs have `localized_name : Option<String>` as parameter, this will always
//! return `None` unless you use `language()` parameter
//! **Note**: I recommend using `language` cause it gives names like "clarity" instead of "item_clarity"

//==============================================================================
//IEconDOTA2_570
//==============================================================================

pub(crate) mod get_heroes {
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
}

pub(crate) mod get_game_items {
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
}

pub(crate) mod get_rarities {
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
}

pub(crate) mod get_tournament_prize_pool {
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
}

//==============================================================================
//IDOTA2Match_205790
//==============================================================================

pub(crate) mod get_league_listing {
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
}

//==============================================================================
//IDOTA2Match_570
//==============================================================================

pub(crate) mod get_live_league_games {

    use serde::de::{self, Error as _, MapAccess, Visitor};
    use serde::Deserialize;

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
        /// I hate steam for just giving identical keys in JSON and for mozilla to not point in out
        /// in their JSON result bobo
        #[serde(flatten)]
        abilities: Abilities,
    }

    #[derive(Debug)]
    struct Abilities(Vec<Ability>);

    #[derive(Debug, Deserialize)]
    struct Ability {
        ability_level: usize,
        ability_id: usize,
    }

    impl<'de> Deserialize<'de> for Abilities {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct MyVisitor;

            impl<'d> Visitor<'d> for MyVisitor {
                type Value = Vec<Ability>;

                fn expecting(
                    &self,
                    f: &mut std::fmt::Formatter<'_>,
                ) -> Result<(), std::fmt::Error> {
                    f.write_str("a map of abilities")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'d>,
                {
                    let mut abilities = Vec::new();
                    while let Some((key, mut value)) = access.next_entry()? {
                        if key == "abilities" {
                            abilities.append(&mut value);
                        } else {
                            return Err(M::Error::unknown_field(key, &["abilities"]));
                        }
                    }
                    Ok(abilities)
                }
            }
            Ok(Abilities(deserializer.deserialize_map(MyVisitor)?))
        }
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
}

pub(crate) mod get_top_live_game {
    #[derive(Deserialize, Debug)]
    pub struct GetTopLiveGame {
        pub(crate) game_list: Vec<GameList>,
    }

    #[derive(Deserialize, Debug)]
    pub struct GameList {
        activate_time: usize,
        deactivate_time: usize,
        lobby_id: usize,
        league_id: usize,
        lobby_type: usize,
        game_type: Option<usize>,
        delay: usize,
        spectators: usize,
        game_mode: usize,
        average_mmr: usize,
        match_id: usize,
        series_id: usize,
        team_name_radiant: Option<String>,
        team_name_dire: Option<String>,
        sort_score: usize,
        last_update_time: usize,
        radiant_lead: isize,
        radiant_score: usize,
        dire_score: usize,
        players: Option<Vec<Player>>,
        building_state: usize,
        weekend_tourney_tournament_id: Option<usize>,
        weekend_tourney_division: Option<usize>,
        weekend_tourney_skill_level: Option<usize>,
        weekend_tourney_bracket_round: Option<usize>,
        custom_game_difficulty: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct Player {
        account_id: usize,
        hero_id: usize,
    }
}
