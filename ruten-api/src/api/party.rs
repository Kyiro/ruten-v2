use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct Party;

#[OpenApi]
impl Party {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "party", env!("CARGO_PKG_VERSION"))
    }
}