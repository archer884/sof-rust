#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

#[macro_use] extern crate router;
extern crate chrono;
extern crate iron;
extern crate persistent;
extern crate rand;
extern crate serde;
extern crate serde_json;

mod handler;
mod model;
mod service;

use iron::prelude::*;
use model::Cookie;
use persistent::Read;
use service::CookieService;

fn main() {
    let cookies = match read_cookies() {
        Some(cookies) => cookies,
        None => {
            println!("provide cookie directory");
            std::process::exit(1);
        }
    };

    let mut chain = Chain::new(router! {
        cookie: get "/api/cookie" => handler::cookie,
        cookie_by_category: get "/api/cookie/:category" => handler::cookie_by_category,
    });
    chain.link(Read::<CookieService>::both(cookies));
    Iron::new(chain).http("localhost:5000").unwrap();
}

fn read_cookies() -> Option<CookieService> {
    use std::fs::{self, File};
    use std::io::Read;

    let files = std::env::args().nth(1)
        .and_then(|dir| fs::read_dir(dir).ok())
        .map(|dir| dir.filter_map(|file| file.map(|file| file.path()).ok()));

    let file_contents = match files {
        None => return None,
        Some(files) => files.filter_map(|file| match File::open(&file) {
            Err(_) => None,
            Ok(mut content) => {
                let mut buf = String::new();
                content.read_to_string(&mut buf).ok();

                let category = file.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();
                Some((category, buf))
            }
        })
    };

    let mut cookies = Vec::new();
    for (category, content) in file_contents {
        for quote in content.split('%') {
            cookies.push(Cookie::new(category.clone().into(), quote.trim().into()));
        }
    }

    Some(CookieService::new(cookies))
}
