use clap::{App, Arg};
//cargo run -- --config myconfig.conf --input myfile.txt -v
fn main() {
    let matches = App::new("My App")
        .version("1.0")
        .author("John Doe <johndoe@example.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("input")
             .help("Sets the input file to use")
             .required(true)
             .index(1))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Gets a value for input, if supplied by user
    let input = matches.value_of("input").unwrap();
    println!("Using input file: {}", input);

    // Gets the level of verbosity
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Invalid verbosity level"),
    }
}

