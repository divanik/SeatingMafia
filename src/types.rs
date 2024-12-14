use std::collections::{BTreeMap, BTreeSet};

use num_rational::Rational32;

type Id = u8;

struct Table {
    judge: Id,
    players: [Id; 10],
}

type Round = Vec<Table>;

type Tournament = Vec<Round>;

type ForbiddenPair = (Id, Id);

type ForbiddenPairs = BTreeSet<ForbiddenPair>;

// type GamesPerPerson = BTreeMap<Id, u8>;

struct TournamentCharacteristics {
    forbidden_pairs: ForbiddenPairs,
    games_per_person: u8, 
    players: Vec<Id>,
    judges: Vec<Id>,
    people: Vec<Id>
}

type PreferredMutualGames = BTreeMap<(Id, Id), (u8, u8)>;

struct TournamentState {
    tournament: Tournament,
    mutual_games: MutualGames,
    preferred_mutual_games: PreferredMutualGames,
    fine: u64
}

// impl TournamentCharacteristics {

// fn is_table_ok(&self, table : &Table) -> bool {
//     for i in 0..10 {
//         for j in 0..10 {
//             if i != j && (table.players[i] == table.players[j]) || (self.forbidden_pairs.contains(&(table.players[i], table.players[j]))) {
//                 return false;
//             }
//         }
//     }
//     for i in 0..10 {
//         if table.judge == table.players[i] || self.forbidden_pairs.contains(&(table.judge, table.players[i])) {
//             return false;
//         }
//     }
//     true
// }

type Meetings = BTreeMap<(Id, Id), u8>;

fn count_meetings_in_tournament(tournament : &Tournament) -> Meetings {
    let mut meetings = BTreeMap::new();
    for round in tournament {
        for table in round {
            for i in 0..10 {
                for j in 0..10 {
                    if i != j {
                        *meetings.entry((table.players[i], table.players[j])).or_insert(0) += 1;
                        *meetings.entry((table.players[j], table.players[i])).or_insert(0) += 1;
                    }
                }
            }
            for i in 0..10 {
                *meetings.entry((table.players[i], table.judge)).or_insert(0) += 1;
                *meetings.entry((table.judge, table.players[i])).or_insert(0) += 1;
            }
        }
    }
    meetings
}

type MutualGames = BTreeMap<(Id, Id), u8>;


fn count_preferred_mutual_games(characteristics : &TournamentCharacteristics) -> PreferredMutualGames {
    let mut preferred_mutual_games = BTreeMap::new();
    let mut allowed_players_number = BTreeMap::new();
    let mut allowed_judges_number = BTreeMap::new();
    for player1 in &characteristics.players {
        for player2 in &characteristics.players {
            if (player1 != player2) && !characteristics.forbidden_pairs.contains(&(*player1, *player2)) {
                *allowed_players_number.entry(*player1).or_insert(0) += 1;
            }
        }
    }

    for judge in &characteristics.judges {
        for player in &characteristics.players {
            if !characteristics.forbidden_pairs.contains(&(*judge, *player)) {
                *allowed_judges_number.entry(*player).or_insert(0) += 1;
            }
        }
    }
    for player1 in &characteristics.players {
        for player2 in &characteristics.players {
            if (player1 != player2) && !characteristics.forbidden_pairs.contains(&(*player1, *player2)) {
                let rat = Rational32::new(5 * characteristics.games_per_person as i32, allowed_players_number[player1]) + Rational32::new(5 * characteristics.games_per_person as i32, allowed_players_number[player2]);
                preferred_mutual_games.insert((*player1, *player2), (rat.floor().to_integer() as u8, rat.ceil().to_integer()  as u8));
            } else {
                preferred_mutual_games.insert((*player1, *player2), (0, 0));
            }
        }
    }
    for judge in &characteristics.judges {
        for player in &characteristics.players {
            if !characteristics.forbidden_pairs.contains(&(*judge, *player)) {
                let rat = Rational32::new(10 * characteristics.games_per_person as i32, allowed_players_number[player]);
                preferred_mutual_games.insert((*player, *judge), (rat.floor().to_integer() as u8, rat.ceil().to_integer()  as u8));      
            }
        }
    }
    preferred_mutual_games
}



fn get_fine(tournament : &Tournament, characteristics : &TournamentCharacteristics) -> u64 {
    let mutual_games = count_meetings_in_tournament(tournament);
    let preferred_mutual_games = count_preferred_mutual_games(characteristics);
    let mut fine : u64 = 0;
    for (pair, (min, max)) in preferred_mutual_games {
        let actual_games = mutual_games.get(&pair).unwrap_or(&0);
        if characteristics.forbidden_pairs.contains(&pair) {
            fine += 1_000_000_000;
        }
        if *actual_games < min {
            fine += ((min - *actual_games) as u64).pow(3);
        } else if *actual_games > max {
            fine += ((*actual_games - max) as u64).pow(3);
        }
    }
    fine
}

struct MoveVector {
    round: usize,
    table1: usize,
    table2: usize,
    player1: usize,
    player2: usize
}

fn remove_player_from_table(table : &mut Table, player : &MoveVector) {
    
}
 
fn change_fine(state : &mut TournamentState, move_vector : &MoveVector) {

}
