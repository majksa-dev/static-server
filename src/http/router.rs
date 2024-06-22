use gateway::{Request, Router};

#[derive(Debug, Default)]
pub struct AnyRouter(String);

impl AnyRouter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Router for AnyRouter {
    fn matches(&self, _request: &Request) -> Option<&String> {
        Some(&self.0)
    }
}
