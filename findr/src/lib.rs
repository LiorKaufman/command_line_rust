#![allow(warnings)]
use crate::EntryType::*;
use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    use clap::{Arg, ArgGroup, Command};

    let matches = Command::new("findr")
        .version("0.1.0")
        .author("Lior Kaufman")
        .about("Rust find")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n') // changed from `.short("n")`
                .long("name")
                .help("Name")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .help("Entry type")
                .value_parser(["f", "d", "l"]),
        )
        .get_matches();

    // let names = matches
    //     .values_of_lossy("names")
    //     .map(|vals| {
    //         vals.into_iter()
    //             .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
    //             .collect::<Result<Vec<_>, _>>()
    //     })
    //     .transpose()?
    //     .unwrap_or_default();

    // Clap should disallow anything but "d," "f," or "l"
    let entry_types = matches
        .get_many::<String>("types")
        .map(|vals| {
            vals.into_iter()
                .map(|val| match val.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();

    let paths = matches
        .get_many::<String>("paths")
        .unwrap_or_default()
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();
    let names: Vec<Regex> = matches
        .get_many::<String>("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    Ok(Config {
        paths: paths, // entry_types,
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:?}", config);
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
}
