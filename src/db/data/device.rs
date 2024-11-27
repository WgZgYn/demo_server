use crate::db::database::Session;
use crate::dto::entity::simple::{AccountInfo, AreaInfo, DeviceInfo, DeviceType, HouseInfo};
use crate::dto::http::response::{AccountDevices, AreaDevices, HouseDevices};
use deadpool_postgres::PoolError;
use tokio_postgres::Error;
use crate::dto::http::request::DeviceUpdate;

impl Session {
    pub async fn delete_device(&self, device_id: i32) -> Result<u64, Error> {
        self.0
            .execute("DELETE FROM device WHERE device_id = $1", &[&device_id])
            .await
    }

    pub async fn add_device(
        &self,
        device_name: &str,
        efuse_mac: &str,
        area_id: i32,
        account_id: i32,
        model_id: i32,
    ) -> Result<u64, Error> {
        self.0.execute("INSERT INTO device (device_name, efuse_mac, area_id, created_by, model_id) VALUES ($1, $2, $3, $4, $5, $6)",
                       &[&device_name, &efuse_mac, &area_id, &account_id, &model_id]).await
    }

    // TODO: use account_devices_view instead
    pub async fn get_device_info(
        &self,
        device_id: i32,
    ) -> Result<DeviceInfo, Box<dyn std::error::Error>> {
        let rows = self
            .0
            .query(
                "SELECT * FROM device d \
        JOIN device_model m USING(model_id) \
        JOIN device_type t USING(type_id) \
        JOIN device_control c USING(model_id) \
        WHERE device_id = $1",
                &[&device_id],
            )
            .await?;
        let mut row_iter = rows.into_iter();
        let first = row_iter.next();
        if let None = first {
            return Err(format!("no such device {}", device_id).into());
        }
        let first = first.unwrap();
        let mut device_info = DeviceInfo {
            device_id,
            device_name: first.get("device_name"),
            efuse_mac: first.get("efuse_mac"),
            model_id: first.get("model_id"),
            model_name: first.get("model_name"),
            device_type: DeviceType {
                type_id: first.get("type_id"),
                type_name: first.get("type_name"),
            },
            service: vec![first.get("parameter")],
        };
        for r in row_iter {
            device_info.service.push(r.get("parameter"));
        }
        Ok(device_info)
    }

    pub async fn add_device_by(
        &self,
        area_id: i32,
        device_name: &str,
        efuse_mac: &str,
        model_id: i32,
        account_id: i32,
    ) -> Result<u64, Error> {
        self.0
            .execute(
                "INSERT INTO device\
            (device_name, efuse_mac, area_id, created_by, model_id) \
            VALUES ($1, $2, $3, $4, $5, $6)",
                &[&device_name, &efuse_mac, &area_id, &account_id, &model_id],
            )
            .await
    }

    pub async fn get_device_mac_by_id(&self, device_id: i32) -> Result<String, PoolError> {
        let row = self
            .0
            .query_one(
                "SELECT efuse_mac FROM device WHERE device_id = $1",
                &[&device_id],
            )
            .await?;
        Ok(row.get("efuse_mac"))
    }
    pub async fn get_device_id_by_mac(&self, efuse_mac: &str) -> Result<i32, PoolError> {
        let row = self
            .0
            .query_one(
                "SELECT device_id FROM device WHERE efuse_mac = $1",
                &[&efuse_mac],
            )
            .await?;
        Ok(row.get("device_id"))
    }
}

impl Session {
    pub async fn get_houses_devices(
        &self,
        account_id: i32,
    ) -> Result<Vec<HouseDevices>, PoolError> {
        let mut houses_devices = Vec::<HouseDevices>::new();
        let rows = self
            .0
            .query(
                "SELECT * FROM account_devices_view WHERE account_id = $1;",
                &[&account_id],
            )
            .await?;
        for row in rows {
            // database ensure that the house_id and house_name Not Null,
            let house_id: i32 = row.get("house_id");
            let house_name = row.get("house_name");

            let areas_devices = if let Some(house_devices) = houses_devices
                .iter_mut()
                .find(|s| s.house_info.house_id == house_id)
            {
                &mut house_devices.areas_devices
            } else {
                let house_info = HouseInfo {
                    house_id,
                    house_name,
                };
                houses_devices.push(HouseDevices {
                    house_info,
                    areas_devices: Vec::new(),
                });
                &mut houses_devices.last_mut().unwrap().areas_devices
            };

            let have_ares: Option<i32> = row.get("area_id");
            if have_ares.is_none() {
                continue;
            }

            let area_id = row.get("area_id");
            let area_name = row.get("area_name");

            let devices = if let Some(area_devices) = areas_devices
                .iter_mut()
                .find(|s| s.area_info.area_id == area_id)
            {
                &mut area_devices.devices
            } else {
                let area_info = AreaInfo { area_id, area_name };
                areas_devices.push(AreaDevices {
                    area_info,
                    devices: Vec::new(),
                });
                &mut areas_devices.last_mut().unwrap().devices
            };

            let have_devices: Option<i32> = row.get("device_id");
            if have_devices.is_none() {
                continue;
            }

            let device_id = row.get("device_id");
            let device_name = row.get("device_name");
            let efuse_mac = row.get("efuse_mac");
            let model_id = row.get("model_id");
            let model_name = row.get("model_name");
            let type_id = row.get("type_id");
            let type_name = row.get("type_name");
            let parameter: Option<serde_json::Value> = row.get("parameter");

            if let Some(device) = devices.iter_mut().find(|s| s.device_id == device_id) {
                if let Some(v) = parameter {
                    device.service.push(v);
                }
            } else {
                devices.push(DeviceInfo {
                    device_id,
                    device_name,
                    efuse_mac,
                    model_id,
                    model_name,
                    device_type: DeviceType { type_id, type_name },
                    service: if let Some(v) = parameter {
                        vec![v]
                    } else {
                        vec![]
                    },
                });
            }
        }
        Ok(houses_devices)
    }
    // Success
    pub async fn get_account_devices(
        &self,
        account_id: i32,
        username: String,
    ) -> Result<AccountDevices, PoolError> {
        let account_info = AccountInfo {
            account_id,
            username,
        };
        let houses_devices = self.get_houses_devices(account_id).await?;
        Ok(AccountDevices {
            account_info,
            houses_devices,
        })
    }

    pub async fn get_house_devices(
        &self,
        account_id: i32,
        house_id: i32,
    ) -> Result<HouseDevices, Box<dyn std::error::Error>> {
        let vh = self.get_houses_devices(account_id).await?;
        match vh.into_iter().find(|s| s.house_info.house_id == house_id) {
            Some(house_devices) => Ok(house_devices),
            None => Err(format!("no such house {}", house_id).into()),
        }
    }

    pub async fn get_area_devices(
        &self,
        account_id: i32,
        house_id: i32,
        area_id: i32,
    ) -> Result<Option<AreaDevices>, Box<dyn std::error::Error>> {
        let hd = self.get_house_devices(account_id, house_id).await?;
        match hd
            .areas_devices
            .into_iter()
            .find(|a| a.area_info.area_id == area_id)
        {
            Some(h) => Ok(Some(h)),
            None => Err(format!("no such area: {}", area_id).into()),
        }
    }

    pub async fn update_device_info(&self, info: DeviceUpdate) -> Result<u64, Error> {
        let mut p = "UPDATE device SET ".to_string();
        let mut a = Vec::<&(dyn tokio_postgres::types::ToSql + Sync)>::new();

        if let Some(ref name) = info.device_name {
            a.push(name);
            p += &format!("device_name = ${}", a.len());
        }

        if let Some(ref id) = info.area_id {
            a.push(id);
            p += &format!("area_id = ${}", a.len());
        }

        self.0.execute(&p, &a).await
    }
}
