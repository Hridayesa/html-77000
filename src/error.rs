use derive_more::From;
// use serde::{Deserialize, Serialize};
// use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

// #[serde_as]
#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),
    UnexpectedFilename{
        file_name: String,
    },
    DuplicatePoem{
        number: u32
    },
    PathError{
        path: String
    },
    NoPoemsInTheBook{
        number: u32,
    },
    NoTranslationForPoem{
        number: u32,
    },
    CanNotAddLine_PoemHasNoNumber{
        line: String,
    },
    Html{
        html: String,
    },

    // -- Externals
    #[from]
    // #[serde_as(as = "DisplayFromStr")]
    ConfigIo(std::io::Error),

    #[from]
    ConfigToml(toml::de::Error),

    // #[from]
    ParseSelectorErrorKind(String),

    #[from]
    TerraParsingTemplateError(tera::Error),

    #[from]
    Parse(std::num::ParseIntError),
}

// region:    --- Custom

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}

impl <'a> From<scraper::error::SelectorErrorKind<'a>> for Error {
    fn from(val: scraper::error::SelectorErrorKind<'a>) -> Self {
        Self::ParseSelectorErrorKind( format!("{:?}", val ) )
    }
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
