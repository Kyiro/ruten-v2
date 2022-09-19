use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct Friends;

#[OpenApi]
impl Friends {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "friends", env!("CARGO_PKG_VERSION"))
    }
}