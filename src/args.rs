use clap::{Arg, Command};

pub fn parse_args() -> (String, String) {
    let matches = Command::new("git-diff-highlight")
        .version("1.0")
        .about("Highlights git diffs based on a regex pattern")
        .arg(
            Arg::new("regex")
                .long("regex")
                .short('r')
                .help("The regex pattern to match in the diff")
                .required(true),
        )
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .help("The path to the repository")
                .default_value("."),
        )
        .get_matches();

    // Specify the expected type explicitly with `::<&str>`
    let regex = matches.get_one::<String>("regex").unwrap().to_string();
    let path = matches.get_one::<String>("path").unwrap().to_string();

    (regex, path)
}

