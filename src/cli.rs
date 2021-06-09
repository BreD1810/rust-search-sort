pub struct Parameters {
    pub algorithm: String,
    pub length: u32,
}

pub fn parse_parameters() -> Parameters {
    use clap::*;

    let sorting_algos = &["bubble", "insertion", "selection"];

    let matches = App::new("rust-search-sort")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("list")
                .long("list")
                .help("List the available sorting algorithms"),
        )
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .help("Set the length of the list")
                .default_value("100"),
        )
        .arg(
            Arg::with_name("algorithm")
                .long("sorting-algorithm")
                .help("The sorting algorithm to use")
                .case_insensitive(true)
                .takes_value(true)
                .possible_values(sorting_algos)
                .required_unless("list"),
        )
        .get_matches();

    if matches.is_present("list") {
        println!("Available sorting algorithms:");
        for algo in sorting_algos {
            println!("- {}", algo);
        }
        use std::process;
        process::exit(0);
    }

    Parameters {
        algorithm: matches.value_of("algorithm").unwrap().to_string(),
        length: value_t_or_exit!(matches, "length", u32),
    }
}
