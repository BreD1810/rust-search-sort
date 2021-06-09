use clap::arg_enum;
use clap::Result;

pub struct Parameters {
    pub sort: Result<Sort>,
    pub search: Result<Search>,
    pub length: u32,
}

arg_enum!{
    #[derive(Copy, Clone)]
    pub enum Sort {
         Bubble,
         Cocktail,
         Comb,
         Gnome,
         Heap,
         Insertion,
         Merge,
         Quick,
         Selection,
         Shell,
    }
}

arg_enum!{
    #[derive(Copy, Clone)]
    pub enum Search {
        Binary,
        Linear,
    }
}

pub fn parse_parameters() -> Parameters {
    use clap::*;

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
            Arg::with_name("sort-algorithm")
                .long("sorting-algorithm")
                .help("The sorting algorithm to use")
                .case_insensitive(true)
                .takes_value(true)
                .possible_values(&Sort::variants())
                .required_unless("list")
                .conflicts_with("search-algorithm"),
        )
        .arg(
            Arg::with_name("search-algorithm")
                .long("search-algorithm")
                .help("The search algorithm to use")
                .case_insensitive(true)
                .takes_value(true)
                .possible_values(&Search::variants())
                .required_unless("list")
                .conflicts_with("sort-algorithm"),
        ).get_matches();

    if matches.is_present("list") {
        println!("Available sorting algorithms:");
        for algo in &Sort::variants() {
            println!("- {}", algo);
        }
        println!("\nAvailable searching algorithms:");
        for algo in &Search::variants() {
            println!("- {}", algo);
        }
        use std::process;
        process::exit(0);
    }

    Parameters {
        sort: value_t!(matches, "sort-algorithm", Sort),
        search: value_t!(matches, "search-algorithm", Search),
        length: value_t_or_exit!(matches, "length", u32),
    }
}
