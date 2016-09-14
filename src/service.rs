use iron::typemap::Key;
use model::Cookie;
use rand::{self, Rng};

pub struct CookieService { cookies: Vec<Cookie> }

impl CookieService {
    pub fn new(cookies: Vec<Cookie>) -> CookieService {
        CookieService { cookies: cookies }
    }
    
    pub fn get(&self) -> &Cookie {
        rand::thread_rng().choose(&self.cookies).expect("there are no damn cookies in here")
    }

    pub fn by_category(&self, category: &str) -> Option<&Cookie> {
        let category = category.replace("-", "").to_lowercase();
        let cookies: Vec<_> = self.cookies
            .iter()
            .filter(|cookie| category == cookie.category().replace("-", "").to_lowercase())
            .collect();

        rand::thread_rng().choose(&cookies[..]).map(|cookie| *cookie)
    }
}

impl Key for CookieService {
    type Value = CookieService;
}
