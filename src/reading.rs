use std::{collections::HashSet, fs::{read_dir, OpenOptions}, io::{BufRead, BufReader, Write}, path::PathBuf};

use regex::Regex;

use crate::data::{DataSetting, UlpData};

pub fn reading_folder(path: &PathBuf, word: &Vec<&str>) -> Vec<PathBuf> {

    let mut paths: Vec<PathBuf> = Vec::new();

    if let Ok(reader) = read_dir(path) {
        for entry in reader.filter_map(|x|x.ok()) {
            if entry.path().is_file() && word.iter().any(|x|entry.file_name().to_string_lossy().to_lowercase().contains(x)) {
                paths.push(entry.path().to_path_buf());
            }
            if entry.path().is_dir() {
                let result = reading_folder(&entry.path().to_path_buf(), word);
                for path in result {
                    paths.push(path);
                }
            }
        }
    }
    paths
}

pub fn workdata(paths: &Vec<PathBuf>) -> usize {
    let user_keywords = ["user", "login", "name"];
    let bad_list = ["unknown", "none", "*"];

    let setting = DataSetting {
        email_regex: Regex::new(r"^\S+@\S+\.\S+$").unwrap(),
        login_regex: Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap(),
        number_regex: Regex::new(r"^\+?\d{1,4}?[-.\s]?\(?\d{1,3}?\)?[-.\s]?\d{1,4}[-.\s]?\d{1,4}[-.\s]?\d{1,9}$").unwrap(),
    };

    let mut finded: HashSet<String> = HashSet::new();

    for path in paths {
        if let Ok(file) = OpenOptions::new().read(true).open(path) {
            let reader = BufReader::new(file);
            let buffer: Vec<String> = reader.lines().filter_map(|x| x.ok()).collect();

            let mut counter: Vec<usize> = Vec::new();
            if buffer.len() >= 3 {
                for (index, line) in buffer.iter().enumerate() {
                    if user_keywords.iter().any(|user| line.to_lowercase().contains(user)) {
                        counter.push(index);
                    }
                }

                for &index in &counter {
                    if index > 0 && index + 1 < buffer.len() {
                        let url_parts: Vec<&str> = buffer[index - 1].split_whitespace().collect();
                        let data_parts: Vec<&str> = buffer[index].split_whitespace().collect();
                        let password_parts: Vec<&str> = buffer[index + 1].split_whitespace().collect();

                        if url_parts.len() > 1 && data_parts.len() > 1 && password_parts.len() > 1 {
                            let url = url_parts[1];
                            let data = data_parts[1];
                            let password = password_parts[1];
                            let formdata = UlpData {
                                url: url.to_owned(),
                                data: data.to_owned(),
                                password: password.to_owned(),
                            };
                            if let Some(ulp) = formdata.check(&setting, &bad_list) {
                                finded.insert(ulp);
                            }
                        }
                    }
                }
            }
        }
    }
    write_res(&finded);
    finded.len()
}

pub fn write_res(buffer: &HashSet<String>) {
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).append(true).open("zap.txt") {
        for line in buffer {    
            if let Err(_) = file.write(format!("{}\n", line).as_bytes()) {}
        }
    }
}