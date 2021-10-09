use cmdx::fs::dir::Dir;
use cmdx::fs::file::File;
use std::convert::TryFrom;
use std::env;
use std::ffi::OsStr;
use std::io::{self, Stdout, Write};
use std::path::PathBuf;
use std::process;

struct Ls<'args> {
    writer: Stdout,
    paths: Vec<&'args OsStr>,
}

impl<'args> Ls<'args> {
    pub fn new(writer: Stdout, paths: Vec<&'args OsStr>) -> Self {
        Ls { writer, paths }
    }

    pub fn run(&self) -> io::Result<()> {
        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for path in &self.paths {
            match File::try_from(PathBuf::from(path)) {
                Ok(file) => {
                    if file.is_dir() {
                        match Dir::try_from(file) {
                            Ok(dir) => dirs.push(dir),
                            Err(e) => eprintln!("{:?}: {}", path, e),
                        }
                    } else {
                        files.push(file);
                    }
                }
                Err(e) => {
                    eprintln!("{:?}: {}", path, e);
                }
            };
        }

        self.display_files(&files)?;
        self.display_dirs(&dirs, !files.is_empty())?;

        Ok(())
    }

    fn display_files(&self, files: &[File]) -> io::Result<()> {
        if files.is_empty() {
            return Ok(());
        }

        for (idx, file) in files.iter().enumerate() {
            if idx < files.len() - 1 {
                writeln!(&self.writer, "{} ", file.name)?;
            } else {
                write!(&self.writer, "{}", file.name)?;
            }
        }

        Ok(())
    }

    fn display_dirs(&self, dirs: &[Dir], displayed_files: bool) -> io::Result<()> {
        if displayed_files {
            write!(&self.writer, "\n\n")?;
        }

        let multiple_displays = displayed_files || dirs.len() > 1;

        for (idx, dir) in dirs.iter().enumerate() {
            if multiple_displays {
                if idx == 0 {
                    writeln!(&self.writer, "{}: ", dir.name)?;
                } else {
                    writeln!(&self.writer, "\n\n{}: ", dir.name)?;
                }
            }

            for file in &dir.contents {
                // @todo: display dotfiles if -a flag is set
                if file.is_dotfile() {
                    continue;
                }
                write!(&self.writer, "{} ", file.name)?;
            }
        }

        writeln!(&self.writer)?;

        Ok(())
    }
}

fn main() {
    // Assuming only paths are passed in from the command line argument (no options)
    // @todo: add support for options
    let args = env::args_os().skip(1).collect::<Vec<_>>();
    let mut paths = args.iter().map(|s| s.as_os_str()).collect::<Vec<_>>();

    // If no path is passed in, use the current directory as the default
    if paths.is_empty() {
        paths = vec![OsStr::new(".")];
    }

    let ls = Ls::new(io::stdout(), paths);

    match ls.run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
