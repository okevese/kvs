
extern crate clap;
use clap::{ Arg, App };
use kvs::*;

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .author("Oke Vese <okevese@gmail.com")
        .about("A key value store")
        .subcommand(
            App::new("set")
                .about("Sets new key values")
                .arg(
                    Arg::with_name("key")
                        .help("Key to be added")
                        .required(true)
                )
                .arg(
                    Arg::with_name("value")
                        .help("Value for the attached key")
                        .required(true)
                )
        )
        .subcommand(
            App::new("get")
                .about("Gets a stored key value")
                .arg(
                    Arg::with_name("key")
                        .help("Key to be added")
                        .required(true)
                )
    
        )
        /*
        .arg(
            Arg::with_name("arg")
                .help("some argument to the binary")
        )*/
        .get_matches();

    println!("{:?}", matches.usage());
    /*
    if let Some(arg) = matches.value_of("arg") {
        println!("An arg was passed in: {}", arg);
    } else {
        panic!(2);
    }*/
    if matches.is_present("set") {
        println!("'set' was run.");
    }

    match matches.subcommand_name() {
        Some("set") => println!("unimplemented"),
        Some("get") => println!("unimplemented"),
        _ => panic!()
    }

}

