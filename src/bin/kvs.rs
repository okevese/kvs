
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
        .subcommand(
            App::new("rm")
                .about("Deletes a stored key value")
                .arg(
                    Arg::with_name("key")
                        .help("Key to be deleted")
                        .required(true)
                )
    
        )
        .get_matches();

    println!("{:?}", matches.usage());

    match matches.subcommand() {
        ("set", Some(_set_matches)) => {
            panic!("unimplemented");
        },
        ("get", Some(_get_matches)) => {
            panic!("unimplemented");
        },
        ("rm", Some(_rm_matches)) => {
            panic!("unimplemented");
        },
        ("", None) => {
            panic!(2);
        },
        _ => unreachable!(),
    }

}

