//! Module to echo strings

#[cfg(test)]
mod tests;

use clap::{App, Arg, ArgMatches};

/// returns output string from given CLI arguments
pub fn get_string_from_match(matches: &ArgMatches) -> String {
    let strings: Vec<&str> = match matches.values_of("STRING") {
        Some(s) => s.collect(),
        _ => Vec::new(),
    };

    let mut output_str = strings.join(" ");
    match matches.occurrences_of("n") {
        0 => {
            output_str.push('\n');
        }
        _ => {}
    }

    return output_str;
}

/// returns a clap App for echo
pub fn get_app() -> App<'static, 'static> {
    App::new("echo")
        .version("0.0.1")
        .author("Eunsoo Sheen <eunsoo.sheen@gmail.com>")
        .about("Display a line of text")
        .arg(
            Arg::with_name("n")
                .short("n")
                .help("do not output the trailing newline"),
        )
        .arg(Arg::with_name("STRING").multiple(true))
}
