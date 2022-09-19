use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct DataRouter;

#[OpenApi]
impl DataRouter {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "datarouter", env!("CARGO_PKG_VERSION"))
    }
}