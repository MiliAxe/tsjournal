mod args;
mod config;
mod journal;

use journal::JournalManager;
use std::path::PathBuf;

fn main() {
    let args = args::Cli::parse_args();

    match args.cmd {
        args::Commands::New { ref title } => {
            let expanded_default_path = shellexpand::tilde(config::DEFAULT_PATH);
            let default_pathbuf: PathBuf = PathBuf::from(expanded_default_path.as_ref());

            let root_pathbuf = match args.dir {
                Some(ref directory) => directory,
                None => &default_pathbuf,
            };
            let jmanager = JournalManager {
                rootdir: root_pathbuf.clone(),
            };
            jmanager.create_dirs();
            jmanager.new_journal(title);
        }

        args::Commands::Print {} => {
            dbg!("Print journals...");
        }
        args::Commands::Edit {} => {
            dbg!("Edit journals...");
        }
    }

    dbg!(args);
}
