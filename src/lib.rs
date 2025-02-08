use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test {
    use crate::grammar;

    #[test]
    fn test() {
        let t = grammar::TermParser::new();
        assert!(t.parse("1").unwrap() == 1);
        assert!(t.parse("(2)").unwrap() == 2);
        assert!(t.parse("((3))").unwrap() == 3);
    }
}
