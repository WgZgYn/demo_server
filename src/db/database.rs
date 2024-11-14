use crate::api::auth::{create_token, Role};
use crate::db::create_connection_pool;
use crate::dto::entity::simple::{AccountInfo, AreaInfo, DeviceInfo, DeviceType, HouseInfo, UserInfo};
use crate::dto::http::response::{AccountDevices, AreaDevices, HouseDevices};
use crate::utils::config::DataBaseConfig;
use deadpool_postgres::{Pool, PoolError};
use std::ops::Deref;

pub struct Session(deadpool_postgres::Client);

impl Deref for Session {
    type Target = deadpool_postgres::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct DataBase {
    pool: Pool,
}

impl DataBase {
    pub async fn new(config: &DataBaseConfig) -> Self {
        Self {
            pool: create_connection_pool(config).await.unwrap()
        }
    }
    pub fn from(pool: Pool) -> Self {
        Self { pool }
    }
    pub async fn get_session(&self) -> Result<Session, PoolError> {
        let client = self.pool.get().await?;
        Ok(Session(client))
    }
}

impl Session {
    pub async fn get_account_all_devices(&self, account_id: i32, username: String) -> Result<AccountDevices, PoolError> {
        let account_info = AccountInfo { account_id, username };
        let mut houses_devices = Vec::<HouseDevices>::new();

        let rows = self.query("SELECT * FROM account_devices_view WHERE account_id = $1;", &[&account_id]).await?;

        for row in rows {
            let house_id: i32 = row.get("house_id");
            let house_name = row.get("house_name");

            let mut areas_devices =
                if let Some(house_devices) = houses_devices.iter_mut().find(|s| s.house_info.house_id == house_id) {
                    &mut house_devices.areas_devices
                } else {
                    let house_info = HouseInfo { house_id, house_name };
                    houses_devices.push(HouseDevices {
                        house_info,
                        areas_devices: Vec::new(),
                    });
                    &mut houses_devices.last_mut().unwrap().areas_devices
                };

            let area_id: i32 = row.get("area_id");
            let area_name = row.get("area_name");

            let devices =
                if let Some(area_devices) = areas_devices.iter_mut().find(|s| s.area_info.area_id == area_id) {
                    &mut area_devices.devices
                } else {
                    let area_info = AreaInfo { area_id, area_name };
                    areas_devices.push(AreaDevices {
                        area_info,
                        devices: Vec::new(),
                    });
                    &mut areas_devices.last_mut().unwrap().devices
                };

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
                    device_type: DeviceType {
                        type_id,
                        type_name,
                    },
                    service: if let Some(v) = parameter { vec![v] } else { vec![] },
                });
            }
        }

        Ok(AccountDevices {
            account_info,
            houses_devices,
        })
    }


    // pub async fn get_house_all_devices(&self, account_id: i32, house_id: i32)  {
    //
    // }
    //
    // pub async fn get_area_all_devices(&self, account_id: i32, house_id: i32, area_id: i32)  {
    //
    // }

    pub async fn get_account_info() -> Option<AccountInfo> {
        None
    }

    pub async fn get_user_info(&self, account_id: i32) -> Option<UserInfo> {
        None
    }

    pub async fn get_all_house_info(&self, account_id: i32) -> Vec<HouseInfo> {
        Vec::new()
    }

    pub async fn get_all_area_info(&self, account_id: i32) -> Vec<AreaInfo> {
        Vec::new()
    }

    pub async fn get_device_info(&self, account_id: i32, device_id: i32) -> Option<DeviceInfo> {
        None
    }

    pub async fn get_account_id_password_hash(&self, username: &str) -> Result<(i32, String), PoolError> {
        let client = self;
        let row = client.query_one(
            "SELECT password_hash, account_id FROM account WHERE username = $1",
            &[&username],
        ).await?;
        Ok((row.get(1), row.get(0)))
    }

    pub async fn update_account_last_login(&self, account_id: i32, username: String) -> Result<String, PoolError> {
        let client = self;
        client.execute(
            "UPDATE account SET last_login=CURRENT_TIMESTAMP WHERE account_id = $1",
            &[&account_id],
        ).await?;
        let token = create_token(username, Role::User, account_id);
        Ok(token)
    }

    pub async fn add_account<'a>(&self, username: &'a str, password_hash: &'a str, salt: &[u8]) -> Result<(), PoolError> {
        let client = self;
        client.execute("INSERT INTO account \
                (username, password_hash, salt) \
                VALUES ($1, $2, $3)", &[&username, &password_hash, &hex::encode(salt)],
        ).await?;
        Ok(())
    }
}

#[derive(Default)]
struct Cache {}

pub struct CachedDataBase {
    db: DataBase,
    cache: Cache,
}

impl CachedDataBase {
    pub async fn new(config: &DataBaseConfig) -> Self {
        Self {
            db: DataBase::new(config).await,
            cache: Cache::default(),
        }
    }
}