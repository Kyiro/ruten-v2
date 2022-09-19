use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct LightSwitch;

#[OpenApi]
impl LightSwitch {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "lightswitch", env!("CARGO_PKG_VERSION"))
    }
}