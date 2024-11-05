use crate::db::{ExecuteType, QueryOneType};
use deadpool_postgres::{GenericClient, Object};

pub async fn add_user_info(
    client: Object,
    account_id: i32,
    real_name: Option<String>,
    gender: Option<String>,
    location: Option<String>,
    age: Option<i32>,
    identity: Option<String>,
) -> ExecuteType {
    client
        .execute(
            "INSERT INTO user_info\
        (account_id, real_name, gender, location, age, identity)\
        VALUES($1, $2, $3, $4, $5, $6)",
            &[&account_id, &real_name, &gender, &location, &age, &identity],
        )
        .await
}

pub async fn query_user_info(client: Object, account_id: i32) -> QueryOneType {
    client
        .query_one(
            "SELECT * FROM user_info WHERE account_id = $1",
            &[&account_id],
        )
        .await
}
