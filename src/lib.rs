extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};

// ID[IMPL::yaml-extraction::]
/// Contains identifier checks and results from usage
pub struct YogurtYaml<'a> {
    ident_checks: Vec<IdentChecker<'a>>,
    results: Vec<Result>,
}

/// Results found via extraction from strings
pub struct Result {
    text: String,
    start: usize,
    end: usize,
}

/// Access results via convenient functions
impl Result {
    /// return results as proper yaml string
    pub fn get_text(&self) -> &String {
        &self.text
    }
    /// return results with additional information
    pub fn get_print(&self) -> String {
        let mut result = self.text.clone();
        result.push_str(" at ");
        result.push_str(&self.start.to_string());
        result.push_str(" -> ");
        result.push_str(&self.end.to_string());
        result
    }
    /// return results as vector of yaml struct
    pub fn get_yaml(&self) -> Vec<Yaml> {
        YamlLoader::load_from_str(&self.text).unwrap()
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn new(text: String, start: usize, end: usize) -> Result {
        Result { text, start, end }
    }
}

pub struct Indicators<'a> {
    ident_strings: &'a [&'a str],
    range: IdentRange,
}

impl<'a> Indicators<'a> {
    pub fn new(ident_strings: &'a [&'a str], range: IdentRange) -> Indicators<'a> {
        Indicators {
            ident_strings,
            range,
        }
    }
}

/// Implements YogurtYaml functions
impl<'a> YogurtYaml<'a> {
    /// Create a new curt instance
    pub fn new(indicator_lists: &'a [Indicators]) -> YogurtYaml<'a> {
        let mut ident_checks = Vec::new();
        for indicator_list in indicator_lists {
            ident_checks.extend(create_ident_checks(
                indicator_list.ident_strings,
                indicator_list.range,
            ));
        }
        let results = Vec::new();
        YogurtYaml {
            ident_checks,
            results,
        }
    }

    /// Create a new curt instance
    pub fn new_from_str(indicators: &'a [&'a str]) -> YogurtYaml<'a> {
        let ident_checks = create_ident_checks(indicators, IdentRange::Brackets);
        let results = Vec::new();
        YogurtYaml {
            ident_checks,
            results,
        }
    }

    // ID[IMPL::Multiline_Support, implements: REQ::Multi_Line]
    /// Extract yaml from string
    pub fn curt(&mut self, s: &str) {
        self.results
            .extend(cut_yaml_unchecked(&mut self.ident_checks, s));
    }

    /// Extracts yaml and clears string if not open
    pub fn curt_clear(&mut self, s: &mut String) {
        self.results.extend(cut_yaml(&mut self.ident_checks, s));
        if !self.reset_open() {
            s.clear();
        }
    }

    /// Return results
    pub fn get_results(&self) -> &Vec<Result> {
        &self.results
    }

    /// Clear the list of results
    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    /// Checks whether there is any not `SemanticPosition::Out` containing `ident_check` in the list of `ident_checks`
    pub fn is_open(&self) -> bool {
        for ident_check in &self.ident_checks {
            if ident_check.semantic_position != SemanticPosition::Out {
                return true;
            }
        }
        false
    }

    /// Clears results and resets all `ident_checks`
    pub fn reset(&mut self) {
        for ident_check in &mut self.ident_checks {
            reset(ident_check);
        }
        self.clear_results();
    }

    /// Resets all `ident_checks' and returns according to `self.is_open()`
    pub fn reset_open(&mut self) -> bool {
        let mut result = false;
        for ident_check in &mut self.ident_checks {
            if ident_check.semantic_position != SemanticPosition::Out {
                reset(ident_check);
                result = true;
            }
        }
        result
    }
}

/// Enables extraction of yaml data defined by identifiers and closures
struct IdentChecker<'a> {
    range: IdentRange,
    ident: &'a str,
    first_char: char,
    begin_char: char,
    end_char: char,
    // mut:
    semantic_position: SemanticPosition,
    length: usize,
    closures: i32,
}

#[derive(PartialEq)]
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

fn check_out(ident_check: &mut IdentChecker, c: char) {
    if c == ident_check.first_char {
        ident_check.length = 1;
        ident_check.semantic_position = SemanticPosition::Ident;
    }
}

fn clean_up(ident_check: &mut IdentChecker, c: char) {
    reset(ident_check);
    check_out(ident_check, c); // Could be the start of a ident
}

fn reset(ident_check: &mut IdentChecker) {
    ident_check.length = 0;
    ident_check.closures = 0;
    ident_check.semantic_position = SemanticPosition::Out;
}

fn check_ident(ident_check: &mut IdentChecker, c: char) {
    let check_size = ident_check.ident.len() < ident_check.length;
    if check_size {
        if ident_check.begin_char == c {
            ident_check.semantic_position = SemanticPosition::In;
            ident_check.closures = 1;
        } else {
            clean_up(ident_check, c);
        }
    } else if c
        != ident_check
            .ident
            .chars()
            .nth(ident_check.length - 1)
            .unwrap()
    {
        clean_up(ident_check, c);
    }
}

fn check_ident_tag(ident_check: &mut IdentChecker, c: char) {
    if c == ident_check.begin_char {
        ident_check.semantic_position = SemanticPosition::In;
    } else if c == ' ' || c == '\n' || c == ',' || c == '.' {
        if ident_check.length > 2 {
            ident_check.semantic_position = SemanticPosition::Done;
        } else {
            reset(ident_check);
        }
    } else if c == ident_check.first_char {
        ident_check.length = 1;
        ident_check.semantic_position = SemanticPosition::Ident;
    }
}

fn check_in(ident_check: &mut IdentChecker, c: char) {
    let begin = ident_check.begin_char;
    let end = ident_check.end_char;
    if c == end {
        ident_check.closures -= 1;
        check_end(ident_check);
    } else if c == begin {
        ident_check.closures += 1;
    } else if c == '\'' {
        ident_check.semantic_position = SemanticPosition::InSingleQuote;
    } else if c == '"' {
        ident_check.semantic_position = SemanticPosition::InDoubleQuote;
    }
}

fn check_single_quote(ident_check: &mut IdentChecker, c: char) {
    if c == '\'' {
        ident_check.semantic_position = SemanticPosition::In;
    } else if c == '\\' {
        ident_check.semantic_position = SemanticPosition::InSingleQuoteEscaped;
    }
}

fn check_double_quote(ident_check: &mut IdentChecker, c: char) {
    if c == '"' {
        ident_check.semantic_position = SemanticPosition::In;
    } else if c == '\\' {
        ident_check.semantic_position = SemanticPosition::InDoubleQuoteEscaped;
    }
}

fn check_end(ident_check: &mut IdentChecker) {
    if ident_check.closures == 0 {
        ident_check.semantic_position = SemanticPosition::Done;
    }
}

fn add_result(results: &mut Vec<Result>, ident_check: &mut IdentChecker, s: &str, i: usize) {
    let end = i - 1;
    let length;
    match ident_check.length.checked_sub(2) {
        Some(a) => length = a,
        None => length = 0,
    }; // TODO: Minus 2 is a bit odd ..
    let start = end.checked_sub(length).unwrap();

    let mut text: String = s.chars().skip(start).take(length).collect();
    text = text.replacen(ident_check.begin_char, ": ", 1);
    text.insert(0, '{');
    text.push('}');
    results.push(Result { text, start, end });
}

pub fn cut_yaml_ident_strings(ident_strings: &[&str], s: &str) -> Vec<Result> {
    let mut ident_checks = create_ident_checks(ident_strings, IdentRange::Brackets);
    cut_yaml(&mut ident_checks, s)
}

fn check_ident_checks(ident_checks: &mut Vec<IdentChecker>, s: &str, results: &mut Vec<Result>) {
    for ident_check in ident_checks {
        ident_check.length += 1;
        if ident_check.semantic_position == SemanticPosition::Done
            || (ident_check.range == IdentRange::Tag
                && ident_check.semantic_position != SemanticPosition::Out)
        {
            add_result(results, ident_check, s, s.len());
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum IdentRange {
    Tag,
    Brackets,
    Closures,
    Crickets,
    Rounds,
}

fn create_ident_checks<'a>(ident_strings: &'a [&'a str], range: IdentRange) -> Vec<IdentChecker> {
    let mut ident_checks = Vec::new();
    let begin_char;
    let end_char;

    match range {
        IdentRange::Closures => {
            begin_char = '{';
            end_char = '}';
        }
        IdentRange::Brackets => {
            begin_char = '[';
            end_char = ']';
        }
        IdentRange::Crickets => {
            begin_char = '<';
            end_char = '>';
        }
        IdentRange::Rounds => {
            begin_char = '(';
            end_char = ')';
        }
        IdentRange::Tag => {
            begin_char = ':';
            end_char = '\n';
        }
    }

    for ident in ident_strings {
        ident_checks.push(IdentChecker {
            range,
            ident,
            first_char: ident.chars().nth(0).unwrap(),
            begin_char,
            end_char,
            semantic_position: SemanticPosition::Out,
            length: 0,
            closures: 0,
        });
    }
    ident_checks
}

fn cut_yaml(ident_checks: &mut Vec<IdentChecker>, s: &str) -> Vec<Result> {
    let mut results = cut_yaml_unchecked(ident_checks, s);
    check_ident_checks(ident_checks, s, &mut results);
    results
}

fn cut_yaml_unchecked(ident_checks: &mut Vec<IdentChecker>, s: &str) -> Vec<Result> {
    let mut results = Vec::new();
    for (i, c) in s.chars().enumerate() {
        for ident_check in &mut *ident_checks {
            ident_check.length += 1;
            match ident_check.semantic_position {
                SemanticPosition::Out => {
                    check_out(ident_check, c);
                }
                SemanticPosition::Ident => {
                    if ident_check.range == IdentRange::Tag {
                        check_ident_tag(ident_check, c);
                    } else {
                        check_ident(ident_check, c);
                    }
                }
                SemanticPosition::In => {
                    if c == ident_check.end_char && ident_check.range == IdentRange::Tag {
                        ident_check.semantic_position = SemanticPosition::Done;
                    } else {
                        check_in(ident_check, c);
                    }
                }
                SemanticPosition::InSingleQuote => {
                    check_single_quote(ident_check, c);
                }
                SemanticPosition::InDoubleQuote => {
                    check_double_quote(ident_check, c);
                }
                SemanticPosition::InSingleQuoteEscaped => {
                    ident_check.semantic_position = SemanticPosition::InSingleQuote;
                }
                SemanticPosition::InDoubleQuoteEscaped => {
                    ident_check.semantic_position = SemanticPosition::InDoubleQuote;
                }
                SemanticPosition::Done => {
                    add_result(&mut results, ident_check, s, i);
                    reset(ident_check);
                    check_out(ident_check, c);
                }
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use crate::cut_yaml_ident_strings;

    #[test]
    fn test_cut_yaml() {
        let result = cut_yaml_ident_strings(&["ID"], &"ID[Test]".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_cut_yaml_distraction() {
        let result = cut_yaml_ident_strings(
            &["ID"],
            &"other stuff ID[Test, TestContent: 3] more stuff".to_string(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[0].start, 12);
        assert_eq!(result[0].end, 35);
    }

    #[test]
    fn test_cut_yaml_ident_strings_distraction() {
        let result = cut_yaml_ident_strings(
            &["ID"],
            &"other stuff ID[Test, TestContent: 3] more stuff".to_string(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[0].start, 12);
        assert_eq!(result[0].end, 35);
    }

    #[test]
    fn test_cut_yaml_multiple_entries() {
        let result = cut_yaml_ident_strings(&["ID"], &"other stuff ID[Test, TestContent: 3] more\n ID[Test2, TestContent: 4] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[1].text, "{ID: Test2, TestContent: 4}");
        assert_eq!(result[2].text, "{ID: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_multiple_entries2() {
        let result = cut_yaml_ident_strings(&["ID"], &"other stuff ID[Test, TestContent: 3] more\n ID[Test2, TestContent: 4] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[1].text, "{ID: Test2, TestContent: 4}");
        assert_eq!(result[2].text, "{ID: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_multiple_lines() {
        let result = cut_yaml_ident_strings(&["ID"], &"other stuff ID[Test, \nTestContent: 3] more\n ID[Test2, \nTestContent: 4\n] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, \nTestContent: 3}");
        assert_eq!(result[0].start, 12);
        assert_eq!(result[0].end, 36);
        assert_eq!(result[1].text, "{ID: Test2, \nTestContent: 4\n}");
        assert_eq!(result[2].text, "{ID: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_many_id_multiple_entries() {
        let result = cut_yaml_ident_strings(&["ID", "REF", "ADD"], &"other stuff ID[Test, TestContent: 3] more\n REF[Test, TestContent: 4] stuADD[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, TestContent: 3}");
        assert_eq!(result[1].text, "{REF: Test, TestContent: 4}");
        assert_eq!(result[2].text, "{ADD: Test3, TestContent: a7ad}");
    }

    #[test]
    fn test_cut_yaml_nested() {
        let result = cut_yaml_ident_strings(&["ID", "REF", "ADD"], &"other stuff ID[Test, \nTestContent: 3] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, \nTestContent: 3}");
        assert_eq!(result[1].text, "{REF: Test2, \nTestContent: [4]\n}");
        assert_eq!(result[2].text, "{ADD: Test3, TestContent: [[a,7],[a,d]]}");
    }

    #[test]
    fn test_cut_yaml_escaped() {
        let result =  cut_yaml_ident_strings(&["ID", "REF", "ADD"], &r#"other stuff ID[Test, \nTestContent: ']3]]'] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff"#.to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, r#"{ID: Test, \nTestContent: ']3]]'}"#);
        assert_eq!(result[1].text, r#"{REF: Test2, \nTestContent: [4]\n}"#);
        assert_eq!(
            result[2].text,
            r#"{ADD: Test3, TestContent: [[a,7],[a,d]]}"#
        );
    }

    #[test]
    fn test_cut_yaml_ident_strings_escaped() {
        let result = cut_yaml_ident_strings(&["ID", "REF", "ADD"], &"other stuff ID[Test, \nTestContent: ']3]]'] more\n REF[Test2, \nTestContent: [\"4\"]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "{ID: Test, \nTestContent: ']3]]'}");
        assert_eq!(result[1].text, "{REF: Test2, \nTestContent: [\"4\"]\n}");
        assert_eq!(
            result[2].text,
            r#"{ADD: Test3, TestContent: [[a,7],[a,d]]}"#
        );
    }

    #[test]
    fn test_cut_yaml_ident_strings_fix() {
        let result = cut_yaml_ident_strings(
            &["ID", "REF"],
            &r#"- ID[REQ, caption: "Requirements"]"#.to_string(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, r#"{ID: REQ, caption: "Requirements"}"#);
    }

    use crate::YogurtYaml;
    #[test]
    fn test_curt() {
        let test_data = &mut r#"other stuff ID[Test, \nTestContent: ']3]]'] more\n REF[Test2, \nTestContent: ["4"]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]"#.to_string();
        let mut curt = YogurtYaml::new_from_str(&["ID", "REF", "ADD"]);
        let result = curt.get_results();
        assert_eq!(result.len(), 0);
        curt.curt_clear(test_data);
        let result = curt.get_results();
        assert_eq!(result[0].text, r#"{ID: Test, \nTestContent: ']3]]'}"#);
        assert_eq!(result[1].text, r#"{REF: Test2, \nTestContent: ["4"]\n}"#);
        assert_eq!(
            result[2].text,
            r#"{ADD: Test3, TestContent: [[a,7],[a,d]]}"#
        );
    }

    use crate::IdentRange;
    use crate::Indicators;
    #[test]
    fn test_tags() {
        let test_data =
            &mut "other stuff #Test,\n @more\n\n #Test2 @TestContent: more content\n".to_string();
        let mut indicator_lists = Vec::new();
        indicator_lists.push(Indicators::new(&["#", "@"], IdentRange::Tag));
        let mut curt = YogurtYaml::new(&indicator_lists);
        let result = curt.get_results();
        assert_eq!(result.len(), 0);
        curt.curt_clear(test_data);
        let result = curt.get_results();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].text, r#"{#Test}"#);
        assert_eq!(result[1].text, r#"{@more}"#);
        assert_eq!(result[2].text, r#"{#Test2}"#);
        assert_eq!(result[3].text, r#"{@TestContent:  more content}"#);
    }

    #[test]
    fn test_tags_empty() {
        let test_data =
            &mut "other stuff # Test,\n @ more\n\n ## Test2 @@ TestContent: more content\n"
                .to_string();
        let mut indicator_lists = Vec::new();
        indicator_lists.push(Indicators::new(&["#", "@"], IdentRange::Tag));
        let mut curt = YogurtYaml::new(&indicator_lists);
        curt.curt_clear(test_data);
        let result = curt.get_results();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_curt_aggregate() {
        let test_data_part_a =
            &mut r#"other stuff ID[Test, \nTestContent: ']3]]'] more\n"#.to_string();
        let test_data_part_b = &mut r#"REF[Test2, \nTestContent: ["4"]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff"#.to_string();
        let mut curt = YogurtYaml::new_from_str(&["ID", "REF", "ADD"]);
        let result = curt.get_results();
        assert_eq!(result.len(), 0);
        curt.curt(test_data_part_a);
        let result = curt.get_results();
        assert_eq!(result.len(), 1);
        curt.curt_clear(test_data_part_b);
        let result = curt.get_results();
        assert_eq!(result[0].text, r#"{ID: Test, \nTestContent: ']3]]'}"#);
        assert_eq!(result[1].text, r#"{REF: Test2, \nTestContent: ["4"]\n}"#);
        assert_eq!(
            result[2].text,
            r#"{ADD: Test3, TestContent: [[a,7],[a,d]]}"#
        );
    }

    // ID[TEST_Multiline, tests: RQM_Multiline]
    #[test]
    fn test_curt_aggregate_multiline_id() {
        let test_data_part_a = &mut r#"other stuff ID[Test, \n"#.to_string();
        let test_data_part_b = &mut r#"TestContent: ']3]]'] more\n"#.to_string();
        let test_data_part_c = &mut r#"REF[Test2, \nTestContent: ["4"]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff"#.to_string();
        let mut curt = YogurtYaml::new_from_str(&["ID", "REF", "ADD"]);
        let result = curt.get_results();
        assert_eq!(result.len(), 0);
        curt.curt_clear(test_data_part_a);
        let mut data = test_data_part_a.to_owned() + test_data_part_b;
        curt.curt_clear(&mut data);
        let result = curt.get_results();
        assert_eq!(result.len(), 1);
        curt.curt_clear(test_data_part_c);
        let result = curt.get_results();
        assert_eq!(result[0].text, r#"{ID: Test, \nTestContent: ']3]]'}"#);
        assert_eq!(result[1].text, r#"{REF: Test2, \nTestContent: ["4"]\n}"#);
        assert_eq!(
            result[2].text,
            r#"{ADD: Test3, TestContent: [[a,7],[a,d]]}"#
        );
    }
}
