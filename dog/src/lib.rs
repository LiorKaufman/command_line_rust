use clap::{App, Arg};
use std::{error::Error, vec};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("dog")
        .version("0.1.0")
        .author("Lior Kaufman")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .about("like cat but it's made in rust and called dog")
        .get_matches();

    Ok(Config {
        files: vec![String::from("test")],
        number_lines: false,
        number_nonblank_lines: false,
    })
}
