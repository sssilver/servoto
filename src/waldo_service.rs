use rotor_http::server::{RecvMode, Server, Head, Response};
use rotor::{Scope, Time};
use serde_json;
use std::error::Error;
use std::time::Duration;

use context::Context;
use error::WaldoError;


pub enum WaldoService {
    UpdateCatalog,
    GetPhoto(String),
    MethodNotAllowed,
}


impl Server for WaldoService {
    type Seed = ();
    type Context = Context;

    fn headers_received(_seed: (),
                        head: Head,
                        _res: &mut Response,
                        scope: &mut Scope<Context>) -> Option<(Self, RecvMode, Time)> {

        use self::WaldoService::*;

        Some((
            match head.method {
                "GET" => GetPhoto(head.path[1..].to_string()),
                "POST" => UpdateCatalog,
                _ => MethodNotAllowed,
            },
            RecvMode::Buffered(1024),
            scope.now() + Duration::new(10, 0)
        ))
    }

    fn request_received(self,
                        _: &[u8],
                        response: &mut Response,
                        scope: &mut Scope<Context>) -> Option<Self> {

        use self::WaldoService::*;

        match self {
            UpdateCatalog => {
                match scope.update_catalog() {
                    Ok(()) => respond(response, 200, b"OK"),
                    Err(error) => respond(response, 500, error.description().as_bytes())
                }
            }

            GetPhoto(uuid) => {
                match scope.get_photo(&uuid) {
                    Ok(photo) => respond(response, 200, serde_json::to_string(&photo).unwrap().as_bytes()),
                    Err(error) => {
                        let error_code = match error {
                            WaldoError::PhotoNotFound(_) => 404,
                            _ => 500
                        };

                        respond(response, error_code, error.to_string().as_bytes())
                    }
                }
            }

            MethodNotAllowed => {
                respond(response, 405, format!("Method not allowed").as_bytes())
            }
        }

        None
    }

    //
    // We don't support proxy servers, so the following four functions are not implemented
    //
    fn request_chunk(self, _chunk: &[u8], _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self> {
        unreachable!()
    }

    fn request_end(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self> {
        unreachable!()
    }

    fn timeout(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<(Self, Time)> {
        unimplemented!()
    }

    fn wakeup(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self> {
        unimplemented!()
    }
}


fn respond(response: &mut Response, code: u16, body: &[u8]) {
    let message = match code {
        200 => "OK",
        404 => "Not Found",
        500 => "Internal server error",
        _ => ""
    };

    response.status(code, message);
    response.add_length(body.len() as u64).unwrap();
    response.done_headers().unwrap();
    response.write_body(body);

    response.done();
}
