use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
    #[regex("[0-9]", |lex| lex.slice().parse::<usize>().ok())]
    Digit(usize),

    #[token("\n")]
    NewLine,

    #[regex("[a-zA-Z]+", logos::skip)]
    Ignored,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum Row {
    #[token("\n")]
    NewLine,

    #[regex("([a-zA-Z]|[0-9])+")]
    Row,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f\n]+")]
pub enum Numbers {
    #[regex("[0-9]", |lex| lex.slice().parse::<usize>().ok())]
    Digit(usize),

    #[token("zero")]
    Zero,

    #[token("one")]
    One,

    #[token("two")]
    Two,

    #[token("three")]
    Three,

    #[token("four")]
    Four,

    #[token("five")]
    Five,

    #[token("six")]
    Six,

    #[token("seven")]
    Seven,

    #[token("eight")]
    Eight,

    #[token("nine")]
    Nine,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f\n]+")]
pub enum NumbersReverse {
    #[regex("[0-9]", |lex| lex.slice().parse::<usize>().ok())]
    Digit(usize),

    #[token("orez")]
    Zero,

    #[token("eno")]
    One,

    #[token("owt")]
    Two,

    #[token("eerht")]
    Three,

    #[token("ruof")]
    Four,

    #[token("evif")]
    Five,

    #[token("xis")]
    Six,

    #[token("neves")]
    Seven,

    #[token("thgie")]
    Eight,

    #[token("enin")]
    Nine,
}

pub fn get_sum_of_all_calibration_values(lex: &mut logos::Lexer<'_, Token>) -> Result<usize,String>
{
    let mut digits_found: Vec<usize> = Vec::new();

    let mut sum: usize = 0;
    loop {
        match lex.next() {
            Some(Ok(Token::Digit(number))) => digits_found.push(number),
            Some(Ok(Token::NewLine)) => {
                sum += digits_found[0] * 10 + digits_found[digits_found.len() - 1];
                digits_found.clear();
            },
            Some(Err(_)) => return Err("Parsing error".to_string()),
            Some(Ok(Token::Ignored)) => return Err("Ignored token passed through".to_string()),
            None => {
                sum += digits_found[0] * 10 + digits_found[digits_found.len() - 1];
                return Ok(sum)
            },
        }
    }
}

pub fn row_to_number(row: &str) -> Result<usize, String>
{
    let mut from_left : logos::Lexer<'_, Numbers> = <Numbers as logos::Logos>::lexer(row);
    let reverse = row.to_owned().chars().rev().collect::<String>();
    let mut from_right : logos::Lexer<'_, NumbersReverse> = <NumbersReverse as logos::Logos>::lexer(&reverse);

    let left: usize;
    let right: usize;

    loop {
        match from_left.next() {
            Some(Ok(Numbers::Digit(number))) => {
                left = number;
                break;
            },
            Some(Ok(Numbers::Zero)) => {
                left = 0;
                break;
            },
            Some(Ok(Numbers::One)) => {
                left = 1;
                break;
            },
            Some(Ok(Numbers::Two)) => {
                left = 2;
                break;
            },
            Some(Ok(Numbers::Three)) => {
                left = 3;
                break;
            },
            Some(Ok(Numbers::Four)) => {
                left = 4;
                break;
            },
            Some(Ok(Numbers::Five)) => {
                left = 5;
                break;
            },
            Some(Ok(Numbers::Six)) => {
                left = 6;
                break;
            },
            Some(Ok(Numbers::Seven)) => {
                left = 7;
                break;
            },
            Some(Ok(Numbers::Eight)) => {
                left = 8;
                break;
            },
            Some(Ok(Numbers::Nine)) => {
                left = 9;
                break;
            },
            Some(Err(_)) => continue,
            None => return Err("Parsing error".to_owned()),
        }
    }

    loop {
        match from_right.next() {
            Some(Ok(NumbersReverse::Digit(number))) => {
                right = number;
                break;
            },
            Some(Ok(NumbersReverse::Zero)) => {
                right = 0;
                break;
            },
            Some(Ok(NumbersReverse::One)) => {
                right = 1;
                break;
            },
            Some(Ok(NumbersReverse::Two)) => {
                right = 2;
                break;
            },
            Some(Ok(NumbersReverse::Three)) => {
                right = 3;
                break;
            },
            Some(Ok(NumbersReverse::Four)) => {
                right = 4;
                break;
            },
            Some(Ok(NumbersReverse::Five)) => {
                right = 5;
                break;
            },
            Some(Ok(NumbersReverse::Six)) => {
                right = 6;
                break;
            },
            Some(Ok(NumbersReverse::Seven)) => {
                right = 7;
                break;
            },
            Some(Ok(NumbersReverse::Eight)) => {
                right = 8;
                break;
            },
            Some(Ok(NumbersReverse::Nine)) => {
                right = 9;
                break;
            },
            Some(Err(_)) => continue,
            None => return Err("Parsing error".to_owned()),
        }
    }

    Ok(left*10 + right)
}

pub fn get_sum_of_all_calibration_values_extra(lex: &mut logos::Lexer<'_, Row>) -> Result<usize, String>
{
    let mut sum: usize = 0;
    loop {
        match lex.next() {
            Some(Ok(Row::Row)) => {
                sum += row_to_number(lex.slice())?;
            },
            Some(Ok(Row::NewLine)) => {
                continue;
            }
            Some(Err(_)) => return Err("Parsing error".to_owned()),
            None => return Ok(sum),
        }
    }
}
