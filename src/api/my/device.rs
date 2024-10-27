use crate::api::auth::Claims;
use crate::template::template::{claims_template, claims_with_json_template};
use crate::db::device::{add_device, show_device};
use crate::utils;
use actix_web::{web, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddDevice {
    device_name: String,
    area_id: i32,
    type_id: i32,
    efuse_mac: String,
    chip_model: String,
}

pub async fn add_device_api(
    body: web::Json<AddDevice>,
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> HttpResponse {
    claims_with_json_template(
        body,
        pool,
        req,
        Box::new(|body, claims, client| Box::pin(add(body, claims, client))),
    )
    .await
}

async fn add(body: AddDevice, claims: Claims, client: Object) -> HttpResponse {
    match add_device(
        client,
        &body.device_name,
        &body.efuse_mac,
        &body.chip_model,
        body.type_id,
        body.area_id,
        claims.id(),
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

pub async fn show_devices_api(db: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    claims_template(
        db,
        req,
        Box::new(|claims, client| Box::pin(show(client, claims))),
    )
    .await
}

async fn show(client: Object, claims: Claims) -> HttpResponse {
    let account_id = claims.id();
    match show_device(client, account_id).await {
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
                    if let Some(area) = house
                        .areas_devices
                        .iter_mut()
                        .find(|a| a.area_name == area_name)
                    {
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
