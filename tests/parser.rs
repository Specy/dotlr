mod common;

use {
    dotlr::{
        Action,
        ConstantToken,
        Grammar,
        Item,
        Parser,
        ParserError,
        Rule,
        State,
        Symbol,
        Token,
    },
    indexmap::{
        IndexMap,
        IndexSet,
    },
    std::ops::Deref,
};


#[test]
fn creating_parser_for_semantically_correct_grammars() {
    for grammar in common::grammars::CORRECT {
        let grammar = Grammar::parse(grammar).unwrap();
        assert!(Parser::new(grammar).is_ok());
    }
}

#[test]
fn failing_to_create_parser_for_semantically_incorrect_grammars() {
    for grammar in common::grammars::SEMANTICALLY_INCORRECT {
        let grammar = Grammar::parse(grammar).unwrap();
        assert!(Parser::new(grammar).is_err());
    }
}


#[test]
fn raising_correct_error_when_creating_parser_for_empty_grammar() {
    let grammar = Grammar::parse(common::grammars::EMPTY).unwrap();
    let error = Parser::new(grammar).unwrap_err();
    assert_eq!(error.to_string(), "grammar is empty");
}

#[test]
fn raising_correct_error_when_creating_parser_for_undefined_symbol_grammar() {
    let grammar = Grammar::parse(common::grammars::UNDEFINED_SYMBOL).unwrap();
    let error = Parser::new(grammar).unwrap_err();
    assert_eq!(error.to_string(), "symbol F in rule S -> E '+' F is not defined");
}

#[test]
fn raising_correct_error_when_creating_parser_for_undefined_regex_token_grammar() {
    let grammar = Grammar::parse(common::grammars::UNDEFINED_REGEX_TOKEN).unwrap();
    let error = Parser::new(grammar).unwrap_err();
    assert_eq!(error.to_string(), "regex token %i in rule E -> %i '+' %i is not defined");
}

#[test]
fn raising_correct_error_when_creating_parser_for_shift_reduce_conflict_grammar() {
    let grammar = Grammar::parse(common::grammars::SHIFT_REDUCE_CONFLICT).unwrap();
    let error = Parser::new(grammar).unwrap_err();

    let error_string = error.to_string();
    if let ParserError::Conflict { parser, token, state } = error {
        assert_eq!(error_string, format!("conflict at state {} on {}", state, token));

        let possible_actions = parser.action_table()[state].get(&token);
        assert!(possible_actions.is_some());

        let mut has_shift_action = false;
        let mut has_reduce_action = false;

        for action in possible_actions.unwrap().iter() {
            match action {
                Action::Shift { .. } => has_shift_action = true,
                Action::Reduce { .. } => has_reduce_action = true,
                _ => {},
            }
        }

        assert!(has_shift_action && has_reduce_action);
    }
}

#[test]
fn raising_correct_error_when_creating_parser_for_reduce_reduce_conflict_grammar() {
    let grammar = Grammar::parse(common::grammars::REDUCE_REDUCE_CONFLICT).unwrap();
    let error = Parser::new(grammar).unwrap_err();

    let error_string = error.to_string();
    if let ParserError::Conflict { parser, token, state } = error {
        assert_eq!(error_string, format!("conflict at state {} on {}", state, token));

        let possible_actions = parser.action_table()[state].get(&token);
        assert!(possible_actions.is_some());

        let mut reduce_action_count = 0;
        for action in possible_actions.unwrap().iter() {
            if let Action::Reduce { .. } = action {
                reduce_action_count += 1
            }
        }

        assert!(reduce_action_count >= 2);
    }
}


#[test]
fn correctly_creating_parser_for_binary_addition_grammar() {
    let grammar = Grammar::parse(common::grammars::BINARY_ADDITION).unwrap();
    let parser = Parser::new(grammar).unwrap();

    assert_eq!(
        parser.grammar().to_string().trim(),
        r#"

E -> E '+' B
E -> B
B -> '0'
B -> '1'

        "#
        .trim()
    );

    let first_table = parser.first_table();
    {
        // +--------+--------------+
        // | Symbol |  First Set   |
        // +--------+--------------+
        // | B      | { '0', '1' } |
        // +--------+--------------+
        // | E      | { '0', '1' } |
        // +--------+--------------+

        #[rustfmt::skip]
        assert_eq!(
            *first_table.deref(),
            [
                (
                    Symbol::from("B"),
                    [
                        ConstantToken::from("0").into(),
                        ConstantToken::from("1").into(),
                    ]
                        .into(),
                ),
                (
                    Symbol::from("E"),
                    [
                        ConstantToken::from("0").into(),
                        ConstantToken::from("1").into(),
                    ]
                        .into(),
                )
            ]
                .into_iter()
                .collect::<IndexMap<_, _>>()
        );
    }

    let follow_table = parser.follow_table();
    {
        // +--------+------------+
        // | Symbol | Follow Set |
        // +--------+------------+
        // | B      | { $, '+' } |
        // +--------+------------+
        // | E      | { $, '+' } |
        // +--------+------------+

        #[rustfmt::skip]
        assert_eq!(
            *follow_table.deref(),
            [
                (
                    Symbol::from("B"),
                    [
                        Token::Eof,
                        ConstantToken::from("+").into(),
                    ]
                        .into(),
                ),
                (
                    Symbol::from("E"),
                    [
                        Token::Eof,
                        ConstantToken::from("+").into(),
                    ]
                        .into(),
                ),
            ]
                .into_iter()
                .collect::<IndexMap<_, _>>()
        );
    }

    let automaton = parser.automaton();
    {
        // +-------+------------------+------------+--------------+
        // | State |      Items       | Lookaheads | Transitions  |
        // +-------+------------------+------------+--------------+
        // | 0     |  E -> . E '+' B  | { $, '+' } |   E   ->  1  |
        // |       |  E -> . B        | { $, '+' } |   B   ->  2  |
        // |       |  B -> . '0'      | { $, '+' } |  '0'  ->  3  |
        // |       |  B -> . '1'      | { $, '+' } |  '1'  ->  4  |
        // +-------+------------------+------------+--------------+
        // | 1     |  E -> E . '+' B  | { $, '+' } |  '+'  ->  5  |
        // +-------+------------------+------------+--------------+
        // | 2     |  E -> B .        | { $, '+' } |              |
        // +-------+------------------+------------+--------------+
        // | 3     |  B -> '0' .      | { $, '+' } |              |
        // +-------+------------------+------------+--------------+
        // | 4     |  B -> '1' .      | { $, '+' } |              |
        // +-------+------------------+------------+--------------+
        // | 5     |  E -> E '+' . B  | { $, '+' } |  '0'  ->  3  |
        // |       |  B -> . '0'      | { $, '+' } |  '1'  ->  4  |
        // |       |  B -> . '1'      | { $, '+' } |   B   ->  6  |
        // +-------+------------------+------------+--------------+
        // | 6     |  E -> E '+' B .  | { $, '+' } |              |
        // +-------+------------------+------------+--------------+

        #[rustfmt::skip]
        assert_eq!(
            automaton.states(),
            [
                // State 0
                State::new(
                    0,
                    [
                        // E -> . E '+' B | { $, '+' }
                        Item::new(
                            Rule::new(
                                "E",
                                [
                                    Symbol::from("E").into(),
                                    ConstantToken::from("+").into(),
                                    Symbol::from("B").into()
                                ]
                            ),
                            0,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                        // E -> . B | { $, '+' }
                        Item::new(
                            Rule::new(
                                "E",
                                [
                                    Symbol::from("B").into()
                                ]
                            ),
                            0,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                        // B -> . '0' | { $, '+' }
                        Item::new(
                            Rule::new(
                                "B",
                                [
                                    ConstantToken::from("0").into()
                                ]
                            ),
                            0,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                        // B -> . '1' | { $, '+' }
                        Item::new(
                            Rule::new(
                                "B",
                                [
                                    ConstantToken::from("1").into()
                                ]
                            ),
                            0,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                    ],
                    [
                        //  E -> 1
                        (Symbol::from("E").into(), 1),
                        //  B -> 2
                        (Symbol::from("B").into(), 2),
                        // '0' -> 3
                        (ConstantToken::from("0").into(), 3),
                        // '1' -> 4
                        (ConstantToken::from("1").into(), 4),
                    ],
                ),

                // State 1
                State::new(
                    1,
                    [
                        // E -> E . '+' B | { $, '+' }
                        Item::new(
                            Rule::new(
                                "E",
                                [
                                    Symbol::from("E").into(),
                                    ConstantToken::from("+").into(),
                                    Symbol::from("B").into()
                                ]
                            ),
                            1,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                    ],
                    [
                        // '+' -> 5
                        (ConstantToken::from("+").into(), 5),
                    ],
                ),

                // State 2
                State::new(
                    2,
                    [
                        // E -> B . | { $, '+' }
                        Item::new(
                            Rule::new(
                                "E",
                                [
                                    Symbol::from("B").into()
                                ]
                            ),
                            1,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                    ],
                    [
                    ],
                ),

                // State 3
                State::new(
                    3,
                    [
                        // B -> '0' . | { $, '+' }
                        Item::new(
                            Rule::new(
                                "B",
                                [
                                    ConstantToken::from("0").into()
                                ]
                            ),
                            1,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                    ],
                    [
                    ],
                ),

                // State 4
                State::new(
                    4,
                    [
                        // B -> '1' . | { $, '+' }
                        Item::new(
                            Rule::new(
                                "B",
                                [
                                    ConstantToken::from("1").into()
                                ]
                            ),
                            1,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                    ],
                    [
                    ],
                ),

                // State 5
                State::new(
                    5,
                    [
                        // E -> E '+' . B | { $, '+' }
                        Item::new(
                            Rule::new(
                                "E",
                                [
                                    Symbol::from("E").into(),
                                    ConstantToken::from("+").into(),
                                    Symbol::from("B").into()
                                ]
                            ),
                            2,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                        // B -> . '0' | { $, '+' }
                        Item::new(
                            Rule::new(
                                "B",
                                [
                                    ConstantToken::from("0").into()
                                ]
                            ),
                            0,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                        // B -> . '1' | { $, '+' }
                        Item::new(
                            Rule::new(
                                "B",
                                [
                                    ConstantToken::from("1").into()
                                ]
                            ),
                            0,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                    ],
                    [
                        // '0'  ->  3
                        (ConstantToken::from("0").into(), 3),
                        // '1'  ->  4
                        (ConstantToken::from("1").into(), 4),
                        //  B   ->  6
                        (Symbol::from("B").into(), 6),
                    ],
                ),

                // State 6
                State::new(
                    6,
                    [
                        // E -> E '+' B . | { $, '+' }
                        Item::new(
                            Rule::new(
                                "E",
                                [
                                    Symbol::from("E").into(),
                                    ConstantToken::from("+").into(),
                                    Symbol::from("B").into()
                                ]
                            ),
                            3,
                            [Token::Eof, ConstantToken::from("+").into()],
                        ),
                    ],
                    [
                    ],
                ),
            ]
        );
    }

    let action_table = parser.action_table();
    {
        // +-------+--------------------------------+--------------+
        // |       |             Action             |     Goto     |
        // | State | ------------------------------ | ------------ |
        // |       |    '+'    '0'    '1'     $     |    E    B    |
        // +-------+--------------------------------+--------------+
        // | 0     |     -     s3     s4      -     |    1    2    |
        // +-------+--------------------------------+--------------+
        // | 1     |    s5      -      -      -     |    -    -    |
        // +-------+--------------------------------+--------------+
        // | 2     |    r2      -      -     a2     |    -    -    |
        // +-------+--------------------------------+--------------+
        // | 3     |    r3      -      -     r3     |    -    -    |
        // +-------+--------------------------------+--------------+
        // | 4     |    r4      -      -     r4     |    -    -    |
        // +-------+--------------------------------+--------------+
        // | 5     |     -     s3     s4      -     |    -    6    |
        // +-------+--------------------------------+--------------+
        // | 6     |    r1      -      -     a1     |    -    -    |
        // +-------+--------------------------------+--------------+

        #[rustfmt::skip]
        assert_eq!(
            action_table,
            [
                // State 0
                IndexMap::<Token, IndexSet<Action>>::from_iter(
                    [
                        (
                            ConstantToken::from("0").into(),
                            IndexSet::from([Action::Shift { next_state: 3 }]),
                        ),
                        (
                            ConstantToken::from("1").into(),
                            IndexSet::from([Action::Shift { next_state: 4 }]),
                        ),
                    ],
                ),
                // State 1
                IndexMap::<Token, IndexSet<Action>>::from(
                    [
                        (
                            ConstantToken::from("+").into(),
                            IndexSet::from([Action::Shift { next_state: 5 }]),
                        ),
                    ],
                ),
                // State 2
                IndexMap::<Token, IndexSet<Action>>::from(
                    [
                        (
                            ConstantToken::from("+").into(),
                            IndexSet::from([Action::Reduce { rule_index: 1 }]),
                        ),
                        (
                            Token::Eof,
                            IndexSet::from([Action::Accept { rule_index: 1 }]),
                        ),
                    ],
                ),
                // State 3
                IndexMap::<Token, IndexSet<Action>>::from(
                    [
                        (
                            ConstantToken::from("+").into(),
                            IndexSet::from([Action::Reduce { rule_index: 2 }]),
                        ),
                        (
                            Token::Eof,
                            IndexSet::from([Action::Reduce { rule_index: 2 }]),
                        ),
                    ],
                ),
                // State 4
                IndexMap::<Token, IndexSet<Action>>::from(
                    [
                        (
                            ConstantToken::from("+").into(),
                            IndexSet::from([Action::Reduce { rule_index: 3 }]),
                        ),
                        (
                            Token::Eof,
                            IndexSet::from([Action::Reduce { rule_index: 3 }]),
                        ),
                    ],
                ),
                // State 5
                IndexMap::<Token, IndexSet<Action>>::from(
                    [
                        (
                            ConstantToken::from("0").into(),
                            IndexSet::from([Action::Shift { next_state: 3 }]),
                        ),
                        (
                            ConstantToken::from("1").into(),
                            IndexSet::from([Action::Shift { next_state: 4 }]),
                        ),
                    ],
                ),
                // State 6
                IndexMap::<Token, IndexSet<Action>>::from(
                    [
                        (
                            ConstantToken::from("+").into(),
                            IndexSet::from([Action::Reduce { rule_index: 0 }]),
                        ),
                        (
                            Token::Eof,
                            IndexSet::from([Action::Accept { rule_index: 0 }]),
                        ),
                    ],
                ),
            ]
        );
    }

    let goto_table = parser.goto_table();
    {
        // +-------+--------------+
        // |       |     Goto     |
        // | State | ------------ |
        // |       |    E    B    |
        // +-------+--------------+
        // | 0     |    1    2    |
        // +-------+--------------+
        // | 1     |    -    -    |
        // +-------+--------------+
        // | 2     |    -    -    |
        // +-------+--------------+
        // | 3     |    -    -    |
        // +-------+--------------+
        // | 4     |    -    -    |
        // +-------+--------------+
        // | 5     |    -    6    |
        // +-------+--------------+
        // | 6     |    -    -    |
        // +-------+--------------+

        #[rustfmt::skip]
        assert_eq!(
            goto_table,
            [
                // State 0
                IndexMap::<Symbol, usize>::from_iter(
                    [
                        (Symbol::from("E"), 1),
                        (Symbol::from("B"), 2),
                    ],
                ),
                // State 1
                IndexMap::<Symbol, usize>::from_iter(
                    [
                    ],
                ),
                // State 2
                IndexMap::<Symbol, usize>::from_iter(
                    [
                    ],
                ),
                // State 3
                IndexMap::<Symbol, usize>::from_iter(
                    [
                    ],
                ),
                // State 4
                IndexMap::<Symbol, usize>::from_iter(
                    [
                    ],
                ),
                // State 5
                IndexMap::<Symbol, usize>::from_iter(
                    [
                        (Symbol::from("B"), 6),
                    ],
                ),
                // State 6
                IndexMap::<Symbol, usize>::from_iter(
                    [
                    ],
                ),
            ]
        );
    }
}
