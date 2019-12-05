extern crate argparse;
extern crate libcurt;

use argparse::ArgumentParser;
use libcurt::YogurtYaml;
use std::io::{self, Read, Write};

/// Uses YogurtYaml to extract yaml from piped data intro standard out
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

        if !curt.reset_open() {
            for result in curt.get_results() {
                writeln!(handle, "- {}", result.get_text()).unwrap();
            }
            curt.clear_results();
            line.clear();
        }
    }
}

/// main function of curt-extract
fn main() {
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Extract yaml from text via pipe e.g. `cat file | curt-extract`");
        ap.parse_args_or_exit();
    }
    pipe_data(YogurtYaml::new_from_str(&["ID", "REF", "ADD", "END"]));
}
