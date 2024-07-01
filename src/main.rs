use anyhow::Error;
use clap::Parser;
use log::info;

mod utils {
    pub mod fun;
}

#[derive(Parser)]
struct Cli {
    /// Word to search in the file
    pattern: String,
    /// Path to the file to search, using as root, the current directory
    path: std::path::PathBuf,
    /// Progress bar timer
    #[clap(short, long, default_value = "10")]
    timmer: u64,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Starting the magic...");

    let args = Cli::parse();

    utils::fun::progress_bar_with_sleep(args.timmer);
    let _ = utils::fun::search_pattern(args.pattern, args.path);

    info!("Hocus Pocus, programmus terminus!");
    Ok(())
}

// Since println! is super slow, alternative we can use BufWriter and writeln to stdout.
// here is an example on how to lock the stdout and write to it.
/*
   let stdout = io::stdout(); // get the global stdout entity
   let mut handle = stdout.lock(); // acquire a lock on it
   writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
*/