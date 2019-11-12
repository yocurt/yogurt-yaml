extern crate regex;
extern crate yaml_rust;

use regex::Regex;
use regex::RegexSet;
use yaml_rust::{Yaml, YamlLoader};

// ID[IMPL::yaml-extraction::]
pub struct YogurtYaml<'a> {
    indicators: &'a [&'a str],
    combined_pair: RegexPair,
    regex_set: RegexSet,
}

enum State {
    Open,
    Closed,
    Invalid,
}

pub struct Result {
    text: String,
    state: State,
    start: usize,
    end: usize,
}

struct RegexPair {
    // indicator: String,
    regex: Regex,
}

impl Result {
    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_yaml(&self) -> Vec<Yaml> {
        YamlLoader::load_from_str(&self.text).unwrap()
    }
}

impl<'a> YogurtYaml<'a> {
    pub fn new(indicators: &'a [&'a str]) -> YogurtYaml<'a> {
        let pairs = create_pairs(&indicators);
        let combined_pair = create_combined_pair(&indicators);
        YogurtYaml {
            indicators,
            regex_set: get_regexset(&pairs),
            combined_pair,
        }
    }

    pub fn check(&self, s: &str) -> bool {
        self.regex_set.is_match(s)
    }

    pub fn extract(&self, s: &str) -> Vec<Result> {
        cut_yaml(&self.combined_pair, &s.to_string())
    }

    pub fn extract2(&self, s: &str) -> Vec<Result> {
        cut_yaml_idents(&self.indicators, &s.to_string())
    }

    pub fn extract_clear(&self, s: &mut String) -> Vec<Result> {
        let result = cut_yaml(&self.combined_pair, &s);
        s.clear();
        result
    }

    pub fn extract2_clear(&self, s: &mut String) -> Vec<Result> {
        let result = cut_yaml_idents(&self.indicators, &s.to_string());
        s.clear();
        result
    }

    pub fn verify(_extracts: Vec<Result>) {}

    pub fn combine(_extracts: Vec<Result>) {}
}

fn create_combined_pair(strs: &[&str]) -> RegexPair {
    let mut combined_pair_str: String = "(".to_string();

    for s in strs {
        combined_pair_str.push_str(&s);
        combined_pair_str.push('|');
    }

    combined_pair_str.pop();
    combined_pair_str.push(')');

    create_pair(&combined_pair_str)
}

fn create_pairs(strs: &[&str]) -> Vec<RegexPair> {
    let mut pairs = Vec::new();
    for s in strs {
        pairs.push(create_pair(s));
    }
    pairs
}

// ID[regex, info: "(?P<ident>{})\[(?P<content>[^\]]*)"]
fn create_pair(s: &str) -> RegexPair {
    let re_str = format!(r"(?P<ident>{})\[(?P<content>[^\]]*)", s);
    let re = Regex::new(&re_str).unwrap();
    RegexPair {
        // indicator: s.to_string(),
        regex: re,
    }
}

// fn get_yaml(pairs: Vec<RegexPair>, s: String) -> Vec<yaml_rust::Yaml> {
//     let _re = Regex::new(r"test").unwrap();
//     let mut yaml_str: String = "".to_string();
//     for pair in pairs {
//         let yaml_str_vec = cut_yaml(&pair, &s);
//         yaml_str = prettyfy(yaml_str_vec);
//     }
//     YamlLoader::load_from_str(&yaml_str).unwrap()
// }

fn get_regexset(pairs: &[RegexPair]) -> RegexSet {
    let mut regs = Vec::new();
    for pair in pairs {
        regs.push(pair.regex.to_string());
    }
    RegexSet::new(regs).unwrap()
}

// fn prettyfy(_yaml_vec: Vec<Result>) -> String {
//     "Test".to_string()
// }

fn cut_yaml(reg: &RegexPair, s: &str) -> Vec<Result> {
    let mut v = Vec::new();
    for caps in reg.regex.captures_iter(&s) {
        let mut yaml_str: String = (&caps["ident"]).to_string();
        yaml_str.push_str(": ");
        yaml_str.push_str(&caps["content"]);

        let pos_start = caps.get(0).unwrap().start();
        let pos_end = caps.get(0).unwrap().end();

        let mut result = check_nested(pos_start, &s, yaml_str);

        result.insert(0, '{');
        result.push('}');

        v.push(Result {
            text: result,
            state: State::Closed,
            start: pos_start,
            end: pos_end,
        });
    }
    v
}

struct Identcheck<'a> {
    ident: &'a str,
    first_char: char,
    begin_char: char,
    end_char: char,
    // mut:
    semantic_position: SemanticPosition,
    length: usize,
    closures: i64,
}

enum SemanticPosition {
    Out,
    Ident,
    In,
    InSingleQuote,
    InDoubleQuote,
    InSingleQuoteEscaped,
    InDoubleQuoteEscaped,
    Done,
}

fn check_out(identcheck: &mut Identcheck, c: &char) {
    if *c == identcheck.first_char {
        identcheck.length = 1;
        identcheck.semantic_position = SemanticPosition::Ident;
    }
}

fn clean_up(identcheck: &mut Identcheck, c: &char) {
    reset(identcheck);
    check_out(identcheck, c); // Could be the start of a ident
}

fn reset(identcheck: &mut Identcheck) {
    identcheck.length = 0;
    identcheck.closures = 0;
    identcheck.semantic_position = SemanticPosition::Out;
}

fn check_ident(identcheck: &mut Identcheck, c: &char) {
    let check_size = identcheck.ident.len() < identcheck.length;
    if check_size {
        if identcheck.begin_char == *c {
            identcheck.semantic_position = SemanticPosition::In;
            identcheck.closures = 1;
        } else {
            clean_up(identcheck, c);
        }
    } else if *c != identcheck.ident.chars().nth(identcheck.length - 1).unwrap() {
        clean_up(identcheck, c);
    }
}

fn check_in(identcheck: &mut Identcheck, c: &char) {
    let begin = identcheck.begin_char;
    let end = identcheck.end_char;
    if *c == begin {
        identcheck.closures += 1;
    } else if *c == end {
        identcheck.closures -= 1;
        check_end(identcheck);
    } else if *c == '\'' {
        identcheck.semantic_position = SemanticPosition::InSingleQuote;
    } else if *c == '"' {
        identcheck.semantic_position = SemanticPosition::InDoubleQuote;
    }
}

fn check_single_quote(identcheck: &mut Identcheck, c: &char) {
    if *c == '\'' {
        identcheck.semantic_position = SemanticPosition::In;
    } else if *c == '\\' {
        identcheck.semantic_position = SemanticPosition::InSingleQuoteEscaped;
    }
}

fn check_double_quote(identcheck: &mut Identcheck, c: &char) {
    if *c == '\'' {
        identcheck.semantic_position = SemanticPosition::In;
    } else if *c == '\\' {
        identcheck.semantic_position = SemanticPosition::InDoubleQuoteEscaped;
    }
}

fn check_end(identcheck: &mut Identcheck) {
    if identcheck.closures == 0 {
        identcheck.semantic_position = SemanticPosition::Done;
    }
}

fn cut_yaml_idents(idents: &[&str], s: &str) -> Vec<Result> {
    let mut v = Vec::new();
    let mut identchecks = Vec::new();

    for ident in idents {
        identchecks.push(Identcheck {
            ident,
            first_char: ident.chars().nth(0).unwrap(),
            begin_char: '[',
            end_char: ']',
            semantic_position: SemanticPosition::Out,
            length: 0,
            closures: 0,
        });
    }

    for (i, c) in s.chars().enumerate() {
        for identcheck in &mut identchecks {
            identcheck.length += 1;
            match identcheck.semantic_position {
                SemanticPosition::Out => {
                    check_out(identcheck, &c);
                }
                SemanticPosition::Ident => {
                    check_ident(identcheck, &c);
                }
                SemanticPosition::In => {
                    check_in(identcheck, &c);
                }
                SemanticPosition::InSingleQuote => {
                    check_single_quote(identcheck, &c);
                }
                SemanticPosition::InDoubleQuote => {
                    check_double_quote(identcheck, &c);
                }
                SemanticPosition::InSingleQuoteEscaped => {
                    identcheck.semantic_position = SemanticPosition::InSingleQuote;
                }
                SemanticPosition::InDoubleQuoteEscaped => {
                    identcheck.semantic_position = SemanticPosition::InDoubleQuote;
                }
                _ => {}
            }
            // Check identcheck to be done
            match identcheck.semantic_position {
                SemanticPosition::Done => {
                    let pos_start = i - identcheck.length;
                    let pos_end = i;
                    let mut result_text: String =
                        s.chars().skip(pos_start).take(pos_end - 1).collect();
                    result_text = result_text.replacen(identcheck.begin_char, ": ", 1);
                    result_text.insert(0, '{');
                    result_text.push('}');
                    v.push(Result {
                        text: result_text,
                        state: State::Closed,
                        start: pos_start,
                        end: pos_end,
                    });
                    reset(identcheck);
                }
                _ => {}
            }
        }
    }
    v
}

fn check_nested(pos: usize, s: &str, yaml_str: String) -> String {
    let reg = Regex::new(r#"(\[|\]|'|")"#).unwrap();

    if reg.is_match(&yaml_str) {
        let start = pos;
        let mut len = 0;
        let mut bracket = 0;
        let mut string_open_d = false;
        let mut string_open_s = false;
        let mut escaped = false;

        let split = s.split_at(start).1;
        for c in split.chars() {
            len += 1;
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                if !string_open_s {
                    if !string_open_d {
                        string_open_d = true;
                    } else {
                        string_open_d = false;
                    }
                }
            } else if c == '\'' {
                if !string_open_d {
                    if !string_open_s {
                        string_open_s = true;
                    } else {
                        string_open_s = false;
                    }
                }
            } else if string_open_d || string_open_s {
            } else if c == '[' {
                bracket += 1;
            } else if c == ']' {
                bracket -= 1;
                if bracket == 0 {
                    break;
                }
            }
        }
        let result: String = s.chars().skip(start).take(len - 1).collect();
        return result.replacen("[", ": ", 1);
    }
    yaml_str
}

#[cfg(test)]
mod tests {
    use crate::create_pair;
    #[test]
    fn test_create_pair() {
        let name = "ID";
        let pair = create_pair(name);
        // assert_eq!(name, pair.indicator);
        assert!(pair.regex.to_string().contains(name));
    }

    use crate::cut_yaml;
    use crate::cut_yaml_idents;
    #[test]
    fn test_cut_yaml() {
        let pair = create_pair("ID");
        let result = cut_yaml(&pair, &"ID[Test]".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_cut_yaml2() {
        let result = cut_yaml_idents(&["ID"], &"ID[Test]".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_cut_yaml_distraction() {
        let pair = create_pair("ID");
        let result = cut_yaml(
            &pair,
            &"other stuff ID[Test, TestContent: 3] more stuff".to_string(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        // assert_eq!(result[0].start, 12);
        // assert_eq!(result[0].end, 35);
    }

    #[test]
    fn test_cut_yaml_multiple_entries() {
        let pair = create_pair("ID");
        let result = cut_yaml(&pair, &"other stuff ID[Test, TestContent: 3] more\n ID[Test2, TestContent: 4] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[1].text, "{ID: Test2, TestContent: 4}");
        assert_eq!(result[2].text, "{ID: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_multiple_entries2() {
        let result = cut_yaml_idents(&["ID"], &"other stuff ID[Test, TestContent: 3] more\n ID[Test2, TestContent: 4] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[1].text, "{ID: Test2, TestContent: 4}");
        assert_eq!(result[2].text, "{ID: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_multiple_lines() {
        let pair = create_pair("ID");
        let result = cut_yaml(&pair, &"other stuff ID[Test, \nTestContent: 3] more\n ID[Test2, \nTestContent: 4\n] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, \nTestContent: 3}");
        // assert_eq!(result[0].start, 12);
        // assert_eq!(result[0].end, 36);
        assert_eq!(result[1].text, "{ID: Test2, \nTestContent: 4\n}");
        assert_eq!(result[2].text, "{ID: Test3, TestContent: a7ad}");
    }

    use crate::create_combined_pair;
    #[test]
    fn test_cut_yaml_many_id_multiple_entries() {
        let pair = create_combined_pair(&["ID", "REF", "ADD"]);
        let result = cut_yaml(&pair, &"other stuff ID[Test, TestContent: 3] more\n REF[Test, TestContent: 4] stuADD[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[1].text, "{REF: Test, TestContent: 4}");
        assert_eq!(result[2].text, "{ADD: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_nested() {
        let pair = create_combined_pair(&["ID", "REF", "ADD"]);
        let result = cut_yaml(&pair, &"other stuff ID[Test, \nTestContent: 3] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, \nTestContent: 3}");
        assert_eq!(result[1].text, "{REF: Test2, \nTestContent: [4]\n}");
        assert_eq!(result[2].text, "{ADD: Test3, TestContent: [[a,7],[a,d]]}");
    }

    #[test]
    fn test_cut_yaml_escaped() {
        let pair = create_combined_pair(&["ID", "REF", "ADD"]);
        let result = cut_yaml(&pair, &r#"other stuff ID[Test, \nTestContent: ']3]]'] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff"#.to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, r#"{ID: Test, \nTestContent: ']3]]'}"#);
        assert_eq!(result[1].text, r#"{REF: Test2, \nTestContent: [4]\n}"#);
        assert_eq!(
            result[2].text,
            r#"{ADD: Test3, TestContent: [[a,7],[a,d]]}"#
        );
    }
}
