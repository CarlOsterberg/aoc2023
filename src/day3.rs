use logos::{Logos, Lexer, Span};
use std::collections::HashSet;

#[derive(Logos, Debug, PartialEq)]
pub enum GearRatios {
    #[token("\n", |lex| lex.span())]
    NewLine(Span),

    #[token(".", logos::skip)]
    Dot,

    #[regex("[0-9]+", |lex| lex.slice().parse::<usize>().ok())]
    Number(usize),

    #[regex(r"[^\.0-9]", |lex| lex.span())]
    Character(Span),
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Coordinates
{
    x: i32,
    y: i32,
}

impl Coordinates
{
    fn add(&self, x:i32, y:i32) -> Coordinates
    {
        Coordinates {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn coordinate_has_two_adjacent_parts(&self, parts: &Vec<Part>) -> Option<usize>
    {
        let mut vec:Vec<usize> = Vec::new();

        for part in parts {
            let start_x = part.coordinate.x-1;
            let end_x = part.coordinate.x + part.number.to_string().len() as i32;
            let coord_start_x = Coordinates {x: start_x, y: part.coordinate.y};
            let coord_end_x = Coordinates {x: end_x, y: part.coordinate.y};

            if coord_start_x == *self || coord_end_x == *self {
                vec.push(part.number);
                continue;
            }

            for number in start_x..=end_x {
                let coord_plus_y = Coordinates {x: number, y: part.coordinate.y+1};
                let coord_negative_y = Coordinates {x: number, y: part.coordinate.y-1};
                if coord_plus_y == *self || coord_negative_y == *self {
                    vec.push(part.number);
                    break;
                }
            }
        }
        if vec.len() == 2 {
            Some(vec[0] * vec[1])
        }
        else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Part
{
    number: usize,
    coordinate: Coordinates,
}

impl Part {
    pub fn new(number: usize, span: Span, newlines: i32, rowlength: i32) -> Part
    {
        Part {
            number,
            coordinate: Coordinates {
                x: span.start as i32 - rowlength,
                y: newlines
            }
        }
    }

    pub fn get_number(&self) -> usize
    {
        self.number
    }

    pub fn has_adjacent_symbol(&self, symbol_coordinates: &HashSet<Coordinates>) -> bool
    {
        let start_x = -1;
        let end_x = self.number.to_string().len() as i32;

        if symbol_coordinates.contains(&self.coordinate.add(start_x, 0)) {
            return true;
        }
        if symbol_coordinates.contains(&self.coordinate.add(end_x, 0)) {
            return true;
        }

        for number in start_x..=end_x {
            if symbol_coordinates.contains(&self.coordinate.add(number, 1)) {
                return true;
            }
            if symbol_coordinates.contains(&self.coordinate.add(number, -1)) {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Schematic
{
    engine_parts: Vec<Part>,
    symbol_coordinates: HashSet<Coordinates>,
}

impl Schematic {
    pub fn new() -> Schematic
    {
        Schematic {
            engine_parts: vec![],
            symbol_coordinates: HashSet::new(),
        }
    }

    pub fn get_parts_with_adjacent_symbol(&self) -> Vec<Part>
    {
        let mut vec:Vec<Part> = Vec::new();

        for part in &self.engine_parts {
            if part.has_adjacent_symbol(&self.symbol_coordinates) {
                vec.push(part.clone());
            }
        }
        vec
    }

    pub fn get_gear_ratios(&self) -> Vec<usize>
    {
        let valid_parts = self.get_parts_with_adjacent_symbol();
        let mut gear_ratios = Vec::new();

        for coordinate in &self.symbol_coordinates {
            match coordinate.coordinate_has_two_adjacent_parts(&valid_parts) {
                Some(gear_ratio) => gear_ratios.push(gear_ratio),
                None => (),
            }
        }

        gear_ratios
    }
}

pub fn tokens_to_schematic(lex: &mut logos::Lexer<'_, GearRatios>) -> Result<Schematic, String>
{
    let mut schematic = Schematic::new();
    let mut newlines: i32 = 0;
    let mut rowlength: i32 = 0;
    loop {
        match lex.next() {
            Some(Ok(GearRatios::Number(number))) => schematic.engine_parts.push(Part::new(number, lex.span(), newlines, rowlength)),
            Some(Ok(GearRatios::NewLine(span))) => {
                rowlength = span.end as i32;
                newlines += 1;
            },
            Some(Ok(GearRatios::Character(span))) => 
            {
                schematic.symbol_coordinates.insert(
                    Coordinates {
                        x: span.start as i32 - rowlength,
                        y: newlines,
                    }
                );
            },
            Some(_) => return Err("Parsing error".to_owned()),
            None => return Ok(schematic),
        }
    }
}

