use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct Presence;

#[OpenApi]
impl Presence {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "presence", env!("CARGO_PKG_VERSION"))
    }
}