use crate::db::ExecuteType;
use deadpool_postgres::Object;
use tokio_postgres::Row;

pub async fn add_area(
    client: Object,
    area_name: &str,
    house_id: i32,
    account_id: i32,
) -> ExecuteType {
    client
        .execute(
            "INSERT INTO area (area_name, house_id, created_vy) VALUES ($1, $2)",
            &[&area_name, &house_id, &account_id],
        )
        .await
}

pub async fn query_area(client: Object, house_id: i32) -> Result<Vec<Row>, tokio_postgres::Error> {
    client
        .query("SELECT * FROM area WHERE house_id = $1", &[&house_id])
        .await
}

// TODO: not completed
pub async fn delete_area(client: Object, area_id: i32, house_id: i32) -> ExecuteType {
    client
        .execute(
            "DELETE FROM area WHERE area_id = $1 AND house_id = $2",
            &[&area_id, &house_id],
        )
        .await
}
