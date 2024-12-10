use clap::{Arg, Command};

pub fn parse_args() -> (String, String, usize) {
    let matches = Command::new("git-diff-highlight")
        .version("1.0")
        .about("Highlights git diffs based on a regex pattern")
        .arg(
            Arg::new("regex")
                .help("The regex pattern to match in the diff")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .help("The path to the repository")
                .default_value("."),
        )
        .arg(
            Arg::new("context_lines")
                .long("conlines")
                .short('l')
                .help("The number of context lines to show")
                .value_parser(clap::value_parser!(usize))
                .default_value("1"),
        )
        .get_matches();

    let regex = matches.get_one::<String>("regex").unwrap().to_string();
    let path = matches.get_one::<String>("path").unwrap().to_string();
    let context_lines = *matches.get_one::<usize>("context_lines").unwrap();

    (regex, path, context_lines)
}
