use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test {
    use crate::grammar;

    #[test]
    fn term() {
        let t = grammar::TermParser::new();
        assert!(t.parse("1").unwrap() == 1);
        assert!(t.parse("(2)").unwrap() == 2);
        assert!(t.parse("((3))").unwrap() == 3);
    }

    #[test]
    fn factor() {
        let t = grammar::FactorParser::new();
        assert!(t.parse("1*2").unwrap() == 2);
        assert!(t.parse("(2)*3").unwrap() == 6);
        assert!(t.parse("((8))/2").unwrap() == 4);
    }

    #[test]
    fn expr() {
        let t = grammar::ExprParser::new();
        assert!(t.parse("1+2").unwrap() == 3);
        assert!(t.parse("(2)*3+1").unwrap() == 7);
        assert!(t.parse("2+((8))/2").unwrap() == 6);
        assert!(t.parse("2+((8+2))/2").unwrap() == 7);
        assert!(t.parse("2+((8*3))/2").unwrap() == 14);
    }
}
