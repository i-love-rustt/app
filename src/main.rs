mod db;
mod models;
mod handlers;
mod routes;




#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();


    dotenvy::dotenv().ok();
    let database_url =  std::env::var("DATABASE_URL").expect("DATABASE_URL غير موجود");
    let pool = db::connect(&database_url).await;


    let app = routes::create_router(pool) ;


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}



