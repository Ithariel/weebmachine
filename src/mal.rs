use regex::{Error, Regex};

pub struct Mal {
    mal_regex: Regex,
    image_regex: Regex,
}

impl Mal {
    pub fn new() -> Result<Self, Error> {
        Ok(Mal {
            mal_regex: Regex::new(r"(https://myanimelist\.net/anime/\d+/?)")?,
            // Lets just hope this will be the right image lmao
            image_regex: Regex::new(
                r"(https://cdn\.myanimelist\.net/images/anime/\d+/\d+\.(?:jpg|png))",
            )?,
        })
    }

    pub async fn return_first_image_url(&self, selftext: &str) -> Option<String> {
        let mal_url = self.mal_regex.captures(selftext)?[0].to_string();
        let mal_body = reqwest::get(mal_url).await.ok()?.text().await.ok()?;
        Some(self.image_regex.captures(&mal_body)?[0].to_string())
    }
}
