extern crate argparse;
extern crate libcurt;

use argparse::{ArgumentParser, StoreTrue};
use libcurt::YogurtYaml;
use std::io::{self, Read, Write};

fn pipe_data(mut curt: YogurtYaml) {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let mut line = String::new();

    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 {
            break;
        }

        curt.curt(&line);

        if !curt.is_open() {
            for result in curt.get_results() {
                writeln!(handle, "- {}", result.get_text()).unwrap();
            }
            curt.clear_results();
            line.clear();
        }
    }
}

fn main() {
    let mut pipe = false;
    let mut test = false;
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
        ap.parse_args_or_exit();
    }

    let curt = YogurtYaml::new(&["ID", "REF", "ADD", "END"]);

    if pipe {
        pipe_data(curt);
    } else if test {
        let test_data = "other stuff ID[Test, \nTestContent: \"3\"] more\n REF[Test2, \nTestContent: [4]\n] stuADD[Test3, TestContent: [[a,7],[a,d]]]";
        let results = curt.extract(test_data);
        for result in results {
            println!("{:?}", &result.get_text());
        }
    }
}
