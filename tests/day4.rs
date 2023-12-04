use aoc2023;

#[cfg(test)]
mod day4tests {
    use aoc2023::day4::{*};

    use super::*;

    #[test]
    fn parse_to_tokens_from_example()
    {
        let string = aoc2023::read_to_string("tests/data/day4example.txt").unwrap();
        let mut lex: logos::Lexer<'_, ScratchcardTokens> = <ScratchcardTokens as logos::Logos>::lexer(&string);

        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Game(1))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(41))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(48))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(83))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(86))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(17))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Pipe)));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(83))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(86))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(6))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(31))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(17))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(9))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(48))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Number(53))));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::NewLine)));
        assert_eq!(lex.next(), Some(Ok(ScratchcardTokens::Game(2))));
    }

    #[test]
    fn scratchcards_intersectionize_example()
    {
        let string = aoc2023::read_to_string("tests/data/day4example.txt").unwrap();
        let mut lex: logos::Lexer<'_, ScratchcardTokens> = <ScratchcardTokens as logos::Logos>::lexer(&string);

        let res = Scratchcards::fill_sets(&mut lex);
        assert!(res.is_ok());
        let scratchcards = res.unwrap();
        assert_eq!(13, Scratchcards::intersectionize(scratchcards));

    }

    #[test]
    fn scratchcards_intersectionize_task()
    {
        let string = aoc2023::read_to_string("tests/data/day4a.txt").unwrap();
        let mut lex: logos::Lexer<'_, ScratchcardTokens> = <ScratchcardTokens as logos::Logos>::lexer(&string);

        let scratchcards = Scratchcards::fill_sets(&mut lex).unwrap();
        assert_eq!(22193, Scratchcards::intersectionize(scratchcards));
    }

    #[test]
    fn scratchcards_cascading_copies_example()
    {
        let string = aoc2023::read_to_string("tests/data/day4example.txt").unwrap();
        let mut lex: logos::Lexer<'_, ScratchcardTokens> = <ScratchcardTokens as logos::Logos>::lexer(&string);

        let scratchcards = Scratchcards::fill_sets(&mut lex).unwrap();

        assert_eq!(
            30,
            Scratchcards::cascading_copies(scratchcards)
            .iter()
            .fold(0, |acc, x| acc + x)
        );
    }

    #[test]
    fn scratchcards_cascading_copies_task()
    {
        let string = aoc2023::read_to_string("tests/data/day4a.txt").unwrap();
        let mut lex: logos::Lexer<'_, ScratchcardTokens> = <ScratchcardTokens as logos::Logos>::lexer(&string);

        let scratchcards = Scratchcards::fill_sets(&mut lex).unwrap();

        assert_eq!(
            5625994,
            Scratchcards::cascading_copies(scratchcards)
            .iter()
            .fold(0, |acc, x| acc + x)
        );
    }
}
