use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fs::{self, Metadata};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub extension: Option<String>,
    pub metadata: Metadata,
    pub path: PathBuf,
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
            .map(|idx| filename[idx + 1..].to_ascii_lowercase())
    }

    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    pub fn is_dotfile(&self) -> bool {
        self.name.chars().nth(0) == Some('.')
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
            path,
        })
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for File {}

#[cfg(test)]
mod tests {
    use super::File;
    use std::path::Path;

    #[test]
    fn test_filename() {
        assert_eq!(".", File::filename(Path::new(".")));
        assert_eq!("..", File::filename(Path::new("..")));
        assert_eq!("/", File::filename(Path::new("/")));
        assert_eq!("abcd", File::filename(Path::new("abcd")));
        assert_eq!(".zshrc", File::filename(Path::new(".zshrc")));
        assert_eq!("abcd.efg", File::filename(Path::new("abcd.efg")));
        assert_eq!("abcd.efg", File::filename(Path::new("/var/tmp/abcd.efg")));
        assert_eq!("abcd.efg", File::filename(Path::new("one/two/abcd.efg")));
    }

    #[test]
    fn test_extension() {
        assert_eq!(None, File::extension(Path::new(".")));
        assert_eq!(None, File::extension(Path::new("..")));
        assert_eq!(None, File::extension(Path::new("/")));
        assert_eq!(None, File::extension(Path::new("abcd")));
        assert_eq!(
            Some("zshrc".to_string()),
            File::extension(Path::new(".zshrc"))
        );
        assert_eq!(
            Some("efg".to_string()),
            File::extension(Path::new("abcd.efg"))
        );
        assert_eq!(
            Some("efg".to_string()),
            File::extension(Path::new("/var/tmp/abcd.efg"))
        );
        assert_eq!(
            Some("efg".to_string()),
            File::extension(Path::new("one/two/abcd.efg"))
        );
    }
}
