pub const SRC_BASE_URL: &str = "https://hianime.to";
pub const SRC_AJAX_URL: &str = "/ajax"; 
pub const SRC_HOME_URL: &str = "/home";
pub const SRC_SEARCH_URL: &str = "/search";

pub const IP: &str = "127.0.0.1";
pub const PORT: usize = 4000;

pub const ACCEPT_ENCODING_HEADER: &str = "gzip, deflate, br";
pub const USER_AGENT_HEADER: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4692.71 Safari/537.36";
pub const ACCEPT_HEADER: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9";

// pub fn generate_url(key: &str) -> String {
//     match key {
//         "base" => return format!("{}{}", SRC_URL_PROT_PART, DOMAIN_NAME),
//         "ajax" => return format!("{}{}{}", SRC_URL_PROT_PART, DOMAIN_NAME, SRC_AJAX_URL_PART),
//         "home" => return format!("{}{}{}", SRC_URL_PROT_PART, DOMAIN_NAME, SRC_HOME_URL_PART),
//         "search" => return format!("{}{}{}", SRC_URL_PROT_PART, DOMAIN_NAME, SRC_SEARCH_URL_PART),
//         _ => return String::new()
//     }
// }

// pub fn find(key: &str) -> Result<&str, DefaultError> {
//     match DEFAULT_ENV.iter().find(|x| x.0 == &key) {
//         Some(x) => Ok(x.1),
//         None => Err(DefaultError { message: String::from("value") })
//     }
// }