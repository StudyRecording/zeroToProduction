use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::{configuration::get_configuration, startup::run, telemetry::{get_subscriber, init_subscriber}};





#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let subscriber = get_subscriber(
        "zero2prod".into(), "info".into(), std::io::stdout
    );    
    init_subscriber(subscriber);



    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database.with_db());

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;

    let port: u16 = listener.local_addr().unwrap().port();
    println!("启动地址: http://127.0.0.1:{port}");

    run(listener, connection_pool)?.await

}