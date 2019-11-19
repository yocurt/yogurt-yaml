extern crate argparse;
extern crate libcurt;

use argparse::{ArgumentParser, StoreTrue};
use libcurt::YogurtYaml;
use std::io::{self, Read, Write};

fn pipe_data(curt: YogurtYaml, v2: bool) {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let mut line = String::new();
    let mut results: Vec<libcurt::Result>;

    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 {
            break;
        }

        if v2 {
            results = curt.extract2_clear(&mut line);
        } else {
            results = curt.extract_clear(&mut line);
        }

        for result in results {
            let text = result.get_text();
            writeln!(handle, "- {}", text).unwrap();
        }
    }
}

fn main() {
    let mut pipe = false;
    let mut test = false;
    let mut v2 = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Parse yaml from text");
        ap.refer(&mut pipe).add_option(
            &["-p", "--pipe"],
            StoreTrue,
            "Pipe data through this application",
        );
        ap.refer(&mut test)
            .add_option(&["-t", "--test"], StoreTrue, "Run a test");
        ap.refer(&mut v2)
            .add_option(&["-v", "--v2"], StoreTrue, "Use version 2");
        ap.parse_args_or_exit();
    }

    let curt = YogurtYaml::new(&["ID", "REF", "ADD", "END"]);

    if pipe {
        pipe_data(curt, v2);
    } else if test {
        let test_data = "other stuff ID[Test, \nTestContent: 3] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]ff";
        let results = if v2 {
            curt.extract2(test_data)
        } else {
            curt.extract(test_data)
        };
        for result in results {
            println!("{:?}", &result.get_text());
        }
    }
}
