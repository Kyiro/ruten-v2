use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct Content;

#[OpenApi]
impl Content {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "content", env!("CARGO_PKG_VERSION"))
    }
}