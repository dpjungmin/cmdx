use cmdx::echo::{get_app, get_string_from_match};

fn main() {
    let matches = get_app().get_matches();
    let output = get_string_from_match(&matches);
    print!("{}", output);
}
