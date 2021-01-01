extern crate clap;

use std::error::Error;

use clap::{App, Arg, SubCommand};

use gitrs::{self, GitRepo};

fn main() -> Result<(), Box<dyn Error>> {
    // Get command line arguments.
    let args = App::new("gitrs")
        .version("0.1.0")
        .author("Ryan Cohen <rcohenprogramming@gmail.com>")
        .about("A rust implementation of some git features.")
        .subcommand(
            SubCommand::with_name("init").arg(
                Arg::with_name("directory")
                    .help("Sets the directory to initialize a repository in.")
                    .index(1)
                    .default_value("."),
            ),
        )
        //.subcommand(SubCommand::with_name("hash-object").arg(Arg::with_name("file").index(1)))
        .get_matches();

    let repo = GitRepo::from_args(&args)?;

    // Run subcommand from args.
    if let Err(err) = match args.subcommand() {
        ("init", _) => gitrs::init(&repo),
        _ => Ok(()),
    } {
        return Err(Box::new(err));
    }

    Ok(())
}
