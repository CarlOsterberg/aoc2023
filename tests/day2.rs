use aoc2023;


#[cfg(test)]
mod day2tests {
    use aoc2023::day2::{*};

    use super::*;

    #[test]
    fn parse_to_tokens_from_example()
    {
        let string = aoc2023::read_to_string("tests/data/day2example.txt").unwrap();
        let mut lex: logos::Lexer<'_, CubeConundrum> = <CubeConundrum as logos::Logos>::lexer(&string);

        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Game(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(3))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(4))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(2))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(6))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(2))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewLine)));

        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Game(2))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(2))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(3))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(4))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewLine)));

        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Game(3))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(8))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(6))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(20))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(5))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(4))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(13))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(5))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewLine)));

        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Game(4))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(3))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(6))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(3))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(6))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(3))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(15))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(14))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewLine)));

        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Game(5))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(6))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(3))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewGrab)));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Blue(2))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Red(1))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::Green(2))));
        assert_eq!(lex.next(), Some(Ok(CubeConundrum::NewLine)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn tokens_to_possible_games_example()
    {
        let string = aoc2023::read_to_string("tests/data/day2example.txt").unwrap();
        let mut lex: logos::Lexer<'_, CubeConundrum> = <CubeConundrum as logos::Logos>::lexer(&string);

        let res = Game::tokens_to_games(&mut lex);

        assert!(res.is_ok());
        let games = res.unwrap();
        assert_eq!(5, games.len());

        let possible_games = Game::possible_games(games, 12, 13, 14);
        assert_eq!(possible_games, vec![1, 2, 5]);
        assert_eq!(8, possible_games.iter().fold(0, |acc, &x| acc + x));
    }

    #[test]
    fn tokens_to_possible_games_task()
    {
        let string = aoc2023::read_to_string("tests/data/day2a.txt").unwrap();
        let mut lex: logos::Lexer<'_, CubeConundrum> = <CubeConundrum as logos::Logos>::lexer(&string);

        let res = Game::tokens_to_games(&mut lex);

        assert!(res.is_ok());
        let games = res.unwrap();

        let possible_games = Game::possible_games(games, 12, 13, 14);
        assert_eq!(2551, possible_games.iter().fold(0, |acc, &x| acc + x));
    }

    #[test]
    fn tokens_to_powers_sum_example()
    {
        let string = aoc2023::read_to_string("tests/data/day2example.txt").unwrap();
        let mut lex: logos::Lexer<'_, CubeConundrum> = <CubeConundrum as logos::Logos>::lexer(&string);

        let res = Game::tokens_to_games(&mut lex);

        assert!(res.is_ok());
        let games = res.unwrap();

        assert_eq!(2286, Game::sum_of_powers(games));
    }

    #[test]
    fn tokens_to_powers_sum_task()
    {
        let string = aoc2023::read_to_string("tests/data/day2b.txt").unwrap();
        let mut lex: logos::Lexer<'_, CubeConundrum> = <CubeConundrum as logos::Logos>::lexer(&string);

        let res = Game::tokens_to_games(&mut lex);

        assert!(res.is_ok());
        let games = res.unwrap();

        assert_eq!(62811, Game::sum_of_powers(games));
    }


}
