extern crate clap;
use clap::{App, Arg};
use kvs::*;

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .author("Oke Vese <okevese@gmail.com")
        .about("A key value store")
        .subcommand(
            App::new("set")
                .about("Sets new key values")
                .arg(Arg::with_name("key").help("Key to be added").required(true))
                .arg(
                    Arg::with_name("value")
                        .help("Value for the attached key")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("get")
                .about("Gets a stored key value")
                .arg(Arg::with_name("key").help("Key to be added").required(true)),
        )
        .subcommand(
            App::new("rm").about("Deletes a stored key value").arg(
                Arg::with_name("key")
                    .help("Key to be deleted")
                    .required(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_set_matches)) => {
            to_set(_set_matches.value_of("key"), _set_matches.value_of("value"));
        }
        ("get", Some(_get_matches)) => {
            to_get(_get_matches.value_of("key"));
        }
        ("rm", Some(_rm_matches)) => {
            to_remove(_rm_matches.value_of("key"));
        }
        ("", None) => {
            panic!(2);
        }
        _ => unreachable!(),
    }
}

fn to_set(_key: Option<&str>, _value: Option<&str>) {
    match _key {
        Some(_key) => match _value {
            Some(_value) => {
                let mut _store = KvStore::new();
                _store.set(_key.to_owned(), _value.to_owned());
                println!("{:?}", _store);
                _store.set("profession".to_owned(), "programmer".to_owned());
                println!("{:?}", _store);
            }
            None => unimplemented!(),
        },
        None => unimplemented!(),
    }
}

fn to_get(_key: Option<&str>) {
    match _key {
        Some(_key) => {
            let mut _store = KvStore::new();
            match _store.get(_key.to_owned()) {
                Some(key) => println!("{:?}", key),
                None => println!("Not available"),
            }
        }
        None => unimplemented!(),
    }
}

fn to_remove(_key: Option<&str>) {
    match _key {
        Some(_key) => {
            let mut _store = KvStore::new();
            _store.remove(_key.to_owned());
        }
        None => println!("Not available"),
    }
}
