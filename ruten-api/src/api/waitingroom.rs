use poem_openapi::{OpenApiService, OpenApi, payload::*};

pub struct WaitingRoom;

#[OpenApi]
impl WaitingRoom {
    pub fn service() -> OpenApiService<Self, ()> {
        OpenApiService::new(Self, "waitingroom", env!("CARGO_PKG_VERSION"))
    }
}