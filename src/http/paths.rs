use crate::http::response::Response;

#[derive(Debug)]
pub struct Paths {
    paths: Vec<(String, String, fn() -> Response)>,
    default: fn() -> Response,
}

impl Paths {
    pub fn new(default: fn() -> Response) -> Paths {
        Paths {
            default: default,
            paths: Vec::new()
        }
    }

    pub fn add_route(&mut self, method: String, path: String, f: fn() -> Response) {
        self.paths.push((method, path, f));
    }

    pub fn handle(&self, method: String, path: String) -> Response {
        for (m, p, f) in self.paths.iter() {
            if *m == method && *p == path {
                return f();
            }
        }
        return (self.default)();
    }
}