mod args;
mod journal;

use journal::JournalManager;

fn main() {
    let args = args::Cli::parse_args();

    match args.cmd {
        args::Commands::New { ref title } => {
            let jmanager = JournalManager {
                rootdir: shellexpand::tilde("~/Documents/tsjournals").to_string(),
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
