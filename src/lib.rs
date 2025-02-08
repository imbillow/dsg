pub mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test {
    use crate::ast::Expr;
    use crate::grammar;

    #[test]
    fn expr() {
        let t = grammar::ExprParser::new();
        assert_eq!(format!("{:?}", t.parse("1").unwrap()), "1");
        assert_eq!(format!("{:?}", t.parse("(2)").unwrap()), "2");
        assert_eq!(
            format!("{:?}", t.parse("2+((8*3))/2").unwrap()),
            "(2 + ((8 * 3) / 2))"
        );
    }
}
