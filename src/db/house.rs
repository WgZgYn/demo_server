use deadpool_postgres::Object;

pub async fn add_house(
    client: Object,
    house_name: &str,
    account_id: i32,
) -> Result<u64, tokio_postgres::Error> {
    client
        .execute(
            "INSERT INTO house (house_name, created_by) VALUES ($1, $2)",
            &[&house_name, &account_id],
        )
        .await
}
