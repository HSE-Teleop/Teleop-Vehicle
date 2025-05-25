use zenoh::{Config, Session};

pub async fn create_zenoh_session(CONFIG: &str) -> Session {
    zenoh::init_log_from_env_or("error");
    let config = Config::from_json5(CONFIG).unwrap();
    println!("Opening Zenoh client session...");
    zenoh::open(config).await.unwrap()
}