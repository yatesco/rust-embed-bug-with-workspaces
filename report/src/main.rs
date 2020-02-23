#[macro_use]
extern crate rust_embed;

use std::borrow::Cow;

use askama::Template;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::hyper::{Body, Response, StatusCode, Uri};
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::state::{FromState, State};

// NOTE: these come from the _root_/assets directory, not _root_/report/assets
#[derive(RustEmbed)]
#[folder = "assets/public/"]
struct Asset;

#[derive(Debug, Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {}

fn base(state: State) -> (State, Response<Body>) {

    let tpl = BaseTemplate {};
    // The response is either the rendered template, or a server error if something really goes wrong
    let res = match tpl.render() {
        Ok(content) => create_response(
            &state,
            StatusCode::OK,
            mime::TEXT_HTML_UTF_8,
            content.into_bytes(),
        ),
        Err(_) => create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR),
    };

    (state, res)
}

fn resolve_asset(assets_font_path: Option<Cow<'static, [u8]>>) -> Option<Body> {
    match assets_font_path {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            Some(body)
        }
        None => None
    }
}


fn static_asset(state: State) -> (State, Response<Body>) {
    let uri = Uri::borrow_from(&state);
    println!("URI: {}", &uri);
    let path = uri.path();
    println!("PATH: {}", &path);
    let characters_to_drop = "/assets/".len();
    let path = &path[characters_to_drop..];
    println!("file to resolve: {}", &path);
    let extension = mime::IMAGE_JPEG;
    let res = match resolve_asset(Asset::get(path)) {
        Some(x) => create_response(&state, StatusCode::OK,
                                   extension,
                                   x),
        None => create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR)
    };
    (state, res)
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening at {}", addr);

    println!("printing assets: ");
    for file in Asset::iter() {
        println!("{}", file.as_ref());
    }

    let router = build_simple_router(|route| {
        route.get("/").to(base);
        route.get("/assets/*").to(static_asset);
    });

    gotham::start(addr, router)
}
