#![doc = include_str!("../README.md")]

mod automaton;
mod errors;
mod grammar;
mod parser;
mod tables;
mod trace;
mod tree;
mod utils;

pub use {
    automaton::{
        Automaton,
        Item,
        State,
    },
    errors::{
        GrammarError,
        ParserError,
        ParsingError,
    },
    grammar::{
        AtomicPattern,
        ConstantToken,
        Grammar,
        RegexToken,
        Rule,
        Symbol,
        Token,
    },
    parser::Parser,
    tables::{
        Action,
        FirstTable,
        FollowTable,
        ParsingTables,
    },
    trace::{
        Step,
        Trace,
    },
    tree::Tree,
    utils::{
        Span,
        Spanned,
    },
};

mod prelude {
    #[cfg(feature = "serde")]
    pub use {
        serde_renamed::Serialize,
        utils::serialize_regex_map,
    };


    #[cfg(feature = "wasm")]
    pub use wasm_bindgen::prelude::*;

    #[cfg(not(target_family = "wasm"))]
    pub use colored::*;
    #[cfg(target_family = "wasm")]
    pub use utils::MockColored;

    pub use {
        super::*,
        indexmap::{
            IndexMap,
            IndexSet,
        },
        itertools::Itertools,
        logos::{
            Lexer,
            Logos,
        },
        prettytable::{
            cell,
            format::{
                FormatBuilder,
                LinePosition,
                LineSeparator,
            },
            row,
            Row,
            Table,
        },
        ptree::TreeBuilder,
        regex::Regex,
        smallvec::{
            smallvec,
            SmallVec,
        },
        smol_str::{
            format_smolstr,
            SmolStr,
        },
        std::{
            self,
            collections::BTreeMap,
            fmt::{
                self,
                Debug,
                Display,
            },
            ops::Deref,
        },
        thiserror::Error,
    };
    #[cfg(feature = "serde")]
    pub use {
        serde_renamed::ser::SerializeMap,
        serde_renamed::Serializer,
    };
}
