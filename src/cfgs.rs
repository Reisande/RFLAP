use earlgrey;
use earlgrey::{EarleyParser, Grammar, GrammarBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize)]
struct CfgJson {
    // terminals: lowercase letters or numbers
    terminals: HashSet<char>,
    // start_nonterminal: char,
    non_terminals: HashSet<char>,
    // start nonterminal is 'S'
    // non_terminals are capital characters
    // productions are 2d because there can more than one rule for a non_terminal,
    // and each rule is a series of characters corresponding to non-terminals and terminals
    productions: HashMap<char, Vec<Vec<char>>>,
    tests: Vec<String>,
}

impl CfgJson {
    // should not really be ca;led outside of tests. The method which should create CfgJsons is the derived
    // serde_json::de::from_str
    pub fn new(
        t: HashSet<char>,
        n_t: HashSet<char>,
        p: HashMap<char, Vec<Vec<char>>>,
        ts: Vec<String>,
    ) -> CfgJson {
        // should probably add a check for validity of automaton, or maybe it
        // should be done client side
        CfgJson {
            terminals: t,
            non_terminals: n_t,
            productions: p,
            tests: ts,
        }
    }

    // validate CfgJson
    pub fn validate_cfg_json(&self) -> std::result::Result<(), &'static str> {
        for (terminal, productions) in &self.productions {
            // checks to make sure each production in each rule is made up of valid characters
            if !self.non_terminals.contains(terminal.borrow()) {
                return Err("One of the productions is not actually in the list of non terminals");
            }

            for rule in productions {
                for character in rule {
                    if !self.non_terminals.contains(terminal.borrow())
                        && !self.terminals.contains(character.borrow())
                    {
                        return Err("One of the characters inside of a listed rule is not a terminal or non-terminal");
                    }
                }
            }
        }

        if !self.non_terminals.contains('S'.borrow()) {
            return Err("Does not contain \"S\" as a non-terminal");
        }

        Ok(())
    }

    pub fn create_grammar(&self) -> std::result::Result<Grammar, String> {
        let mut g = earlgrey::GrammarBuilder::default();

        let mut production_strings: HashMap<char, String> = HashMap::new();

        // TODO: this code is disgusting and horribly inefficient, change the char in the CFG
        // TODO: type to be a string and check all invariants
        for non_terminal in self.non_terminals.iter() {
            let mut char_string = [0; 4];

            let non_terminal_string = non_terminal.encode_utf8(&mut char_string).to_string();
            production_strings.insert(*non_terminal, non_terminal_string.to_owned());

            g = g.nonterm(non_terminal_string);
        }

        for terminal in self.terminals.iter() {
            let mut char_string = [0; 4];

            let terminal_string = terminal.encode_utf8(&mut char_string).to_string();
            production_strings.insert(*terminal, terminal_string.to_owned());

            g = g.terminal(terminal_string.to_owned(), move |c| c == terminal_string);
        }

        for (production_name, rules) in self.productions.iter() {
            // go through and create a slice which consists of the strings which make up the rules vec

            for rule in rules {
                let string_vec: Vec<&str> = rule
                    .iter()
                    .map(|c| production_strings.get(c).unwrap().as_str().clone())
                    .collect();

                g = g.rule(
                    production_strings.get(production_name).unwrap().as_str(),
                    string_vec.as_slice(),
                );
            }
        }

        g.into_grammar("S")
    }

    // validate strings
    // The error type means that a string fails to parse, and should be rejected, the ParseTrees
    // are just in case we want to show them later, but are unused for now
    pub fn validate_strings(&self, grammar: earlgrey::Grammar) -> Vec<(String, bool)> {
        let mut return_vec: Vec<(String, bool)> = Vec::new();

        let parser: EarleyParser = earlgrey::EarleyParser::new(grammar);

        for test in &self.tests {
            return_vec.push((
                test.to_owned(),
                match parser.parse(test.split_whitespace()) {
                    Ok(_) => true,
                    Err(_) => false,
                },
            ));
        }

        return_vec
    }

    // check Chomsky Normal form
    pub fn check_chomsky_normal_form(&self) -> std::result::Result<(), &'static str> {
        for (_, rules) in self.productions {
            for rule in rules {
                if rule.len() > 2 {
                    return Err("One of your rules has more than two symbols");
                }

                for symbol in rule {
                    if rule.len() == 2 {
                        // both symbol have to be non-terminals
                        if !self.non_terminals.contains(rule[0].borrow())
                            && !self.non_terminals.contains(rule[1].borrow())
                        {
                            return Err("One of your rules has a non-terminal and a terminal, or two non terminals");
                        }
                    } else if rule.len() == 1 {
                        if !self.terminals.contains(rule[0].borrow()) {
                            return Err("One of your rules has only one symbol, and that one symbol is not a terminal");
                        }
                    } else if rule.len() == 0 {
                        return Err("One of your rules is empty. This should not happen; please report this error on Piazza");
                    }
                }
            }
        }

        Ok(())
    }
}
