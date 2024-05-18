use regex::Regex;

#[derive(Clone)]
pub struct UlpData {
    pub url: String,
    pub data: String,
    pub password: String,
}

pub struct DataSetting {
    pub email_regex: Regex,
    pub login_regex: Regex,
    pub number_regex: Regex
}

impl UlpData {
    fn convert(self) -> String {
        if self.url.starts_with("http") {
            return format!("{}:{}:{}", self.url, self.data, self.password)
        }
        format!("https://{}:{}:{}", self.url, self.data, self.password)
    }

    fn check_bad_word(self, ulp: &str, bad_list: &[&str; 3]) -> bool {
        let checked: String = ulp.to_lowercase();
        bad_list.iter().all(|bad| !checked.contains(&bad.to_lowercase()))
    }

    fn check_len(self, type_data: &u8) -> bool {
        let password_len = self.password.len();
        let data_len = self.data.len();

        if !(5..=24).contains(&password_len) {
            return false;
        }

        match type_data {
            0 => (8..=24).contains(&data_len),
            1 => (5..=24).contains(&data_len),
            2 => (11..=15).contains(&data_len),
            _ => (5..=24).contains(&data_len),
        }
    }
    fn regexp(self, settings: &DataSetting) -> Option<u8> {
        if !self.password.chars().all(|c| c.is_ascii() && c.is_ascii_graphic()) {
            return None;
        }

        if settings.email_regex.is_match(&self.data) {
            Some(0)
        } else if settings.login_regex.is_match(&self.data) {
            Some(1)
        } else if settings.number_regex.is_match(&self.data) {
            Some(2)
        } else {
            None
        }
    }

    pub fn check(self, settings: &DataSetting, bad_list: &[&str; 3]) -> Option<String> {
        if let Some(type_data) = self.clone().regexp(settings) {
            if !self.clone().check_len(&type_data) {
                return None;
            }
            if !self.clone().check_len(&type_data) {
                return None;
            }
            let ulp = self.clone().convert();
            if !self.clone().check_bad_word(&ulp, bad_list) {
                return None;
            }
            return Some(ulp);
        }
        None
    }
}