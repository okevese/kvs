
extern crate clap;
use clap::{ Arg, App };

fn main() {
    let matches = App::new("kvs")
        .version("1.0.0")
        .author("Oke Vese <okevese@gmail.com")
        .about("a key value store")
        .arg(
            Arg::with_name("argument")
                .help("some argument to the binary")
        )
        .get_matches();
    println!("{:?}", matches.usage());

    if let Some(arg) = matches.value_of("argument") {
        println!("An arg was passed in: {}", arg);
    } else {
        panic!(2);
    }
}
