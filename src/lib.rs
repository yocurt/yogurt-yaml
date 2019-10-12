extern crate regex;
extern crate yaml_rust;

use regex::Regex;
use regex::RegexSet;
use yaml_rust::YamlLoader;

pub struct YogurtYaml{
    pairs: Vec<RegexPair>,
    combined_pair: RegexPair,
    regex_set: RegexSet,
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
}

fn create_combined_pair(strs: &Vec<& str>) -> RegexPair {
    let combined_pair_str = format!(r"(?P<ident>{})\[(?P<content>[^\]]*)", "ID");
    return create_pair(&combined_pair_str);
}

fn create_pairs(strs: &Vec<& str>) -> Vec<RegexPair> {
    let mut pairs = Vec::new();
    for s in strs{
        pairs.push(create_pair(s));
    }
    return pairs;
}

fn create_pair(s: &str) -> RegexPair {
    let re_str = format!(r"(?P<ident>{})\[(?P<content>[^\]]*)", s);
    let re = Regex::new(&re_str).unwrap();
    return RegexPair{indicator: s.to_string(),regex: re};
}

fn get_yaml(pairs: Vec<RegexPair>, s: String) -> Vec<yaml_rust::Yaml> {
    let re = Regex::new(r"test").unwrap();
    let mut yaml_str: String = "".to_string();
    for pair in pairs{
        let yaml_str_vec = cut_yaml(pair, &s);
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

fn prettyfy(yaml_vec : Vec<String>) -> String {
    return "Test".to_string();
}

fn cut_yaml(reg: RegexPair, s: &String) -> Vec<String> {
    let mut v = Vec::new();
    for caps in reg.regex.captures_iter(&s){
        let mut yaml_str: String = caps["ident"].to_string();
        yaml_str.push_str(": ");
        yaml_str.push_str(&caps["content"]);
        v.push(yaml_str);
    }
    return v;
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
        let result = cut_yaml(pair, &"ID[Test]".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_cut_yaml_distraction() {
        let pair = create_pair("ID");
        let result = cut_yaml(pair, &"other stuff ID[Test, TestContent: 3] more stuff".to_string());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "ID: Test, TestContent: 3");
    }


    #[test]
    fn test_cut_yaml_multiple_entries() {
        let pair = create_pair("ID");
        let result = cut_yaml(pair, &"other stuff ID[Test, TestContent: 3] more\n ID[Test2, TestContent: 4] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "ID: Test, TestContent: 3");
        assert_eq!(result[1], "ID: Test2, TestContent: 4");
        assert_eq!(result[2], "ID: Test3, TestContent: a7ad");
    }

    #[test]
    fn test_cut_yaml_multiple_lines() {
        let pair = create_pair("ID");
        let result = cut_yaml(pair, &"other stuff ID[Test, \nTestContent: 3] more\n ID[Test2, \nTestContent: 4\n] stuID[Test3, TestContent: a7ad]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "ID: Test, \nTestContent: 3");
        assert_eq!(result[1], "ID: Test2, \nTestContent: 4\n");
        assert_eq!(result[2], "ID: Test3, TestContent: a7ad");
    }

    #[test]
    fn test_cut_yaml_nested() {
        let pair = create_pair("ID");
        let result = cut_yaml(pair, &"other stuff ID[Test, \nTestContent: 3] more\n ID[Test2, \nTestContent: [4]\n] stuID[Test3, TestContent: [[a,7],[a,d]]]ff".to_string());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "ID: Test, \nTestContent: 3");
        assert_eq!(result[1], "ID: Test2, \nTestContent: [4]\n");
        assert_eq!(result[2], "ID: Test3, TestContent: [[a,7],[a,d]]");
    }
}
