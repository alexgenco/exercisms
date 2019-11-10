use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

enum MatchResult<'a> {
    Normal { winner: &'a str, loser: &'a str },
    Draw(&'a str, &'a str),
}

#[derive(PartialEq, Eq, Ord, Clone)]
struct Row {
    team: String,
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

impl Row {
    pub fn empty(team: &str) -> Self {
        Self::new(team, 0, 0, 0, 0, 0)
    }

    pub fn new(team: &str, mp: u32, w: u32, d: u32, l: u32, p: u32) -> Self {
        Self {
            team: team.to_string(),
            mp: mp,
            w: w,
            d: d,
            l: l,
            p: p,
        }
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.p == other.p {
            Some(self.team.cmp(&other.team))
        } else {
            Some(other.p.cmp(&self.p))
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.team, self.mp, self.w, self.d, self.l, self.p
        )
    }
}

pub fn tally(results_text: &str) -> String {
    let results = parse(results_text);
    let rows = accumulate(results);

    let mut res = "Team                           | MP |  W |  D |  L |  P".to_string();

    for row in rows {
        res.push_str("\n");
        res.push_str(&row.to_string());
    }

    res
}

fn parse(results: &str) -> Vec<MatchResult> {
    results
        .split("\n")
        .map(|str| str.split(";").collect::<Vec<&str>>())
        .filter(|row| row.len() == 3)
        .map(|row| match row[2] {
            "win" => MatchResult::Normal {
                winner: row[0],
                loser: row[1],
            },
            "loss" => MatchResult::Normal {
                winner: row[1],
                loser: row[0],
            },
            "draw" => MatchResult::Draw(row[0], row[1]),
            _ => panic!("Unrecognized result {:?}", row[2]),
        })
        .collect()
}

fn accumulate(results: Vec<MatchResult>) -> Vec<Row> {
    let mut rows_map = HashMap::new();

    for result in results {
        match result {
            MatchResult::Normal {
                winner: w,
                loser: l,
            } => {
                let winner_entry = rows_map.entry(w);
                let winner = winner_entry.or_insert_with(|| Row::empty(w));

                winner.mp += 1;
                winner.w += 1;
                winner.p += 3;

                let loser_entry = rows_map.entry(l);
                let loser = loser_entry.or_insert_with(|| Row::empty(l));

                loser.mp += 1;
                loser.l += 1;
            }
            MatchResult::Draw(l, r) => {
                let left_entry = rows_map.entry(l);
                let left = left_entry.or_insert_with(|| Row::empty(l));

                left.mp += 1;
                left.d += 1;
                left.p += 1;

                let right_entry = rows_map.entry(r);
                let right = right_entry.or_insert_with(|| Row::empty(r));

                right.mp += 1;
                right.d += 1;
                right.p += 1;
            }
        }
    }

    let mut res: Vec<Row> = rows_map.values().cloned().collect();

    res.sort();
    res
}
