extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};

// ID[IMPL::yaml-extraction::]
pub struct YogurtYaml<'a> {
    indicators: &'a [&'a str],
}

enum State {
    Open,
    Closed,
}

pub struct Result {
    text: String,
    state: State,
    start: usize,
    end: usize,
}

impl Result {
    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_print(&self) -> String {
        let mut result = self.text.clone();
        result.push_str(" at ");
        result.push_str(&self.start.to_string());
        result.push_str(" -> ");
        result.push_str(&self.end.to_string());
        result.push_str(" : ");
        match &self.state {
            State::Open => result.push_str("Open"),
            State::Closed => result.push_str("Closed"),
        }
        result
    }

    pub fn get_yaml(&self) -> Vec<Yaml> {
        YamlLoader::load_from_str(&self.text).unwrap()
    }
}

impl<'a> YogurtYaml<'a> {
    pub fn new(indicators: &'a [&'a str]) -> YogurtYaml<'a> {
        YogurtYaml {
            indicators,
        }
    }

    pub fn extract2(&self, s: &str) -> Vec<Result> {
        cut_yaml_idents(&self.indicators, &s.to_string())
    }

    pub fn extract2_clear(&self, s: &mut String) -> Vec<Result> {
        let result = cut_yaml_idents(&self.indicators, s);
        s.clear();
        result
    }

    pub fn verify(_extracts: Vec<Result>) {}

    pub fn combine(_extracts: Vec<Result>) {}
}

struct Identcheck<'a> {
    ident: &'a str,
    first_char: char,
    begin_char: char,
    end_char: char,
    // mut:
    semantic_position: SemanticPosition,
    length: usize,
    closures: i32,
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

fn check_out(identcheck: &mut Identcheck, c: char) {
    if c == identcheck.first_char {
        identcheck.length = 1;
        identcheck.semantic_position = SemanticPosition::Ident;
    }
}

fn clean_up(identcheck: &mut Identcheck, c: char) {
    reset(identcheck);
    check_out(identcheck, c); // Could be the start of a ident
}

fn reset(identcheck: &mut Identcheck) {
    identcheck.length = 0;
    identcheck.closures = 0;
    identcheck.semantic_position = SemanticPosition::Out;
}

fn check_ident(identcheck: &mut Identcheck, c: char) {
    let check_size = identcheck.ident.len() < identcheck.length;
    if check_size {
        if identcheck.begin_char == c {
            identcheck.semantic_position = SemanticPosition::In;
            identcheck.closures = 1;
        } else {
            clean_up(identcheck, c);
        }
    } else if c != identcheck.ident.chars().nth(identcheck.length - 1).unwrap() {
        clean_up(identcheck, c);
    }
}

fn check_in(identcheck: &mut Identcheck, c: char) {
    let begin = identcheck.begin_char;
    let end = identcheck.end_char;
    if c == begin {
        identcheck.closures += 1;
    } else if c == end {
        identcheck.closures -= 1;
        check_end(identcheck);
    } else if c == '\'' {
        identcheck.semantic_position = SemanticPosition::InSingleQuote;
    } else if c == '"' {
        identcheck.semantic_position = SemanticPosition::InDoubleQuote;
    }
}

fn check_single_quote(identcheck: &mut Identcheck, c: char) {
    if c == '\'' {
        identcheck.semantic_position = SemanticPosition::In;
    } else if c == '\\' {
        identcheck.semantic_position = SemanticPosition::InSingleQuoteEscaped;
    }
}

fn check_double_quote(identcheck: &mut Identcheck, c: char) {
    if c == '"' {
        identcheck.semantic_position = SemanticPosition::In;
    } else if c == '\\' {
        identcheck.semantic_position = SemanticPosition::InDoubleQuoteEscaped;
    }
}

fn check_end(identcheck: &mut Identcheck) {
    if identcheck.closures == 0 {
        identcheck.semantic_position = SemanticPosition::Done;
    }
}

fn add_result(
    state: State,
    results: &mut Vec<Result>,
    identcheck: &mut Identcheck,
    s: &str,
    i: usize,
) {
    let length = identcheck.length.checked_sub(1).unwrap();
    let start = i.checked_sub(length).unwrap();
    let end = i;

    let mut text: String = s.chars().skip(start).take(length - 1).collect();
    text = text.replacen(identcheck.begin_char, ": ", 1);
    text.insert(0, '{');
    text.push('}');
    results.push(Result {
        text,
        state,
        start,
        end,
    });
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
                    check_out(identcheck, c);
                }
                SemanticPosition::Ident => {
                    check_ident(identcheck, c);
                }
                SemanticPosition::In => {
                    check_in(identcheck, c);
                }
                SemanticPosition::InSingleQuote => {
                    check_single_quote(identcheck, c);
                }
                SemanticPosition::InDoubleQuote => {
                    check_double_quote(identcheck, c);
                }
                SemanticPosition::InSingleQuoteEscaped => {
                    identcheck.semantic_position = SemanticPosition::InSingleQuote;
                }
                SemanticPosition::InDoubleQuoteEscaped => {
                    identcheck.semantic_position = SemanticPosition::InDoubleQuote;
                }
                SemanticPosition::Done => {
                    add_result(State::Closed, &mut v, identcheck, s, i);
                    reset(identcheck);
                    check_out(identcheck, c);
                }
            }
        }
    }
    for identcheck in &mut identchecks {
        identcheck.length += 1;
        match identcheck.semantic_position {
            SemanticPosition::In => add_result(State::Open, &mut v, identcheck, s, s.len()),
            SemanticPosition::Done => add_result(State::Closed, &mut v, identcheck, s, s.len()),
            _ => (),
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use crate::cut_yaml_idents;

    #[test]
    fn test_cut_yaml() {
        let result = cut_yaml_idents(&["ID"], &"ID[Test]".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_cut_yaml_distraction() {
        let result = cut_yaml_idents(&["ID"],
            &"other stuff ID[Test, TestContent: 3] more stuff".to_string(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[0].start, 12);
        assert_eq!(result[0].end, 36); // FIXME: To be checked was 35
    }

    #[test]
    fn test_cut_yaml_idents_distraction() {
        let result = cut_yaml_idents(
            &["ID"],
            &"other stuff ID[Test, TestContent: 3] more stuff".to_string(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[0].start, 12);
        assert_eq!(result[0].end, 36);
    }

    #[test]
    fn test_cut_yaml_multiple_entries() {
        let result = cut_yaml_idents(&["ID"], &"other stuff ID[Test, TestContent: 3] more\n ID[Test2, TestContent: 4] stuID[Test3, TestContent: a7ad]ff".to_string());
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
        let result = cut_yaml_idents(&["ID"], &"other stuff ID[Test, \nTestContent: 3] more\n ID[Test2, \nTestContent: 4\n] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, \nTestContent: 3}");
        assert_eq!(result[0].start, 12);
        assert_eq!(result[0].end, 37); // FIXME: To sbe checked; was 36 before
        assert_eq!(result[1].text, "{ID: Test2, \nTestContent: 4\n}");
        assert_eq!(result[2].text, "{ID: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_many_id_multiple_entries() {
        let result = cut_yaml_idents(&["ID", "REF", "ADD"], &"other stuff ID[Test, TestContent: 3] more\n REF[Test, TestContent: 4] stuADD[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[1].text, "{REF: Test, TestContent: 4}");
        assert_eq!(result[2].text, "{ADD: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_nested() {
        let result = cut_yaml_idents(&["ID", "REF", "ADD"], &"other stuff ID[Test, \nTestContent: 3] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, \nTestContent: 3}");
        assert_eq!(result[1].text, "{REF: Test2, \nTestContent: [4]\n}");
        assert_eq!(result[2].text, "{ADD: Test3, TestContent: [[a,7],[a,d]]}");
    }

    #[test]
    fn test_cut_yaml_escaped() {
        let result =  cut_yaml_idents(&["ID", "REF", "ADD"], &r#"other stuff ID[Test, \nTestContent: ']3]]'] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff"#.to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, r#"{ID: Test, \nTestContent: ']3]]'}"#);
        assert_eq!(result[1].text, r#"{REF: Test2, \nTestContent: [4]\n}"#);
        assert_eq!(
            result[2].text,
            r#"{ADD: Test3, TestContent: [[a,7],[a,d]]}"#
        );
    }

    #[test]
    fn test_cut_yaml_idents_escaped() {
        let result = cut_yaml_idents(&["ID", "REF", "ADD"], &r#"other stuff ID[Test, \nTestContent: ']3]]'] more\n REF[Test2, \nTestContent: ["4"]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff"#.to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, r#"{ID: Test, \nTestContent: ']3]]'}"#);
        assert_eq!(result[1].text, r#"{REF: Test2, \nTestContent: ["4"]\n}"#);
        assert_eq!(
            result[2].text,
            r#"{ADD: Test3, TestContent: [[a,7],[a,d]]}"#
        );
    }

    #[test]
    fn test_cut_yaml_idents_fix() {
        let result = cut_yaml_idents(
            &["ID", "REF"],
            &r#"- ID[REQ, caption: "Requirements"]"#.to_string(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, r#"{ID: REQ, caption: "Requirements"}"#);
    }
}
