use rotor_http::server::{RecvMode, Server, Head, Response};
use rotor::{Scope, Time};
use std::time::Duration;

use context::Context;


pub enum WaldoService {
    PhotoInformation(String)
}


impl Server for WaldoService {
    type Seed = ();
    type Context = Context;

    fn headers_received(_seed: (),
                        head: Head,
                        _res: &mut Response,
                        scope: &mut Scope<Context>) -> Option<(Self, RecvMode, Time)> {

        use self::WaldoService::*;

        Some((match head.path {
            path => PhotoInformation(path[1..].to_string())
        }, RecvMode::Buffered(1024), scope.now() + Duration::new(10, 0)))
    }

    fn request_received(self,
                        _data: &[u8],
                        res: &mut Response,
                        scope: &mut Scope<Context>) -> Option<Self> {

        use self::WaldoService::*;

        match self {
            PhotoInformation(uuid) => {
                let photo = scope.get(&uuid);
                send_string(res, format!("{:?}", photo).as_bytes());
            }
        }

        None
    }

    //
    // We don't support proxy servers, so the following four functions are not implemented
    //
    fn request_chunk(self, _chunk: &[u8], _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self> {
        unreachable!();
    }

    fn request_end(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self> {
        unreachable!();
    }

    fn timeout(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<(Self, Time)> {
        unimplemented!();
    }

    fn wakeup(self, _response: &mut Response, _scope: &mut Scope<Context>) -> Option<Self> {
        unimplemented!();
    }
}


fn send_string(res: &mut Response, data: &[u8]) {
    res.status(200, "OK");
    res.add_length(data.len() as u64).unwrap();
    res.done_headers().unwrap();
    res.write_body(data);
    res.done();
}
