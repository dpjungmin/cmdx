use std::convert::TryFrom;
use std::fs::{self, Metadata};
use std::io;
use std::path::{Path, PathBuf};

pub struct File {
    pub name: String,
    pub extension: Option<String>,
    pub metadata: Metadata,
}

impl File {
    pub fn filename(path: &Path) -> String {
        match path.components().next_back() {
            Some(back) => back.as_os_str().to_string_lossy().to_string(),
            None => unreachable!("`path` does not have a last component"),
        }
    }

    pub fn extension(path: &Path) -> Option<String> {
        let filename = path.file_name().map(|f| f.to_string_lossy().to_string())?;

        filename
            .rfind('.')
            .map(|i| filename[i + 1..].to_ascii_lowercase())
    }
}

impl TryFrom<PathBuf> for File {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let name = File::filename(&path);
        let extension = File::extension(&path);
        let metadata = fs::symlink_metadata(&path)?;

        Ok(File {
            name,
            extension,
            metadata,
        })
    }
}

#[cfg(test)]
mod test_filename {
    use super::File;
    use std::path::Path;

    #[test]
    fn dot() {
        assert_eq!(".", File::filename(Path::new(".")));
    }

    #[test]
    fn dotdot() {
        assert_eq!("..", File::filename(Path::new("..")));
    }

    #[test]
    fn dotfile() {
        assert_eq!(".zshrc", File::filename(Path::new(".zshrc")));
    }

    #[test]
    fn root() {
        assert_eq!("/", File::filename(Path::new("/")));
    }

    #[test]
    fn file() {
        assert_eq!("abcd", File::filename(Path::new("abcd")));
    }

    #[test]
    fn with_extension() {
        assert_eq!("abcd.efg", File::filename(Path::new("abcd.efg")));
    }

    #[test]
    fn absolute() {
        assert_eq!("abcd.efg", File::filename(Path::new("/var/tmp/abcd.efg")));
    }

    #[test]
    fn relative() {
        assert_eq!("abcd.efg", File::filename(Path::new("one/two/abcd.efg")));
    }
}

#[cfg(test)]
mod test_extension {
    use super::File;
    use std::path::Path;

    #[test]
    fn dot() {
        assert_eq!(None, File::extension(Path::new(".")));
    }

    #[test]
    fn dotdot() {
        assert_eq!(None, File::extension(Path::new("..")));
    }

    #[test]
    fn dotfile() {
        assert_eq!(
            Some("zshrc".to_string()),
            File::extension(Path::new(".zshrc"))
        );
    }

    #[test]
    fn root() {
        assert_eq!(None, File::extension(Path::new("/")));
    }

    #[test]
    fn none() {
        assert_eq!(None, File::extension(Path::new("abcd")));
    }

    #[test]
    fn some() {
        assert_eq!(
            Some("efg".to_string()),
            File::extension(Path::new("abcd.efg"))
        );
    }

    #[test]
    fn absolute() {
        assert_eq!(
            Some("efg".to_string()),
            File::extension(Path::new("/var/tmp/abcd.efg"))
        );
    }

    #[test]
    fn relative() {
        assert_eq!(
            Some("efg".to_string()),
            File::extension(Path::new("one/two/abcd.efg"))
        );
    }
}
