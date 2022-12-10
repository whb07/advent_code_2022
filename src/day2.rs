#[derive(PartialEq, Debug, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }
}

impl From<&str> for Hand {
    fn from(d: &str) -> Hand {
        match d {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissor,
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            _ => Hand::Scissor,
        }
    }
}

pub enum Outcome {
    Win,
    Loss,
    Tie,
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Tie => 3,
        }
    }
    fn win() -> Self {
        Outcome::Win
    }
    fn loss() -> Self {
        Outcome::Loss
    }
    fn tie() -> Self {
        Outcome::Tie
    }
}

#[derive(PartialEq, Debug)]
pub struct Round {
    pub(crate) hero: Hand,
    pub(crate) opponent: Hand,
}

impl Round {
    fn new(opponent: Hand, hero: Hand) -> Self {
        Round { hero, opponent }
    }

    fn with_how_round_ends(opponent: Hand, hero_data: &str) -> Self {
        match hero_data {
            "Y" => {
                let hero = opponent.clone();
                Round::new(opponent, hero)
            }

            "X" => match opponent {
                Hand::Rock => Round::new(opponent, Hand::Scissor),
                Hand::Scissor => Round::new(opponent, Hand::Paper),
                Hand::Paper => Round::new(opponent, Hand::Rock),
            },
            _ => match opponent {
                Hand::Rock => Round::new(opponent, Hand::Paper),
                Hand::Scissor => Round::new(opponent, Hand::Rock),
                Hand::Paper => Round::new(opponent, Hand::Scissor),
            },
        }
    }

    fn play(&self) -> usize {
        match (&self.hero, &self.opponent) {
            (Hand::Rock, Hand::Paper) => Outcome::loss().score(),
            (Hand::Rock, Hand::Scissor) => Outcome::win().score(),
            (Hand::Rock, Hand::Rock) => Outcome::tie().score(),
            (Hand::Paper, Hand::Scissor) => Outcome::loss().score(),
            (Hand::Paper, Hand::Paper) => Outcome::tie().score(),
            (Hand::Paper, Hand::Rock) => Outcome::win().score(),
            (Hand::Scissor, Hand::Scissor) => Outcome::tie().score(),
            (Hand::Scissor, Hand::Paper) => Outcome::win().score(),
            (Hand::Scissor, Hand::Rock) => Outcome::loss().score(),
        }
    }

    pub fn score(&self) -> usize {
        let gamescore = self.play();
        let handscore = self.hero.score();
        gamescore + handscore
    }
}

impl From<&String> for Round {
    fn from(d: &String) -> Round {
        let x: Vec<&str> = d.split(" ").collect();
        let opponent = Hand::from(x[0]);
        // let hero = Hand::from();
        Round::with_how_round_ends(opponent, x[1])
    }
}

pub fn run_game(data: &Vec<String>) -> usize {
    data.iter().map(|e| Round::from(e).score()).sum()
}
