use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.to_owned()).collect();
    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };
    println!("{}{}", text.join(" "), ending);
}
