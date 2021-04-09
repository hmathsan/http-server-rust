use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;

pub struct  WebsiteHandler;

impl  Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, 
                    Some("<h1>404 NOT FOUND</h1>\r\n<h3>Path not found</h3>".to_string()))
    
            }
            _ => Response::new(StatusCode::NotFound, 
                Some("<h1>404 NOT FOUND</h1>\r\n<h3>Request method not found</h3>".to_string()))
        }
        //Response::new(StatusCode::Ok, Some("<h1> TEST </HE>".to_string()))
    }
}