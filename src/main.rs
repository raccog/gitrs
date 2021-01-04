use clap::{App, Arg, SubCommand};

use gitrs;

fn main() {
    // Get command line arguments.
    let matches = App::new("gitrs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ryan Cohen <rcohenprogramming@gmail.com>")
        .about("A rust implementation of some git features.")
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("directory")
                        .help("Sets the directory to initialize a repository in.")
                        .index(1),
                )
                .arg(
                    Arg::with_name("quiet")
                        .help("Only print error and warning messages; all other output will be suppressed.")
                        .short("q")
                        .long("quiet")
                        .takes_value(false),
                )
                .version("0.0.1")
        )
        .subcommand(SubCommand::with_name("hash-object").arg(Arg::with_name("file").index(1)))
        .get_matches();

    // Run subcommand from args.
    let result = match matches.subcommand() {
        ("init", _) => gitrs::init(&matches),
        ("hash-object", _) => gitrs::hash_object(&matches),
        _ => Ok(()),
    };

    if let Err(error) = result {
        eprintln!("{}", error);
    }
}
