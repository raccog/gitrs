extern crate clap;

use std::error::Error;

use clap::{App, Arg, SubCommand};

use gitrs;

fn main() -> Result<(), Box<dyn Error>> {
    // Get command line arguments.
    let matches = App::new("gitrs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ryan Cohen <rcohenprogramming@gmail.com>")
        .about("A rust implementation of some git features.")
        .subcommand(
            SubCommand::with_name("init").arg(
                Arg::with_name("directory")
                    .help("Sets the directory to initialize a repository in.")
                    .index(1),
            ),
        )
        .subcommand(SubCommand::with_name("hash-object").arg(Arg::with_name("file").index(1)))
        .get_matches();

    // Run subcommand from args.
    if let Err(err) = match matches.subcommand() {
        ("init", _) => gitrs::init(&matches),
        ("hash-object", _) => gitrs::hash_object(&matches),
        _ => Ok(()),
    } {
        return Err(Box::new(err));
    }

    Ok(())
}
