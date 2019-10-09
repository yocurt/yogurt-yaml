extern crate regex;
extern crate yaml_rust;

use regex::Regex;
use regex::RegexSet;
use yaml_rust::YamlLoader;

pub struct YogurtYaml{
    pairs: Vec<RegexPair>,
    regex_set: RegexSet,
}

struct RegexPair{
    indicator: String,
    regex: Regex,
}

impl YogurtYaml{
    pub fn new(indicators: Vec<& str>) -> YogurtYaml{
        let pairs = create_pairs(&indicators);
        return YogurtYaml{
            regex_set: get_regexset(&pairs),
            pairs: pairs,
        };
    }

    pub fn check(&self, s: &str) -> bool {
        return self.regex_set.is_match(s);
    }
}

fn create_pairs(strs: &Vec<& str>) -> Vec<RegexPair> {
    let mut pairs = Vec::new();
    for s in strs{
        pairs.push(create_pair(s));
    }
    return pairs;
}

fn create_pair(s: &str) -> RegexPair {
    let s = format!(r"{}[(?P<content>.*)]", s); 
    let re = Regex::new(&s).unwrap();
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
        let mut yaml_str: String = reg.indicator.to_string();
        yaml_str.push_str(":");
        yaml_str.push_str(&caps["content"]);
        v.push(yaml_str);
    }
    return v;
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
