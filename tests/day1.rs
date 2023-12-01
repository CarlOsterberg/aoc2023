use aoc2023;


#[cfg(test)]
mod day1tests {
    use aoc2023::day1::{*};

    use super::*;

    #[test]
    fn parse_to_tokens_from_example()
    {
        let string = aoc2023::read_to_string("tests/data/day1example.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        assert_eq!(lex.next(), Some(Ok(Token::Digit(1))));
        assert_eq!(lex.next(), Some(Ok(Token::Digit(2))));
        assert_eq!(lex.next(), Some(Ok(Token::NewLine)));

        assert_eq!(lex.next(), Some(Ok(Token::Digit(3))));
        assert_eq!(lex.next(), Some(Ok(Token::Digit(8))));
        assert_eq!(lex.next(), Some(Ok(Token::NewLine)));

        assert_eq!(lex.next(), Some(Ok(Token::Digit(1))));
        assert_eq!(lex.next(), Some(Ok(Token::Digit(2))));
        assert_eq!(lex.next(), Some(Ok(Token::Digit(3))));
        assert_eq!(lex.next(), Some(Ok(Token::Digit(4))));
        assert_eq!(lex.next(), Some(Ok(Token::Digit(5))));
        assert_eq!(lex.next(), Some(Ok(Token::NewLine)));

        assert_eq!(lex.next(), Some(Ok(Token::Digit(7))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn get_sum_of_all_calibration_values_example()
    {
        let string = aoc2023::read_to_string("tests/data/day1example.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        let res = get_sum_of_all_calibration_values(&mut lex);

        assert_eq!(Ok(142), res);
    }

    #[test]
    fn get_sum_of_all_calibration_values_task()
    {
        let string = aoc2023::read_to_string("tests/data/day1a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        let res = get_sum_of_all_calibration_values(&mut lex);

        assert_eq!(Ok(56397), res);
    }

    #[test]
    fn parse_to_tokens_from_example2()
    {
        let string = aoc2023::read_to_string("tests/data/day1example2.txt").unwrap();
        let mut lex: logos::Lexer<'_, Row> = <Row as logos::Logos>::lexer(&string);

        assert_eq!(lex.next(), Some(Ok(Row::Row)));
        assert_eq!(lex.slice(), "two1nine");
        assert_eq!(lex.next(), Some(Ok(Row::NewLine)));
        assert_eq!(lex.next(), Some(Ok(Row::Row)));
        assert_eq!(lex.slice(), "eightwothree");
        assert_eq!(lex.next(), Some(Ok(Row::NewLine)));
        assert_eq!(lex.next(), Some(Ok(Row::Row)));
        assert_eq!(lex.slice(), "abcone2threexyz");
        assert_eq!(lex.next(), Some(Ok(Row::NewLine)));
        assert_eq!(lex.next(), Some(Ok(Row::Row)));
        assert_eq!(lex.slice(), "xtwone3four");
        assert_eq!(lex.next(), Some(Ok(Row::NewLine)));
        assert_eq!(lex.next(), Some(Ok(Row::Row)));
        assert_eq!(lex.slice(), "4nineeightseven2");
        assert_eq!(lex.next(), Some(Ok(Row::NewLine)));
        assert_eq!(lex.next(), Some(Ok(Row::Row)));
        assert_eq!(lex.slice(), "zoneight234");
        assert_eq!(lex.next(), Some(Ok(Row::NewLine)));
        assert_eq!(lex.next(), Some(Ok(Row::Row)));
        assert_eq!(lex.slice(), "7pqrstsixteen");
    }

    #[test]
    fn parse_from_row()
    {
        let mut lex: logos::Lexer<'_, Numbers> = <Numbers as logos::Logos>::lexer("two1nine");

        assert_eq!(lex.next(), Some(Ok(Numbers::Two)));
        assert_eq!(lex.next(), Some(Ok(Numbers::Digit(1))));
        assert_eq!(lex.next(), Some(Ok(Numbers::Nine)));

        let mut lex: logos::Lexer<'_, NumbersReverse> = <NumbersReverse as logos::Logos>::lexer("enin1owt");

        assert_eq!(lex.next(), Some(Ok(NumbersReverse::Nine)));
        assert_eq!(lex.next(), Some(Ok(NumbersReverse::Digit(1))));
        assert_eq!(lex.next(), Some(Ok(NumbersReverse::Two)));
    }

    #[test]
    fn row_to_number_test()
    {
        let res = row_to_number("two1nine");
        assert_eq!(res, Ok(29));
        let res = row_to_number("eightwothree");
        assert_eq!(res, Ok(83));
        let res = row_to_number("abcone2threexyz");
        assert_eq!(res, Ok(13));
        let res = row_to_number("xtwone3four");
        assert_eq!(res, Ok(24));
        let res = row_to_number("4nineeightseven2");
        assert_eq!(res, Ok(42));
        let res = row_to_number("zoneight234");
        assert_eq!(res, Ok(14));
        let res = row_to_number("7pqrstsixteen");
        assert_eq!(res, Ok(76));
        let res = row_to_number("fiveight");
        assert_eq!(res, Ok(58));
    }

    #[test]
    fn get_sum_of_all_calibration_values_extra_example()
    {
        let string = aoc2023::read_to_string("tests/data/day1example2.txt").unwrap();
        let mut lex: logos::Lexer<'_, Row> = <Row as logos::Logos>::lexer(&string);

        let res = get_sum_of_all_calibration_values_extra(&mut lex);

        assert_eq!(Ok(281), res);
    }
    
    #[test]
    fn get_sum_of_all_calibration_values_extra_task()
    {
        let string = aoc2023::read_to_string("tests/data/day1b.txt").unwrap();
        let mut lex: logos::Lexer<'_, Row> = <Row as logos::Logos>::lexer(&string);

        let res = get_sum_of_all_calibration_values_extra(&mut lex);

        assert_eq!(Ok(55701), res);
    }

}
