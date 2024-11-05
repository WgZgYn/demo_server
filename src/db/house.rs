use crate::db::{ExecuteType, QueryType};
use deadpool_postgres::Object;

pub async fn add_house(client: Object, house_name: &str, account_id: i32) -> ExecuteType {
    client
        .execute(
            "INSERT INTO house (house_name, created_by) VALUES ($1, $2)",
            &[&house_name, &account_id],
        )
        .await
}

pub async fn query_house(client: Object, account_id: i32) -> QueryType {
    client
        .query(
            "SELECT * FROM house \
        JOIN member USING(house_id) \
        WHERE account_id = $1",
            &[&account_id],
        )
        .await
}
