use std::vec;
use std::fmt::Write as FmtWrite;

struct Argument {
    short: String,
    long: String, 
    name: String, 
    help: String, 
}

#[derive(Default)]
pub struct ArgumentParser
{
    desc: String,
    params: Vec<Argument>,
}

impl ArgumentParser {

    pub fn add(&mut self, short: String, long: String, name: String, help: String) {
        self.params.push(Argument{short, long, name, help});
    } 

    pub fn help(&self, args: &Vec<String>) {
        let mut usage = String::new();
        write!(&mut usage, "{} [-h|--help] ", args[0]);
        for param in self.params {
            if param.long.len() > 0 && param.short.len() > 0 {
                write!(&mut usage, "[{}|{}] {} ", param.short, param.long, param.name.to_uppercase());
            }
            else if param.long.len() > 0 {
                write!(&mut usage, "[{}] {} ", param.long, param.name.to_uppercase());
            }
            else {
                write!(&mut usage, "[{}] {} ", param.short, param.name.to_uppercase());
            }
        }
        println!("usage: {}", usage);
        println!("\n{}\n", self.desc);
    }

    pub fn parse(&self, args: &Vec<String>) {
        // check '-h' or '--help' flag exist
        for arg in args {
            if arg == "-h" || arg == "--help" {
                self.help(args);
                return;
            }
        }
    }
}

pub fn print() {
    println!("this is argparse lib");
}
