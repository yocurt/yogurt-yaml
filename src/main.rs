extern crate libcurt;

use libcurt::YogurtYaml;

fn main() {
    let curt = YogurtYaml::new(vec!["ID", "REF", "ADD", "END"]);
    let results = curt.extract(&"other stuff ID[Test, \nTestContent: 3] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff".to_string());
    for result in results {
        println!("{:?}", &result.get_yaml()[0]);
    }
}
