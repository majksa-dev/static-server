use gateway::{Id, Request, Router, RouterBuilder, RouterService};

#[derive(Debug)]
pub struct AnyRouter;

pub struct AnyRouterBuilder;

impl AnyRouterBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl RouterBuilder for AnyRouterBuilder {
    fn build(self: Box<Self>) -> (Vec<String>, RouterService) {
        (vec![String::new()], Box::new(AnyRouter))
    }
}

impl Router for AnyRouter {
    fn matches(&self, _request: &Request) -> Option<Id> {
        Some(0)
    }
}
