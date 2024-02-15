use super::*;

pub fn block(p: &mut Parser) {
    if !p.at(LCurly) {
        p.advance_with_error("Miss {");
    } else {
        let m = p.open();
        p.eat(LCurly);
        while !p.at(RCurly) && !p.eof() {
            let kind = p.current();
            match kind {
                SignalKw => {
                    declaration::signal_declaration(p);
                    p.expect(Semicolon);
                }
                VarKw => {
                    declaration::var_declaration(p);
                    p.expect(Semicolon);
                }
                _ => statement::statement(p),
            }
        }

        p.expect(RCurly);

        p.close(m, Block);
    }
}

#[cfg(test)]
mod tests {

    use crate::grammar::entry::Scope;

    use super::*;
    #[test]
    fn parse_block_test() {
        let source = r#"
            {
               var x, y; 
               var (x, y);
               var (x, y) = a + b;
               var a = x, b = y;
               var a = x, b = y;
               
               signal a; 
               signal a, b;
               signal (a, b);
               signal (a, b) = a - b;
               a <== 12 + 1;
               a ==>b;
            }
        "#;
        // let mut parser = Parser::new(source);

        // parser.parse(Scope::Block);

        // let cst = parser.build_tree().ok().unwrap();

        // println!("{:?}", cst);
    }
}
