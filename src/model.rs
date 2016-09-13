use chrono::{DateTime, UTC};

#[derive(Serialize)]
pub struct Cookie {
    category: String,
    content: String,
}

impl Cookie {
    pub fn new(category: String, content: String) -> Cookie {
        Cookie {
            category: category.into(),
            content: content.into(),
        }
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Serialize)]
pub struct CookieResponse<'a> {
    timestamp: DateTime<UTC>,
    cookie: &'a Cookie,
}

impl<'a> CookieResponse<'a> {
    pub fn new(cookie: &'a Cookie) -> CookieResponse<'a> {
        CookieResponse {
            timestamp: UTC::now(),
            cookie: cookie,
        }
    }
}
