
use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f:,]+")]
pub enum CubeConundrum {
    #[token("\n")]
    NewLine,

    #[token(";")]
    NewGrab,

    #[regex("[0-9]+ red", extract_number)]
    Red(usize),

    #[regex("[0-9]+ green", extract_number)]
    Green(usize),

    #[regex("[0-9]+ blue", extract_number)]
    Blue(usize),

    #[regex("Game [0-9]+", extract_number)]
    Game(usize),
}

fn extract_number(lex: &mut Lexer<CubeConundrum>) -> Option<usize> {
    let parts: Vec<&str> = lex.slice().split_whitespace().collect();
    let part = if parts[0] == "Game" { 1 } else { 0 };
    let res = parts[part].parse::<usize>();
    match res {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Grab
{
    red: usize,
    blue: usize,
    green: usize,
}

impl Grab {
    fn new() -> Grab
    {
        Grab { red: 0, blue: 0, green: 0 }
    }

    fn power(&self) -> usize
    {
        self.red * self.blue * self.green
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Game
{
    id: usize,
    grabs: Vec<Grab>,
}

impl Game {
    fn new(id: usize) -> Game
    {
        Game { id, grabs: vec![] }
    }

    pub fn tokens_to_games(lex: &mut logos::Lexer<'_, CubeConundrum>) -> Result<Vec<Game>, String>
    {
        let mut vec: Vec<Game> = Vec::new();
        let mut game: Game = Game::new(0);
        let mut grab: Grab = Grab::new();
        loop {
            match lex.next()
            {
                Some(Ok(CubeConundrum::Game(number))) => game = Game::new(number),
                Some(Ok(CubeConundrum::Red(number))) => grab.red = number,
                Some(Ok(CubeConundrum::Blue(number))) => grab.blue = number,
                Some(Ok(CubeConundrum::Green(number))) => grab.green = number,
                Some(Ok(CubeConundrum::NewGrab)) => {
                    game.grabs.push(grab.clone());
                    grab = Grab::new();
                },
                Some(Ok(CubeConundrum::NewLine)) => {
                    game.grabs.push(grab.clone());
                    vec.push(game.clone());
                    grab = Grab::new();
                },
                Some(Err(_)) => return Err("parsing error".to_owned()),
                None => break,
            }
        }
        Ok(vec)
    }

    pub fn possible_games(vec: Vec<Game>, red: usize, green: usize, blue: usize) -> Vec<usize>
    {
        let mut vec_res: Vec<usize> = Vec::new();
        let mut is_possible = true;

        for game in vec
        {
            for grab in game.grabs
            {
                if grab.blue > blue ||
                    grab.red > red ||
                    grab.green > green
                {
                    is_possible = false;
                    break;
                }
            }
            if is_possible
            {
                vec_res.push(game.id);
            }
            is_possible = true;
        }

        vec_res
    }

    fn get_fewest_needed_among_grabs(&self) -> Grab
    {
        let mut red:usize = 0;
        let mut blue:usize = 0;
        let mut green:usize = 0;

        for grab in &self.grabs {
            if grab.red > red {
                red = grab.red;
            }
            if grab.blue > blue {
                blue = grab.blue;
            }
            if grab.green > green {
                green = grab.green;
            }
        }

        Grab { red, blue, green }
    }

    pub fn sum_of_powers(vec: Vec<Game>) -> usize
    {
        let res: Vec<usize> = vec.iter().map(|x| x.get_fewest_needed_among_grabs().power()).collect();
        res.iter().fold(0, |acc, &x| acc + x)
    }
}


