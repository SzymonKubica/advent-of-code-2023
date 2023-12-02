use std::collections::HashMap;
use std::fs;

extern crate nom;
use enum_iterator::{all, Sequence};
use nom::bytes::complete::{tag, take_while1, take_while_m_n};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
struct Game {
    id: i32,
    colors: HashMap<Color, i32>,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    colors: HashMap<Color, i32>,
}

#[derive(Eq, Hash, PartialEq, Debug, Sequence)]
enum Color {
    Red,
    Green,
    Blue,
}

pub fn part1(input_file: &str) {
    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");
    let lines = contents.split("\n");

    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        if line != "" {
            let (_input, game) = parse_game(line).unwrap();
            println!("Input line: {}", line);
            println!("Parsed game: {:?}\n", game);
            games.push(game);
        }
    }

    let sum_of_valid_game_ids: i32 = games
        .iter()
        .filter(|g| is_game_valid(g))
        .map(|g| g.id)
        .sum();

    println!("Sum of valid game IDs: {}", sum_of_valid_game_ids);
}

fn is_game_valid(game: &Game) -> bool {
    for round in &game.rounds {
        if !is_round_valid(&round, game) {
            return false;
        }
    }
    return true;
}

fn is_round_valid(round: &Round, game: &Game) -> bool {
    for color in all::<Color>() {
        if round.colors.contains_key(&color) {
            if round.colors.get(&color) > game.colors.get(&color) {
                return false;
            }
        }
    }
    return true;
}

fn from_decimal(input: &str) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(input, 10)
}

// Examle game
// Game 1: 1 red, 2 green, 3 blue; 4 red, 5 green, 6 blue;
fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = (tag("Game ")(input))?;
    let (input, id) = game_id(input)?;
    let (input, _) = (tag(": ")(input))?;
    let round_inputs = input.split("; ");
    let mut rounds: Vec<Round> = Vec::new();

    for round in round_inputs {
        if let Ok((_, round)) = parse_round(round) {
            rounds.push(round);
        }
    }
    let game = Game {
        id,
        // The total number of available cubes is hard-coded in the question
        colors: HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]),
        rounds,
    };
    Ok(("", game))
}

fn parse_round(round: &str) -> IResult<&str, Round> {
    let colors = round.split(", ");

    let mut round = Round {
        colors: HashMap::new(),
    };

    for color in colors {
        let (input, count) = map_res(take_while1(|c: char| c.is_digit(10)), from_decimal)(color)?;
        let (input, _) = (tag(" ")(input))?;
        let color = match input {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!(),
        };
        round.colors.insert(color, count);
    }
    return Ok(("", round));
}

fn game_id(input: &str) -> IResult<&str, i32> {
    map_res(take_while1(|c: char| c.is_digit(10)), from_decimal)(input)
}

pub fn part2(input_file: &str) {}
