use crate::db::DB;
use crate::dto::device::GetDevice;
use actix_web::{web, HttpResponse};
use deadpool_postgres::Object;
use tokio_postgres::Row;

// Outdated
pub async fn get_device(data: web::Data<DB>, msg: web::Json<GetDevice>) -> HttpResponse {
    let id = &msg.account.username;
    match data.devices.read().await.get(&id) {
        Some(d) => HttpResponse::Ok().json(d),
        None => HttpResponse::NotFound().body("Device not found for account"),
    }
}

pub async fn add_device(
    client: Object,
    device_name: &str,
    efuse_mac: &str,
    chip_model: &str,
    type_id: i32,
    area_id: i32,
    account_id: i32,
) -> Result<u64, tokio_postgres::Error> {
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

pub async fn show_device(
    client: Object,
    account_id: i32,
) -> Result<Vec<Row>, tokio_postgres::Error> {
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
            dt.type_name
        FROM member m
        JOIN house h ON m.house_id = h.house_id
        JOIN area a ON a.house_id = h.house_id
        JOIN device d ON d.area_id = a.area_id
        JOIN device_type dt ON dt.type_id = d.type_id
        WHERE m.account_id = $1;",
            &[&account_id],
        )
        .await
}
