use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use crate::parser::ParseError;

pub(crate) fn get_argument<T: std::str::FromStr>(flag: &str) -> Result<T, ParseError> {
    let args: Vec<String> = env::args().collect();

    for (index, argsv) in args.iter().enumerate() {
        if argsv == flag {
            match args.get(index + 1) {
                Some(argument) => {
                    return argument
                        .parse::<T>()
                        .map_err(|_| ParseError::CouldntParse(argument.clone()))
                }
                None => return Err(ParseError::MissingArgument(flag.to_owned())),
            }
        }
    }

    return Err(ParseError::MissingKey(flag.to_owned()));
}

pub fn get_file(flag: Option<&str>) -> Result<File, String> {
    let flag = flag.unwrap_or("-f");

    let path = get_argument::<PathBuf>(flag).unwrap();

    match File::open(&path) {
        Ok(file) => {
            return Ok(file);
        }
        Err(_) => return Err(format!("File {:?} could not be opened.", path.as_os_str())),
    }
}
