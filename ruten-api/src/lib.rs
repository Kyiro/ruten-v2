use poem::*;

// pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod model;
pub(crate) mod util;

pub async fn init(listener: &str) -> std::io::Result<()> {
    let app = Route::new();
        // .nest("/account", api::Account::service())
        // .nest("/content", api::Content::service())
        // .nest("/datarouter", api::DataRouter::service())
        // .nest("/fortnite", api::Fortnite::service())
        // .nest("/friends", api::Friends::service())
        // .nest("/lightswitch", api::LightSwitch::service())
        // .nest("/party", api::Party::service())
        // .nest("/presence", api::Presence::service())
        // .nest("/waitingroom", api::WaitingRoom::service());

    poem::Server::new(listener::TcpListener::bind(listener))
        .run(app)
        .await
}

#[tokio::main]
pub async fn init_sync(listener: &str) -> std::io::Result<()> {
    init(listener).await
}