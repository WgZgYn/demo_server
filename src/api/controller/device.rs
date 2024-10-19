use crate::security::Claims;
use crate::utils;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use deadpool_postgres::{GenericClient, Pool};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Index;
use tokio_postgres::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct AddDevice {
    device_name: String,
    area_id: i32,
    type_id: i32,
    efuse_mac: String,
    chip_model: String,
}

pub async fn add_device(
    body: web::Json<AddDevice>,
    db: web::Data<Pool>,
    req: HttpRequest,
) -> HttpResponse {
    let client = match db.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database error"));
        }
    };

    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => {
            return HttpResponse::InternalServerError().json(utils::Result::error("claims error"))
        }
    };

    let username = claims.sub();
    let account_id: i32 = match client
        .query_one(
            "SELECT account_id FROM account WHERE username = $1",
            &[&username],
        )
        .await
    {
        Ok(row) => row.get(0),
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database error"));
        }
    };

    match client
        .execute(
            "INSERT INTO device\
     (device_name, efuse_mac, chip_model, type_id, area_id, created_by)\
      VALUES ($1, $2, $3, $4, $5, $6)",
            &[
                &body.device_name,
                &body.efuse_mac,
                &body.chip_model,
                &body.type_id,
                &body.area_id,
                &account_id,
            ],
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(utils::Result::error("database error"))
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct DeviceType {
    type_id: i32,
    type_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Device {
    device_id: i32,
    device_name: String,
    efuse_mac: String,
    chip_model: String,
    device_type: DeviceType,
}

#[derive(Deserialize, Serialize, Debug)]
struct AreaDevices {
    area_id: i32,
    area_name: String,
    devices: Vec<Device>,
}

#[derive(Deserialize, Serialize, Debug)]
struct HouseDevices {
    house_id: i32,
    house_name: String,
    areas_devices: Vec<AreaDevices>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash)]
struct House {
    house_id: i32,
    house_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShowDevices {
    houses_devices: Vec<HouseDevices>,
}

pub async fn show_devices(db: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    let client = match db.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("{e}");
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database error"));
        }
    };

    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => {
            return HttpResponse::InternalServerError().json(utils::Result::error("claims error"))
        }
    };

    let account_id = claims.id();
    match client
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
    {
        Ok(rows) => {
            let mut ad: Vec<HouseDevices> = Vec::new();

            for row in rows {
                let house_id: i32 = row.get("house_id");
                let house_name = row.get("house_name");

                let area_id: i32 = row.get("area_id");
                let area_name = row.get("area_name");

                let device_id = row.get("device_id");
                let device_name = row.get("device_name");
                let efuse_mac = row.get("efuse_mac");
                let chip_model = row.get("chip_model");
                let type_id = row.get("type_id");
                let type_name = row.get("type_name");

                if let Some(house) = ad.iter_mut().find(|h| h.house_id == house_id) {
                    if let Some(area) = house.areas_devices.iter_mut().find(|a| a.area_name == area_name) {
                        area.devices.push(Device {
                            device_id,
                            device_name,
                            efuse_mac,
                            chip_model,
                            device_type: DeviceType { type_id, type_name },
                        });
                    } else {
                        house.areas_devices.push(AreaDevices {
                            area_id,
                            area_name,
                            devices: vec![Device {
                                device_id,
                                device_name,
                                efuse_mac,
                                chip_model,
                                device_type: DeviceType { type_id, type_name },
                            }],
                        });
                    }
                } else {
                    ad.push(HouseDevices {
                        house_id,
                        house_name,
                        areas_devices: vec![AreaDevices {
                            area_id,
                            area_name,
                            devices: vec![Device {
                                device_id,
                                device_name,
                                efuse_mac,
                                chip_model,
                                device_type: DeviceType { type_id, type_name },
                            }],
                        }],
                    });
                }
            }
            HttpResponse::Ok().json(utils::Response::success(ShowDevices { houses_devices: ad }))
        }
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(utils::Result::error("database error"))
        }
    }
}
