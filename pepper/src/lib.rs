extern crate futures;
extern crate tokio_core;
extern crate route_recognizer;
//extern crate url;
extern crate hyper;

use std::net::SocketAddr;
use futures::{Stream, Future};
use futures::future::{BoxFuture, FutureResult, ok};
use tokio_core::reactor;
use tokio_core::net::TcpListener;
use route_recognizer::{Router, Match};
use hyper::StatusCode;
use hyper::header::{Accept, ContentType};
use hyper::server::{Http, Service};
pub use hyper::Error;
pub use hyper::header;
pub use route_recognizer::Params;

/*pub struct Request(hyper::server::Request);
pub struct Response(hyper::server::Response);*/
pub use hyper::server::{Request, Response};
pub type BoxResponse = BoxFuture<Response, hyper::Error>;
pub type Handler = fn(Request, Params) -> BoxResponse;

pub struct Server {
    router: Router<Handler>,
    error_handler: Option<Box<Handler>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            router: Router::new(),
            error_handler: None,
        }
    }

    pub fn add_route(&mut self, path: &str, handler: Handler) {
        &self.router.add(path, handler);
    }

    /*
    fn handle(&self) -> reactor::Handle {
        self.handle.clone()
    }*/



    pub fn run(self, addr: &SocketAddr) {
        let arc_self = std::sync::Arc::new(self);
        let server = Http::new()
            .bind(addr, move || Ok(arc_self.clone()))
            .unwrap();
        server.run();

    }
}

impl Service for Server {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        if let Ok(Match { handler, params }) = self.router.recognize(req.path()) {
            return handler(req, params);
        }
        /*        match self.router.recognize(req.path()) {
                Ok(Match { handler, params }) => {
                    req.params = Some(params);
                    handler(req)
                }
                Err(string) => {
                    println!("Error finding route: {}", string);
                    match self.error_handler {
                        Some(handler) => handler(req),
                        None => {

                        }
                    }
                }
            }
            .and_then(|response| ok(e.done()))
            */
        ::futures::future::ok(hyper::server::Response::new()
                                  .with_header(ContentType::plaintext())
                                  .with_status(StatusCode::NotFound))
                .boxed()
    }
}
