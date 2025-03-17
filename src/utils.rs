use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use regex::Regex;
use std::ops::Deref;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use crate::{Error,Result};

lazy_static! {
    static ref RE_DD: Regex = Regex::new(r"\d\d").unwrap();
    static ref RE_SPEC:Regex = Regex::new(r"&.*?;").unwrap();
    static ref RE_NON_DIGIT:Regex = Regex::new(r"\D+").unwrap();
    static ref RE_TAGS:Regex = Regex::new(r"<.*?>").unwrap();
}

pub fn join_file_path(base_dir_name: &str, file_name: &str) -> PathBuf {
    Path::new(base_dir_name).join(file_name)
}
pub fn init_logger() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");    // a builder for `FmtSubscriber`.
}

pub fn path_2_str<'a>(pb: &'a PathBuf)->Result<&'a str>{
    Ok(
        pb.file_name().map(|s|s.to_str()).flatten()
            .ok_or_else(|| 
                Error::PathError{
                    path: pb.as_os_str().to_string_lossy().into()
                })?
    )
}

/// "01&#160;000." -> 1000
/// "03,456." -> 3456
pub fn parse_poem_num_impl(str: &str) -> Result<u32> {
    let r1 = RE_SPEC.replace_all(str, "");
    let binding = RE_NON_DIGIT.replace_all(r1.deref(), "");
    let x = binding.deref();
    // let len = str.len();
    // let x = format!("{}{}",str[0..2].to_string(), str[len - 4..len - 1].to_string());
    Ok(x.parse::<u32>()?)
}

pub fn re_remove_tags(line: &str) -> String {
    RE_TAGS.replace_all(line, "").deref().to_string()
}

/// "Vol.07.html" -> 7
pub fn parse_book_num(name: &str) -> Result<u32> {
    match RE_DD.find(name) {
        Some(m) => { 
            Ok(m.as_str().parse::<u32>()?) 
        }
        None => { 
            Err(Error::UnexpectedFilename{ file_name: name.into() })
         }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_poem_num() -> Result<()> {
        assert_eq!(3456, parse_poem_num_impl("03,456.")?);
        assert_eq!(1000, parse_poem_num_impl("01&#160;000.")?);
        Ok(())
    }
}