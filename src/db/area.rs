use deadpool_postgres::Object;

pub async fn add_area(
    client: Object,
    area_name: &str,
    house_id: i32,
    account_id: i32,
) -> Result<u64, tokio_postgres::Error> {
    client
        .execute(
            "INSERT INTO area (area_name, house_id, created_vy) VALUES ($1, $2)",
            &[&area_name, &house_id, &account_id],
        )
        .await
}

pub async fn get_area(client: Object, house_id: i32) -> Result<Vec<String>, tokio_postgres::Error> {
    let rows = client
        .query(
            "SELECT area_name FROM area WHERE house_id = $1",
            &[&house_id],
        )
        .await?;

    let mut areas = Vec::new();
    for row in rows {
        areas.push(row.get(0));
    }

    Ok(areas)
}

pub async fn delete_area(
    client: Object,
    area_id: i32,
    house_id: i32,
) -> Result<u64, tokio_postgres::Error> {
    client
        .execute(
            "DELETE FROM area WHERE area_id = $1 AND house_id = $2",
            &[&area_id, &house_id],
        )
        .await
}
