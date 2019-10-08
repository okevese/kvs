#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let _yaml = load_yaml!("../../cli.yml");
    let matches = App::from_yaml(_yaml).get_matches();
    
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Print normally...");
        }
    }
}
