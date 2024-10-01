import { strict as assert } from "node:assert";
import { test } from "node:test";
import { Grammar, LR1Parser, LALR1Parser } from "../src";

// Example grammar
const exampleGrammar = `
P -> E
E -> E '+' T
E -> E '-' T
E -> T
T -> %num
%num -> /[0-9]+/
`;

// Tests for the Grammar class
test("Grammar class tests", async (t) => {
    await t.test(
        "Grammar.parse should parse a valid grammar without errors",
        () => {
            const result = Grammar.parse(exampleGrammar);
            assert.ok(result.isOk());
            assert.ok(result.value instanceof Grammar);
        },
    );

    await t.test("Grammar methods should execute without errors", () => {
        const result = Grammar.parse(exampleGrammar);
        assert.ok(result.isOk());

        const grammar = result.value;

        assert.doesNotThrow(() => {
            grammar.getSymbols();
            grammar.getConstantTokens();
            grammar.getStartSymbol();
            grammar.getProductions();
            grammar.getRegexTokens();
            grammar.stringify();
            grammar.clone();
        });
    });
});

// Tests for the LR1Parser and LALR1Parser classes
test("Parser class tests", async (t) => {
    const grammarResult = Grammar.parse(exampleGrammar);
    assert.ok(grammarResult.isOk());
    const grammar = grammarResult.value;

    // LR1Parser tests
    await t.test(
        "LR1Parser.fromGrammar should create a parser without errors",
        () => {
            const parserResult = LR1Parser.fromGrammar(grammar);
            assert.ok(parserResult.isOk());
            assert.ok(parserResult.value instanceof LR1Parser);
        },
    );

    await t.test("LR1Parser methods should execute without errors", () => {
        const parserResult = LR1Parser.fromGrammar(grammar);
        assert.ok(parserResult.isOk());

        const parser = parserResult.value;

        assert.doesNotThrow(() => {
            parser.parse("3 + 4 - 2");
            parser.tokenize("3 + 4 - 2");
            parser.trace("3 + 4 - 2");
            parser.getActionTable();
            parser.getGotoTable();
            parser.getParseTables();
            parser.getAutomaton();
            parser.getFirstTable();
            parser.getFollowTable();
        });
    });

    // LALR1Parser tests
    await t.test(
        "LALR1Parser.fromGrammar should create a parser without errors",
        () => {
            const parserResult = LALR1Parser.fromGrammar(grammar);
            assert.ok(parserResult.isOk());
            assert.ok(parserResult.value instanceof LALR1Parser);
        },
    );

    await t.test("LALR1Parser methods should execute without errors", () => {
        const parserResult = LALR1Parser.fromGrammar(grammar);
        assert.ok(parserResult.isOk());

        const parser = parserResult.value;

        assert.doesNotThrow(() => {
            parser.parse("3 + 4 - 2");
            parser.tokenize("3 + 4 - 2");
            parser.trace("3 + 4 - 2");
            parser.getActionTable();
            parser.getGotoTable();
            parser.getParseTables();
            parser.getAutomaton();
            parser.getFirstTable();
            parser.getFollowTable();
        });
    });
});
