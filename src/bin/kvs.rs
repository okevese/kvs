
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
            to_set(_set_matches.value_of("key"), _set_matches.value_of("value"));
        },
        ("get", Some(_get_matches)) => {
            to_get(_get_matches.value_of("key"));
        },
        ("rm", Some(_rm_matches)) => {
            panic!("unimplemented");
        },
        ("", None) => {
            panic!(2);
        },
        _ => unreachable!(),
    }

    fn to_set(_key: Option<&str>, _value: Option<&str>) {
        match _key {
            Some(_key) => {
                match _value {
                    Some(_value) => {
                        let mut _store = KvStore::new();
                        println!("{:?}", _store);
                        _store.set(_key.to_owned(), _value.to_owned());
                        println!("{:?}", _store);
                        _store.set("profession".to_owned(), "programmer".to_owned());
                        println!("{:?}", _store);
                    },
                    None => unimplemented!()
                }
            },
            None => unimplemented!()
            
        }
    }


    fn to_get(_key: Option<&str>) {
        match _key {
            Some(_key) => {
                unimplemented!()
            },
            None => unimplemented!()
        }
    }

}

