use std::env;
use std::fs::File;
use std::path::PathBuf;

fn get_argument(flag: &str) -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    for (index, argsv) in args.iter().enumerate() {
        if argsv == flag {
            match args.get(index + 1) {
                Some(argument) => return Ok(argument.to_string()),
                None => return Err(format!("Empty paramter for flag {}", flag)),
            }
        }
    }

    return Err(format!("Flag {} not found in args.", flag));
}

pub fn get_file() -> Result<File, String> {
    let flag = "-f";

    let path = match get_argument(flag) {
        Ok(param) => PathBuf::from(param),
        Err(e) => {
            panic!("{}", e);
        }
    };

    match File::open(&path) {
        Ok(file) => {
            return Ok(file);
        }
        Err(_) => return Err(format!("File {:?} could not be opened.", path.as_os_str())),
    }
}
