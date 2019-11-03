use std::fs::File;
use std::ops::Not;
use std::path::Path;
use std::io::{self, Read, Write};

#[derive(Clone)]
enum GameResult {
    Win,
    Loss,
    Draw
}

impl Not for GameResult {
    type Output = GameResult;

    fn not(self) -> GameResult {
        match self {
            GameResult::Win => GameResult::Loss,
            GameResult::Loss => GameResult::Win,
            GameResult::Draw => GameResult::Draw
        }
    }
}

struct Game {
    team1: String,
    team2: String,
    result: GameResult
}

impl Game {
    fn from_str(input: &str) -> Game {
        let items = input.split(';').collect::<Vec<&str>>();

        Game {
            team1: items[0].to_owned(),
            team2: items[1].to_owned(),
            result: match items[2] {
                "win" => GameResult::Win,
                "loss" => GameResult::Loss,
                "draw" => GameResult::Draw,
                _ => panic!("Can't parse game result")
            }
        }
    }
}

struct TeamSummary {
    name: String,
    matches: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize
}

impl TeamSummary {
    fn new(name: String) -> TeamSummary {
        TeamSummary {
            name: name,
            matches: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0
        }
    }

    fn add_result(&mut self, res: &GameResult) {
        match res {
            &GameResult::Win => {
                self.add_win();
            },
            &GameResult::Loss => {
                self.add_loss();
            },
            &GameResult::Draw => {
                self.add_draw();
            }
        }
    }

    fn add_win(&mut self) {
        self.matches += 1;
        self.wins += 1;
        self.points += 3;
    }

    fn add_loss(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }

    fn add_draw(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }
}

pub fn tally(ipath: &Path, opath: &Path) -> Result<usize, io::Error> {
    let mut games = Vec::new();
    let mut input = String::new();
    let mut ifile = try!(File::open(ipath));

    try!(ifile.read_to_string(&mut input));

    for line in input.lines() {
        games.push(Game::from_str(line));
    }

    let summary = summarize(&games);

    try!(dump_summary(&summary, opath));

    Ok(games.len())
}

fn summarize(games: &Vec<Game>) -> Vec<TeamSummary> {
    let mut summary: Vec<TeamSummary> = Vec::new();

    for game in games.iter() {
        let t1 = game.team1.clone();
        let t2 = game.team2.clone();
        let res = game.result.clone();

        let mut ts1 = match summary.iter_mut().find(|s| s.name == t1) {
            Some(&mut summ) => summ,
            None => TeamSummary::new(t1)
        };

        let mut ts2 = match summary.iter_mut().find(|s| s.name == t2) {
            Some(&mut summ) => summ,
            None => TeamSummary::new(t2)
        };

        ts1.add_result(&res);
        ts2.add_result(&!res);
    }

    summary.sort_by(|s1, s2| s2.points.cmp(&s1.points));
    summary
}

fn dump_summary(summary: &Vec<TeamSummary>, path: &Path) -> Result<(), io::Error> {
    let mut rows = Vec::new();
    let header = format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", "Team", "MP", "W", "D", "L", "P");

    rows.push(header);

    for summ in summary.iter() {
        let row = format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            summ.name,
            summ.matches,
            summ.wins,
            summ.draws,
            summ.losses,
            summ.points
            );

        rows.push(row);
    }

    let mut ofile = try!(File::create(path));
    write!(ofile, "{}", rows.join("\n"))
}
