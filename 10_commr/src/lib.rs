use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .version("0.1.0")
        .author("Tian Yu <gasnus@gmail.com>")
        .about("Rust grep")
        .arg(
            Arg::new("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .required(true),
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true),
        )
        .arg(
            Arg::new("suppress_col1")
                .short('1')
                .help("Suppress printing of column 1")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("suppress_col2")
                .short('2')
                .help("Suppress printing of column 2")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("suppress_col3")
                .short('3')
                .help("Suppress printing of column 3")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .help("Case-insensitive comparison of lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("delimiter")
                .value_name("DELIM")
                .short('d')
                .long("output-delimiter")
                .help("Output delimiter")
                .default_value("\t"),
        )
        .get_matches();

    let file1 = matches.get_one::<String>("file1").unwrap().to_string();
    let file2 = matches.get_one::<String>("file2").unwrap().to_string();
    let show_col1 = !matches.get_flag("suppress_col1");
    let show_col2 = !matches.get_flag("suppress_col2");
    let show_col3 = !matches.get_flag("suppress_col3");
    let insensitive = matches.get_flag("insensitive");
    let delimiter = matches.get_one::<String>("delimiter").unwrap().to_string();

    Ok(Config {
        file1,
        file2,
        show_col1,
        show_col2,
        show_col3,
        insensitive,
        delimiter,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Column::Col1(val) => {
                if config.show_col1 {
                    columns.push(val);
                }
            }
            Column::Col2(val) => {
                if config.show_col2 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Column::Col3(val) => {
                if config.show_col3 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    if config.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };

        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };

    let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                std::cmp::Ordering::Equal => {
                    print(Column::Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                std::cmp::Ordering::Less => {
                    print(Column::Col1(val1));
                    line1 = lines1.next();
                }
                std::cmp::Ordering::Greater => {
                    print(Column::Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                print(Column::Col1(val1));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                print(Column::Col2(val2));
                line2 = lines2.next();
            }
            _ => (),
        };
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}
