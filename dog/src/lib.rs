use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    display_d: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut cnt = 0;
                for line_result in file.lines() {
                    let line = line_result?;
                    cnt += 1;
                    if config.number_lines {
                        println!("{:>6}\t{}", cnt, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!("{}", line);
                            cnt -= 1;
                        } else {
                            println!("{:>6}\t{}", cnt, line);
                        }
                    } else if config.display_d {
                        // line.u
                        println!("{}$", line);
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("dog")
        .version("0.1.0")
        .author("Lior Kaufman")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("display_d")
                .short("e")
                .help(
                    "Display non-print characters, 
                 and display a dollar ($) sign at the end of each line",
                )
                .takes_value(false),
        )
        .about("Terminal application like cat but it's made in rust and called dog")
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
        display_d: matches.is_present("display_d"),
    })
}
