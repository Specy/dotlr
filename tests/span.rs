mod common;

use {
    dotlr::{
        Grammar,
        Parser,
        Spanned,
        Token,
    },
    std::ops::Deref,
};
fn fmt_expected<'i>(
    tokens: &Vec<(Spanned<Token>, &'i str)>,
    spans: &Vec<(usize, usize, usize)>,
) -> String {
    format!(
        "[Expected -> Got] [Offset expected -> Offset Got] \n{}",
        tokens
            .iter()
            .enumerate()
            .map(|(i, token)| {
                let span = token.0.span();
                format!(
                    "{}:{} -> {}:{} [{} -> {}]({})",
                    spans[i].0,
                    spans[i].1,
                    span.line,
                    span.column,
                    spans[i].2,
                    span.offset,
                    token.1,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn test_spans<'i>(tokens: Vec<(Spanned<Token>, &'i str)>, spans: Vec<(usize, usize, usize)>) {
    for (i, token) in tokens.iter().enumerate() {
        let span = token.0.span();
        assert_eq!(
            span.line,
            spans[i].0,
            "Line mismatch for token {:?}\n{}",
            token.1,
            fmt_expected(&tokens, &spans)
        );
        assert_eq!(
            span.column,
            spans[i].1,
            "Column mismatch for token {:?}\n{}",
            token.1,
            fmt_expected(&tokens, &spans)
        );
        assert_eq!(
            span.offset,
            spans[i].2,
            "Offset mismatch for token {:?}\n{}",
            token.1,
            fmt_expected(&tokens, &spans)
        );
    }
}


#[test]
fn correctly_calculates_span() {
    let grammar = Grammar::parse(common::grammars::CALCULATOR).unwrap();
    let mut parser = Parser::lalr(grammar).unwrap();
    // do not remove the spaces in the string
    let str = "  11 +  221+3
+20
  
    +44 +5";
    let tokens = parser.tokenize(str).unwrap();
    test_spans(tokens, vec![
        (1, 3, 2),
        (1, 6, 5),
        (1, 9, 8),
        (1, 12, 11),
        (1, 13, 12),
        (2, 1, 14),
        (2, 2, 15),
        (4, 5, 25),
        (4, 6, 26),
        (4, 9, 29),
        (4, 10, 30),
        (4, 11, 31),
    ]);
}
