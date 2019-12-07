extern crate argparse;
extern crate libcurt;

use argparse::{ArgumentParser, Store};
use libcurt::{IdentRange, Indicators, YogurtYaml};
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
    let mut brackets = String::new();
    let mut closures = String::new();
    let mut crickets = String::new();
    let mut rounds = String::new();
    let mut words = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Extract yaml from text via pipe e.g. `cat file | curt-extract`");
        ap.refer(&mut words).add_option(
            &["--words", "-w"],
            Store,
            "Get words defined by an identifier [NOT YET IMPLEMENTED]",
        );
        ap.refer(&mut brackets).add_option(
            &["--brackets", "-b"],
            Store,
            "Get yaml defined by an identifier and enclosed by brackets: `IDENT[.*]`",
        );
        ap.refer(&mut closures).add_option(
            &["--closures", "-c"],
            Store,
            "Get words defined by an identifier and enclosed by closures: `IDENT{.*}`",
        );
        ap.refer(&mut crickets).add_option(
            &["--crickets", "-i"],
            Store,
            "Get yaml defined by an identifier and enclosed by crickets: `IDENT<.*>`",
        );
        ap.refer(&mut rounds).add_option(
            &["--rounds", "-r"],
            Store,
            "Get words defined by an identifier and enclosed by rounds: `IDENT(.*)`",
        );
        ap.parse_args_or_exit();
    }
    // let idents = words.split_whitespace().collect::<Vec<&str>>();
    // let word_indicators = Indicators::new(&idents, IdentRange::Word);
    let idents = brackets.split_whitespace().collect::<Vec<&str>>();
    let brackets_indicators = Indicators::new(&idents, IdentRange::Brackets);
    let idents = closures.split_whitespace().collect::<Vec<&str>>();
    let closures_indicators = Indicators::new(&idents, IdentRange::Closures);
    let idents = brackets.split_whitespace().collect::<Vec<&str>>();
    let crickets_indicators = Indicators::new(&idents, IdentRange::Crickets);
    let idents = closures.split_whitespace().collect::<Vec<&str>>();
    let rounds_indicators = Indicators::new(&idents, IdentRange::Rounds);
    let mut indicators = Vec::new();
    // indicators.push(word_indicators);
    indicators.push(brackets_indicators);
    indicators.push(closures_indicators);
    indicators.push(crickets_indicators);
    indicators.push(rounds_indicators);
    pipe_data(YogurtYaml::new(&indicators));
}
