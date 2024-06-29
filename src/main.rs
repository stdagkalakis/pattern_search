use log::info;
use clap::Parser;
use anyhow::Error;

mod utils{
    pub mod fun; 
}

#[derive(Parser)]
struct Cli
{
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Starting the magic...");

    let args = Cli::parse();

    utils::fun::progress_bar_with_sleep();
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