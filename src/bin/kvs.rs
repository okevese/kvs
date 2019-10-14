
extern crate clap;
use clap::{ Arg, App };
use kvs::*;

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .author("Oke Vese <okevese@gmail.com")
        .about("a key value store")
        .arg(
            Arg::with_name("argument")
                .help("some argument to the binary")
        )
        .arg(
            Arg::with_name("set")
            .multiple(true)
            .takes_value(true)
            .help("get a saved key") 
        )
        .get_matches();

    println!("{:?}", matches.usage());
    
    if let Some(arg) = matches.value_of("argument") {
        println!("An arg was passed in: {}", arg);
    } else {
        panic!(2);
    }

    if let Some(key) = matches.value_of("set") {
        println!("unimplemented");
        panic!(2);
    }
}

