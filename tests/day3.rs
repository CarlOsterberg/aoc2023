use aoc2023;


#[cfg(test)]
mod day3tests {
    use aoc2023::day3::{*};

    use super::*;

    #[test]
    fn parse_to_tokens_from_example()
    {
        let string = aoc2023::read_to_string("tests/data/day3example.txt").unwrap();
        let mut lex: logos::Lexer<'_, GearRatios> = <GearRatios as logos::Logos>::lexer(&string);

        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(467))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(114))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(10..11))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Character(14..15))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(21..22))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(35))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(633))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(32..33))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Character(39..40))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(43..44))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(617))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::Character(47..48))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(54..55))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Character(60..61))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(58))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(65..66))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(592))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(76..77))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(755))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(87..88))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Character(91..92))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::Character(93..94))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(98..99))));

        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(664))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::Number(598))));
        assert_eq!(lex.next(), Some(Ok(GearRatios::NewLine(109..110))));

        assert_eq!(lex.next(), None);

    }

    #[test]
    fn tokens_to_data_structure()
    {
        let string = aoc2023::read_to_string("tests/data/day3example.txt").unwrap();
        let mut lex: logos::Lexer<'_, GearRatios> = <GearRatios as logos::Logos>::lexer(&string);

        let res = tokens_to_schematic(&mut lex);
        assert!(res.is_ok());

        let schematic = res.unwrap();

        assert_eq!(4361, schematic.get_parts_with_adjacent_symbol().iter().fold(0, |acc, x| acc + x.get_number()))
    }

    #[test]
    fn tokens_to_data_structure_task()
    {
        let string = aoc2023::read_to_string("tests/data/day3a.txt").unwrap();
        let mut lex: logos::Lexer<'_, GearRatios> = <GearRatios as logos::Logos>::lexer(&string);

        let res = tokens_to_schematic(&mut lex);
        assert!(res.is_ok());

        let schematic = res.unwrap();

        assert_eq!(521_601, schematic.get_parts_with_adjacent_symbol().iter().fold(0, |acc, x| acc + x.get_number()))
    }

    #[test]
    fn tokens_to_gear_ratios()
    {
        let string = aoc2023::read_to_string("tests/data/day3example.txt").unwrap();
        let mut lex: logos::Lexer<'_, GearRatios> = <GearRatios as logos::Logos>::lexer(&string);

        let res = tokens_to_schematic(&mut lex);
        assert!(res.is_ok());

        let schematic = res.unwrap();

        assert_eq!(467835, schematic.get_gear_ratios().iter().fold(0, |acc, x| acc + x));
    }

    #[test]
    fn tokens_to_gear_ratios_task()
    {
        let string = aoc2023::read_to_string("tests/data/day3a.txt").unwrap();
        let mut lex: logos::Lexer<'_, GearRatios> = <GearRatios as logos::Logos>::lexer(&string);

        let res = tokens_to_schematic(&mut lex);
        assert!(res.is_ok());

        let schematic = res.unwrap();

        assert_eq!(80694070, schematic.get_gear_ratios().iter().fold(0, |acc, x| acc + x));
    }
}
