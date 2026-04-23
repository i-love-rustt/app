use sqlx:: PgPool ;

pub async fn connect(database_url: &str) ->PgPool {

    PgPool::connect(database_url)
    .await
    .expect("فشل الاتصال بالداتا ")
    
}