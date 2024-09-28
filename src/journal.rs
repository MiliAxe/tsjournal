use chrono::{self, Datelike};
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;
use xor_cryptor::XORCryptor;

pub struct JournalManager {
    pub rootdir: PathBuf,
}

impl JournalManager {
    fn write_encrypted_buffer(file_content: String, path: &PathBuf) {
        let mut password = String::new();

        print!("Enter your journal password: ");
        io::stdout().flush().expect("Failed to flush stdout");

        std::io::stdin()
            .read_line(&mut password)
            .expect("Failed to read password from stdin");

        let buffer = file_content.as_bytes().to_vec();

        let res = XORCryptor::new(&password).expect("Failed to create encryptor");

        let encrypted_buffer = res.encrypt_vec(buffer);

        let mut file = fs::File::create(path).expect("Failed to create journal file");
        file.write_all(encrypted_buffer.as_ref())
            .expect("Failed to write encrypted journal");
    }

    fn write_temp_file(file_content: String) -> NamedTempFile {
        let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create temp file.");

        temp_file
            .write_all(file_content.as_bytes())
            .expect("Failed to write to temp file");

        temp_file
    }

    pub fn edit_journal(&self) {
        let journal_path = self.get_journal_fzf_path();
        let journal_content = JournalManager::decrypt_buffer(&journal_path);
        let temp_unencrypted_journal = JournalManager::write_temp_file(journal_content);
        let temp_journal_path = temp_unencrypted_journal.path().to_owned();

        let editor_status = Command::new("nvim").arg(&temp_journal_path).status();

        match editor_status {
            Ok(status) if status.success() => {
                if std::fs::exists(temp_journal_path)
                    .expect("Can't check the existence of new journal")
                {
                    let new_content = std::fs::read_to_string(&temp_unencrypted_journal).unwrap();
                    JournalManager::write_encrypted_buffer(new_content, &journal_path);
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

    pub fn get_journal_content(&self) -> String {
        let journal_path = self.get_journal_fzf_path();
        JournalManager::decrypt_buffer(&journal_path)
    }

    fn decrypt_buffer(path: &PathBuf) -> String {
        let mut password = String::new();

        print!("Enter your journal password: ");
        io::stdout().flush().expect("Failed to flush stdout");

        std::io::stdin()
            .read_line(&mut password)
            .expect("Failed to read password from stdin");

        let buffer = fs::read(path).expect("Failed to read journal");

        let res = XORCryptor::new(&password).expect("Failed to create encryptor");

        let decrypted_buffer = res.decrypt_vec(buffer);

        String::from_utf8_lossy(&decrypted_buffer).to_string()
    }

    fn get_journal_fzf_path(&self) -> PathBuf {
        let fd_child = Command::new("fd")
            .args([
                "-tf",
                "-e",
                "tsj",
                ".",
                &self.rootdir.clone().into_os_string().into_string().unwrap(),
            ])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start fd process");

        let fd_out = fd_child.stdout.expect("Failed to open fd stdout");

        let fzf_child = Command::new("fzf")
            .stdin(Stdio::from(fd_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start fzf process");

        let output = fzf_child
            .wait_with_output()
            .expect("Failed to wait on sed")
            .stdout;

        PathBuf::from(String::from_utf8(output).unwrap().trim_end())
    }

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

    fn get_journal_dir(&self) -> PathBuf {
        // format!("{}/{}", self.rootdir, JournalManager::get_date_str())
        self.rootdir.join(JournalManager::get_date_str())
    }

    pub fn create_dirs(&self) {
        std::fs::create_dir_all(self.get_journal_dir()).expect("Failed to create folders");
    }

    pub fn new_journal(&self, title: &String) {
        let journal_path = self.get_journal_dir().join(format!("{title}.tsj"));
        let journal_temp_file =
            tempfile::NamedTempFile::new().expect("Failed to create temp journal");
        let journal_temp_path = journal_temp_file.path().to_owned();
        let editor_status = Command::new("nvim").arg(&journal_temp_path).status();

        match editor_status {
            Ok(status) if status.success() => {
                if std::fs::exists(&journal_temp_path)
                    .expect("Can't check the existence of new journal")
                {
                    let new_content = std::fs::read_to_string(&journal_temp_path).unwrap();
                    JournalManager::write_encrypted_buffer(new_content, &journal_path);
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
