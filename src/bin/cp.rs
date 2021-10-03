use std::env;
use cmdx::argparse::ArgumentParser;

fn main() {
    let parser: ArgumentParser = Default::default();
    let args = env::args().collect();
    parser.parse(&args);
}
