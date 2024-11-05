use crate::db::{ExecuteType, QueryType};
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
            "
        SELECT
            h.house_id,
            h.house_name,
            a.area_id,
            a.area_name,
            d.device_id,
            d.device_name,
            d.efuse_mac,
            d.chip_model,
            dt.type_id,
            dt.type_name,
            dc.parameter
        FROM member m
        JOIN house h 
            ON m.house_id = h.house_id
        JOIN area 
            a ON a.house_id = h.house_id
        JOIN device d 
            ON d.area_id = a.area_id
        JOIN device_type dt 
            ON dt.type_id = d.type_id
        LEFT JOIN device_control dc 
            ON dc.device_id = d.device_id
        WHERE m.account_id = $1;",
            &[&account_id],
        )
        .await
}
