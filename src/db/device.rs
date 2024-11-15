use crate::db::{ExecuteType, QueryOneType, QueryType};
use deadpool_postgres::{GenericClient, Object};

pub async fn add_device(
    client: Object,
    device_name: &str,
    efuse_mac: &str,
    chip_model: &str,
    type_id: i32,
    area_id: i32,
    account_id: i32,
) -> ExecuteType {
    client
        .execute(
            "INSERT INTO device\
     (device_name, efuse_mac, chip_model, type_id, area_id, created_by)\
      VALUES ($1, $2, $3, $4, $5, $6)",
            &[
                &device_name,
                &efuse_mac,
                &chip_model,
                &type_id,
                &area_id,
                &account_id,
            ],
        )
        .await
}

pub async fn show_device(client: Object, account_id: i32) -> QueryType {
    client
        .query(
            "SELECT * FROM account_devices_view WHERE account_id = $1;",
            &[&account_id],
        )
        .await
}

pub async fn get_device_id_by_mac(client: Object, efuse_mac: &str) -> QueryOneType {
    client
        .query_one(
            "SELECT device_id FROM device WHERE efuse_mac = $1",
            &[&efuse_mac],
        )
        .await
}
