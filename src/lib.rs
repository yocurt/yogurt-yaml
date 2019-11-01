extern crate regex;
extern crate yaml_rust;

use regex::Regex;
use regex::RegexSet;
use yaml_rust::YamlLoader;

// ID[IMPL::yaml-extraction::]
pub struct YogurtYaml{
    pairs: Vec<RegexPair>,
    combined_pair: RegexPair,
    regex_set: RegexSet,
}

pub struct Result{
    text: String,
    start: usize,
    end: usize,
}

struct RegexPair{
    indicator: String,
    regex: Regex,
}

impl YogurtYaml{
    pub fn new(indicators: Vec<& str>) -> YogurtYaml{
        let pairs = create_pairs(&indicators);
        let combined_pair = create_combined_pair(&indicators);
        return YogurtYaml{
            regex_set: get_regexset(&pairs),
            pairs: pairs,
            combined_pair: combined_pair,
        };
    }

    pub fn check(&self, s: &str) -> bool {
        return self.regex_set.is_match(s);
    }

    pub fn extract(&self, s: &str) -> Vec<Result> {
        return cut_yaml(&self.combined_pair, &s.to_string());
    }

    pub fn verify(extracts: Vec<Result>) {
        
    }

    pub fn combine(extracts: Vec<Result>) {

    }
}

fn create_combined_pair(strs: &Vec<& str>) -> RegexPair {
    let mut combined_pair_str : String = "(".to_string();

    for s in strs {
        combined_pair_str.push_str(&s.to_string());
        combined_pair_str.push('|');
    }

    combined_pair_str.pop();
    combined_pair_str.push(')');
    
    return create_pair(&combined_pair_str);
}

fn create_pairs(strs: &Vec<& str>) -> Vec<RegexPair> {
    let mut pairs = Vec::new();
    for s in strs{
        pairs.push(create_pair(s));
    }
    return pairs;
}

// ID[regex, info: "(?P<ident>{})\[(?P<content>[^\]]*)"]
fn create_pair(s: &str) -> RegexPair {
    let re_str = format!(r"(?P<ident>{})\[(?P<content>[^\]]*)", s);
    let re = Regex::new(&re_str).unwrap();
    return RegexPair{indicator: s.to_string(),regex: re};
}

fn get_yaml(pairs: Vec<RegexPair>, s: String) -> Vec<yaml_rust::Yaml> {
    let re = Regex::new(r"test").unwrap();
    let mut yaml_str: String = "".to_string();
    for pair in pairs{
        let yaml_str_vec = cut_yaml(&pair, &s);
        yaml_str = prettyfy(yaml_str_vec);
    }
    return YamlLoader::load_from_str(&yaml_str).unwrap();
}

fn get_regexset(pairs: &Vec<RegexPair>) -> RegexSet {
    let mut regs = Vec::new();
    for pair in pairs{
        regs.push(pair.regex.to_string());
    }
    let set = RegexSet::new(regs).unwrap();
    return set;
}

fn prettyfy(yaml_vec : Vec<Result>) -> String {
    return "Test".to_string();
}

fn cut_yaml(reg: &RegexPair, s: &String) -> Vec<Result> {
    let mut v = Vec::new();
    for caps in reg.regex.captures_iter(&s){
        let mut yaml_str: String = caps["ident"].to_string();
        yaml_str.push_str(": ");
        yaml_str.push_str(&caps["content"]);
        let pos_start = caps.get(0).unwrap().start();
        let pos_end = caps.get(0).unwrap().end();

        let result = check_nested(pos_start, &s, yaml_str);

        v.push(Result{text: result, start: pos_start, end: pos_end});
    }
    return v;
}

fn check_nested(pos: usize, s: &String, yaml_str: String) -> String{
    let reg = Regex::new(r#"(\[|\]|'|")"#).unwrap();

    if reg.is_match(&yaml_str) {
        let start = pos;
        let mut len = 0;
        let mut bracket = 0;
        let mut string_open_d = false;
        let mut string_open_s = false;
        let mut escaped = false;

        let split = s.split_at(start).1;
        for c in split.chars(){
            len += 1;
            if escaped {
                escaped = false;
            } else if c == '\\'{
                escaped = true;
            } else if c == '"' {
                if string_open_s == false {
                    if string_open_d == false {
                        string_open_d = true;
                    } else {
                        string_open_d = false;
                    }
                }
            } else if c == '\'' {
                if string_open_d == false {
                    if string_open_s == false {
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
                    break
                }
            }
        }
        let result: String = s.chars().skip(start).take(len - 1).collect();
        return result.replacen("[", ": ", 1);
    }
    return yaml_str;
}


#[cfg(test)]
mod tests {
    use crate::create_pair;
    #[test]
    fn test_create_pair() {
        let name = "ID";
        let pair = create_pair(name);
        assert_eq!(name, pair.indicator);
        assert!(pair.regex.to_string().contains(name));
    }

    use crate::cut_yaml;
    #[test]
    fn test_cut_yaml() {
        let pair = create_pair("ID");
        let result = cut_yaml(&pair, &"ID[Test]".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_cut_yaml_distraction() {
        let pair = create_pair("ID");
        let result = cut_yaml(&pair, &"other stuff ID[Test, TestContent: 3] more stuff".to_string());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "ID: Test, TestContent: 3");
        assert_eq!(result[0].start, 12);
        assert_eq!(result[0].end, 35);
    }


    #[test]
    fn test_cut_yaml_multiple_entries() {
        let pair = create_pair("ID");
        let result = cut_yaml(&pair, &"other stuff ID[Test, TestContent: 3] more\n ID[Test2, TestContent: 4] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "ID: Test, TestContent: 3");
        assert_eq!(result[1].text, "ID: Test2, TestContent: 4");
        assert_eq!(result[2].text, "ID: Test3, TestContent: a7ad");
    }

    #[test]
    fn test_cut_yaml_multiple_lines() {
        let pair = create_pair("ID");
        let result = cut_yaml(&pair, &"other stuff ID[Test, \nTestContent: 3] more\n ID[Test2, \nTestContent: 4\n] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "ID: Test, \nTestContent: 3");
        assert_eq!(result[0].start, 12);
        assert_eq!(result[0].end, 36);
        assert_eq!(result[1].text, "ID: Test2, \nTestContent: 4\n");
        assert_eq!(result[2].text, "ID: Test3, TestContent: a7ad");
    }
    
    use crate::create_combined_pair;
    #[test]
    fn test_cut_yaml_many_id_multiple_entries() {
        let pair = create_combined_pair(&vec!["ID", "REF", "ADD"]);
        let result = cut_yaml(&pair, &"other stuff ID[Test, TestContent: 3] more\n REF[Test, TestContent: 4] stuADD[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "ID: Test, TestContent: 3");
        assert_eq!(result[1].text, "REF: Test, TestContent: 4");
        assert_eq!(result[2].text, "ADD: Test3, TestContent: a7ad");
    }

    #[test]
    fn test_cut_yaml_nested() {
        let pair = create_combined_pair(&vec!["ID", "REF", "ADD"]);
        let result = cut_yaml(&pair, &"other stuff ID[Test, \nTestContent: 3] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, "ID: Test, \nTestContent: 3");
        assert_eq!(result[1].text, "REF: Test2, \nTestContent: [4]\n");
        assert_eq!(result[2].text, "ADD: Test3, TestContent: [[a,7],[a,d]]");
    }

    #[test]
    fn test_cut_yaml_escaped() {
        let pair = create_combined_pair(&vec!["ID", "REF", "ADD"]);
        let result = cut_yaml(&pair, &r#"other stuff ID[Test, \nTestContent: ']3]]'] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff"#.to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].text, r#"ID: Test, \nTestContent: ']3]]'"#);
        assert_eq!(result[1].text, r#"REF: Test2, \nTestContent: [4]\n"#);
        assert_eq!(result[2].text, r#"ADD: Test3, TestContent: [[a,7],[a,d]]"#);
    }
}
