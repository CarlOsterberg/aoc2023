use logos::{Logos, Lexer};
use std::collections::HashSet;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f:]+")]
pub enum ScratchcardTokens {
    #[token("\n")]
    NewLine,
    #[regex("[0-9]+", |lex| lex.slice().parse::<usize>().ok())]
    Number(usize),

    #[token("|")]
    Pipe,

    #[regex(r"Card\s+[0-9]+", extract_number)]
    Game(usize),
}

fn extract_number(lex: &mut Lexer<ScratchcardTokens>) -> usize {
    let parts: Vec<&str> = lex.slice().split_whitespace().collect();
    parts[1].parse::<usize>().unwrap()
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Scratchcards {
    number: usize,
    winning: HashSet<usize>,
    received: HashSet<usize>,
}

impl Scratchcards {
    pub fn new(number: usize) -> Scratchcards
    {
        Scratchcards { number, winning: HashSet::new(), received: HashSet::new() }
    }

    pub fn fill_sets(lex: &mut logos::Lexer<'_, ScratchcardTokens>) -> Result<Vec<Scratchcards>, String>
    {
        let mut vec: Vec<Scratchcards> = Vec::new();
        let mut game = Scratchcards::new(0);
        let mut insert_into_winning = true;

        for token in lex.into_iter() 
        {
            match token {
                Ok(ScratchcardTokens::Game(number)) => {
                    insert_into_winning = true;
                    game = Scratchcards::new(number);
                },
                Ok(ScratchcardTokens::Number(number)) => {
                    if insert_into_winning {
                        game.winning.insert(number);
                    }
                    else {
                        game.received.insert(number);
                    }
                },
                Ok(ScratchcardTokens::Pipe) => insert_into_winning = false,
                Ok(ScratchcardTokens::NewLine) => {
                    vec.push(game);
                    game = Scratchcards::new(0);
                },
                Err(_) => return Err("Parsing error".to_owned()),
            }
        }
        Ok(vec)
    }

    pub fn intersectionize(scratchcards: Vec<Scratchcards>) -> u32
    {
        scratchcards.iter()
        .map(
            |x|
            {
                let base = x.winning.intersection(&x.received).count() as u32;
                if base == 0
                    { 0 }
                else
                    { (2 as u32).pow( base - 1) }
            }
        ).fold(
            0, |acc: u32, x| acc + x
        )
    }

    pub fn cascading_copies(scratchcards: Vec<Scratchcards>) -> Vec<usize>
    {
        let intersections: Vec<usize> = scratchcards
        .iter()
        .map(
            |x|
                x.winning.intersection(&x.received).count()
        ).collect();

        let length = intersections.len();

        let mut cascading_copies: Vec<usize> = vec![1;length];

        let mut i: usize = 0;
        while i < length  {
            for _ in 0..cascading_copies[i] {
                for j in 1..=intersections[i] {
                    cascading_copies[j + i] += 1;
                }
            }
            i += 1;
        }
        
        cascading_copies
    }

}