use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct Account;

#[OpenApi]
impl Account {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "account", env!("CARGO_PKG_VERSION"))
    }
}