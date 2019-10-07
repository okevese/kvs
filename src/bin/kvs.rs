#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("../../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

   
}
