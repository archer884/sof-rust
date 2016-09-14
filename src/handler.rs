use iron::prelude::*;
use model::CookieResponse;
use persistent::Read;
use router::Router;
use serde_json;
use service::CookieService;

pub fn cookie(req: &mut Request) -> IronResult<Response> {
    let service = req.get::<Read<CookieService>>().map_err(|e| IronError::new(e, "service not available"))?;
    let cookie = service.get();

    respond(CookieResponse::new(cookie))
}

pub fn cookie_by_category(req: &mut Request) -> IronResult<Response> {
    use std::io::{Error, ErrorKind};
    
    let service = req.get::<Read<CookieService>>().map_err(|e| IronError::new(e, "service not available"))?;
    let category = req.extensions.get::<Router>()
        .expect("router not found!?")
        .find("category");

    let cookie = match category.and_then(|c| service.by_category(c)) {
        None => return Err(IronError::new(Error::new(ErrorKind::NotFound, "category unavailable"), "category unavailable")),
        Some(cookie) => cookie
    };

    respond(CookieResponse::new(cookie))
}

fn respond<'a>(response: CookieResponse<'a>) -> IronResult<Response> {
    use iron::headers::ContentType;
    use iron::status;

    let mut response = Response::with((status::Ok, serde_json::to_string(&response).expect("unable to serialize response")));
    response.headers.set(ContentType::json());
    Ok(response)
}
