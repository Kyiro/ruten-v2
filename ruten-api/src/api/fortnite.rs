use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct Fortnite;

#[OpenApi]
impl Fortnite {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "fortnite", env!("CARGO_PKG_VERSION"))
    }
}