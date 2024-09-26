use chrono::{self, Datelike};
use std::process::Command;

pub struct JournalManager {
    pub rootdir: String,
}

impl JournalManager {
    fn now_jalalidate() -> jalali_date::JalaliDate {
        let time_now = chrono::offset::Local::now();

        jalali_date::to_jalali(
            time_now.day().try_into().unwrap(),
            time_now.month().try_into().unwrap(),
            time_now.year().try_into().unwrap(),
        )
        .expect("Failed to convert gregorian date to jalali")
    }

    fn get_date_str() -> String {
        let jalali_date_now = JournalManager::now_jalalidate();

        format!(
            "{}/{}/{}",
            jalali_date_now.year, jalali_date_now.month, jalali_date_now.day
        )
    }

    fn get_journal_dir(&self) -> String {
        format!("{}/{}", self.rootdir, JournalManager::get_date_str())
    }

    pub fn create_dirs(&self) {
        std::fs::create_dir_all(self.get_journal_dir()).expect("Failed to create folders");
    }

    pub fn new_journal(&self, title: &String) {
        let journal_path = format!("{}/{}.tmp", self.get_journal_dir(), title);
        let editor_status = Command::new("nvim").arg(&journal_path).status();

        match editor_status {
            Ok(status) if status.success() => {
                if std::fs::exists(&journal_path).expect("Can't check the existence of new journal")
                {
                    let new_content = std::fs::read_to_string(&journal_path).unwrap();
                    println!("New content: {}", new_content);
                } else {
                    eprintln!("No new journal to write!");
                }
            }
            Ok(status) => {
                eprintln!("Neovim exited with: {}", status)
            }
            Err(e) => {
                eprintln!("Failed to open neovim: {}", e);
            }
        }
    }
}
