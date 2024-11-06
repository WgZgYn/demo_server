use crate::api::auth::Claims;
use crate::db::device::{add_device, show_device};
use crate::template::template::{claims_template, claims_with_data_template};
use crate::utils;
use actix_web::{web, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use log::error;
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use crate::data::device::DeviceStatus;
use crate::utils::Response;

pub async fn add_device_api(
    body: web::Json<AddDevice>,
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> HttpResponse {
    claims_with_data_template(
        body,
        pool,
        req,
        Box::new(|body, claims, client| Box::pin(add(body.into_inner(), claims, client))),
    )
        .await
}

pub async fn query_devices_api(pool: web::Data<Pool>, req: HttpRequest) -> HttpResponse {
    claims_template(
        pool,
        req,
        Box::new(|claims, client| Box::pin(query(client, claims))),
    )
        .await
}

// TODO:
pub async fn update_device_api() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO:
pub async fn delete_device_api() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn device_service_api(
    pool: web::Data<Pool>,
    client: web::Data<AsyncClient>,
    path: web::Path<(i32, String)>,
    req: HttpRequest,
) -> HttpResponse {
    claims_with_data_template(
        (path, client),
        pool,
        req,
        Box::new(|body, claims, client| Box::pin(device_service(client, claims, body))),
    )
        .await
}

pub async fn device_status_api(
    memory: web::Data<DeviceStatus>,
    path: web::Path<i32>,
) -> HttpResponse {
    if let Some(v) = memory.status(*path) {
        HttpResponse::Ok().json(Response::success(v))
    } else {
        HttpResponse::NotFound().finish()
    }
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

async fn query(client: Object, claims: Claims) -> HttpResponse {
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
                let model_name = row.get("model_name");
                let type_id = row.get("type_id");
                let type_name = row.get("type_name");
                let parameter: Option<serde_json::Value> = row.get("parameter");
                let mut s = Vec::new();
                if parameter.is_some() {
                    s.push(parameter.unwrap());
                }
                if let Some(house) = ad.iter_mut().find(|h| h.house_id == house_id) {
                    match house
                        .areas_devices
                        .iter_mut()
                        .find(|a| a.area_name == area_name)
                    {
                        Some(area) => {
                            match area.devices.iter_mut().find(|d| d.device_id == device_id) {
                                Some(d) => d.service.append(&mut s),
                                None => area.devices.push(Device {
                                    device_id,
                                    device_name,
                                    efuse_mac,
                                    model_name,
                                    device_type: DeviceType { type_id, type_name },
                                    service: s,
                                }),
                            }
                        }
                        None => {
                            house.areas_devices.push(AreaDevices {
                                area_id,
                                area_name,
                                devices: vec![Device {
                                    device_id,
                                    device_name,
                                    efuse_mac,
                                    model_name,
                                    device_type: DeviceType { type_id, type_name },
                                    service: s,
                                }],
                            });
                        }
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
                                model_name,
                                device_type: DeviceType { type_id, type_name },
                                service: s,
                            }],
                        }],
                    });
                }
            }
            HttpResponse::Ok().json(Response::success(ShowDevices { houses_devices: ad }))
        }
        Err(e) => {
            error!("{e}");
            HttpResponse::InternalServerError().json(utils::Result::error("database error"))
        }
    }
}

// TODO:
async fn delete() {}

// TODO:
async fn update() {}

async fn device_service(
    client: Object,
    claims: Claims,
    data: (web::Path<(i32, String)>, web::Data<AsyncClient>),
) -> HttpResponse {
    let (path, mqtt) = data;
    let (device_id, service_name) = path.into_inner();
    let account_id = claims.id();

    match client
        .query_one(
            "\
        SELECT *
        FROM member
        JOIN account USING(account_id)
        JOIN area ON area.house_id = member.house_id
        JOIN device ON area.area_id = device.area_id
        WHERE device_id = $1 AND account_id = $2
    ",
            &[&device_id, &account_id],
        )
        .await
    {
        Ok(row) => {
            let mac: String = row.get("efuse_mac");
            match mqtt
                .publish(
                    format!("/device/{}/service", mac),
                    QoS::ExactlyOnce,
                    false,
                    service_name,
                )
                .await
            {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => {
                    error!("{e}");
                    HttpResponse::InternalServerError().json(utils::Result::error("mqtt error"))
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(utils::Result::error(&format!("{e}"))),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddDevice {
    device_name: String,
    area_id: i32,
    type_id: i32,
    efuse_mac: String,
    chip_model: String,
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
    model_name: String,
    device_type: DeviceType,
    service: Vec<serde_json::Value>,
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
