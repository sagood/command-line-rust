use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use clap::{arg, value_parser, Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Tian Yu <gasnus@gmail.com>")
        .about("Rust head")
        .arg(
            arg!(-n --lines <LINES> "Number of lines")
                .value_parser(value_parser!(usize))
                .default_value("10"),
        )
        .arg(
            arg!(-c --bytes <BYTES> "Number of bytes")
                .value_parser(value_parser!(usize))
                .conflicts_with("lines"),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches
        .try_get_one::<usize>("lines")
        .map_err(|e| format!("illegal line count -- {}", e))?
        .expect("illegal line count!")
        .to_owned();

    let bytes = matches
        .try_get_one::<usize>("bytes")
        .map_err(|e| format!("illegal byte count -- {}", e))?
        .map(|x| x.to_owned());

    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|f| f.to_owned())
        .collect();

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
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
