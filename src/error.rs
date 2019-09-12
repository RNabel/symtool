pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Goblin(goblin::error::Error),
    Malformed(String),
    ReplaceString {
        original: String,
        replacement: String,
    },
    StrtabAccess,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::Goblin(e) => write!(f, "{}", e),
            Self::Malformed(s) => write!(f, "{}", s),
            Self::ReplaceString { original, replacement } => write!(
                f,
                "Replacement string (\"{}\") must be the same size or smaller than the original (\"{}\")", replacement, original),
            Self::StrtabAccess => write!(f, "Offset not in range of strtab"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Goblin(e) => Some(e),
            _ => None,
        }
    }
}

impl From<goblin::error::Error> for Error {
    fn from(err: goblin::error::Error) -> Self {
        Self::Goblin(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
