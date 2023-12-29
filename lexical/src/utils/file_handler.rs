use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, error::Error};

use anyhow::{anyhow, Result};

pub(crate) fn get_argument<T>(flag: &str) -> Result<T>
where
    T: FromStr,
    T::Err: Error + Send + Sync,
{
    let args: Vec<String> = env::args().collect();

    let index = args
        .iter()
        .position(|argsv| argsv == flag)
        .ok_or(anyhow!("missing key or argument for `{flag}`"))?;

    let argument = args
        .get(index + 1)
        .ok_or(anyhow!("missing argument for flag `{flag}`"))?;

    argument
        .parse::<T>()
        .map_err(|e| anyhow!("Could not parse {argument}: {e}"))
}

pub(crate) fn get_file(flag: Option<&str>) -> Result<File> {
    let flag = flag.unwrap_or("-f");
    let path = get_argument::<PathBuf>(flag).unwrap();

    Ok(File::open(&path)?)
}
