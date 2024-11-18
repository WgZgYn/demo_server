use std::collections::HashMap;
use crate::api::auth::{create_token, Role};
use crate::db::create_connection_pool;
use crate::dto::entity::simple::{
    AccountInfo, AreaInfo, DeviceInfo, DeviceType, HouseInfo, UserInfo,
};
use crate::dto::http::response::{AccountDevices, AreaDevices, HouseDevices};
use crate::utils::config::DataBaseConfig;
use deadpool_postgres::{Pool, PoolError};
use std::ops::Deref;
use std::sync::Arc;
use log::debug;
use tokio_postgres::Error;
use crate::dto::http::request::UserInfoUpdate;

pub struct Session(deadpool_postgres::Client);

// impl Deref for Session {
//     type Target = deadpool_postgres::Client;
// 
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

pub struct DataBase {
    pool: Pool,
}

impl DataBase {
    pub async fn new(config: &DataBaseConfig) -> Self {
        Self {
            pool: create_connection_pool(config).await.unwrap(),
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
    pub async fn get_houses_devices(
        &self,
        account_id: i32,
    ) -> Result<Vec<HouseDevices>, PoolError> {
        let mut houses_devices = Vec::<HouseDevices>::new();
        let rows = self.0
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
                continue
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
                continue
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

    pub async fn get_account_info_by_id(&self, account_id: i32) -> Result<AccountInfo, PoolError> {
        let client = &self.0;
        let row = client
            .query_one(
                "SELECT username FROM account WHERE account_id = $1",
                &[&account_id],
            )
            .await?;
        Ok(AccountInfo {
            account_id,
            username: row.get("username"),
        })
    }

    pub async fn get_user_info(&self, account_id: i32) -> Result<UserInfo, PoolError> {
        let row = self.0
            .query_one(
                "SELECT * FROM user_info WHERE account_id = $1;",
                &[&account_id],
            )
            .await?;
        Ok(UserInfo {
            age: row.get("age"),
            city: row.get("city"),
            email: row.get("email"),
            name: row.get("name"),
            gender: row.get("gender"),
        })
    }

    pub async fn get_all_house_info(&self, account_id: i32) -> Result<Vec<HouseInfo>, PoolError> {
        let rows = self.0
            .query(
                "SELECT * FROM member JOIN house USING(house_id) WHERE account_id = $1;",
                &[&account_id],
            )
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| HouseInfo {
                house_id: row.get("house_id"),
                house_name: row.get("house_name"),
            })
            .collect())
    }

    pub async fn get_all_area_info(&self, account_id: i32) -> Result<Vec<AreaInfo>, PoolError> {
        let rows = self.0
            .query(
                "SELECT * FROM member JOIN area USING(house_id) WHERE account_id = $1;",
                &[&account_id],
            )
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| AreaInfo {
                area_id: row.get("area_id"),
                area_name: row.get("area_name"),
            })
            .collect())
    }
    pub async fn get_account_id_password_hash(
        &self,
        username: &str,
    ) -> Result<(i32, String), PoolError> {
        let client = &self.0;
        let row = client
            .query_one(
                "SELECT password_hash, account_id FROM account WHERE username = $1",
                &[&username],
            )
            .await?;
        Ok((row.get(1), row.get(0)))
    }

    pub async fn update_account_last_login(
        &self,
        account_id: i32,
        username: String,
    ) -> Result<String, PoolError> {
        self.0.execute(
            "UPDATE account SET last_login=CURRENT_TIMESTAMP WHERE account_id = $1",
            &[&account_id],
        )
            .await?;
        let token = create_token(username, Role::User, account_id);
        Ok(token)
    }

    pub async fn add_account<'a>(
        &self,
        username: &'a str,
        password_hash: &'a str,
        salt: &[u8],
    ) -> Result<(), PoolError> {
        self.0.execute(
            "INSERT INTO account \
                (username, password_hash, salt) \
                VALUES ($1, $2, $3)",
            &[&username, &password_hash, &hex::encode(salt)],
        )
            .await?;
        Ok(())
    }

    pub async fn add_user_info(
        &self,
        account_id: i32,
        gender: Option<String>,
        city: Option<String>,
        age: Option<i32>,
        email: Option<String>,
    ) -> Result<u64, Error> {
        self.0.execute(
            "INSERT INTO user_info \
            (account_id, gender, city, age, email) \
            VALUES($1, $2, $3, $4, $5)",
            &[&account_id, &gender, &city, &age, &email],
        )
            .await
    }

    pub async fn update_user_info(
        &self,
        user_info: UserInfoUpdate,
        account_id: i32,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        debug!("{:#?}", &user_info);

        // 获取旧数据，如果用户不存在，则添加新记录
        let old_info = match self.get_user_info(account_id).await {
            Ok(v) => v,
            Err(_) => {
                return Ok(self
                    .add_user_info(
                        account_id,
                        user_info.gender,
                        user_info.city,
                        user_info.age,
                        user_info.email,
                    )
                    .await?)
            }
        };

        // 动态构建 SQL 语句
        let mut query = String::from("UPDATE user_info SET ");
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![];
        let mut param_index = 1; // SQL 参数占位符从 $1 开始

        if let Some(gender) = user_info.gender.as_ref().or(old_info.gender.as_ref()) {
            query.push_str(&format!("gender = ${}, ", param_index));
            params.push(gender); // 引用的生命周期由 as_ref 确保
            param_index += 1;
        }
        if let Some(city) = user_info.city.as_ref().or(old_info.city.as_ref()) {
            query.push_str(&format!("city = ${}, ", param_index));
            params.push(city); // 同上
            param_index += 1;
        }
        if let Some(age) = user_info.age.as_ref().or(old_info.age.as_ref()) {
            query.push_str(&format!("age = ${}, ", param_index));
            params.push(age);
            param_index += 1;
        }
        if let Some(email) = user_info.email.as_ref().or(old_info.email.as_ref()) {
            query.push_str(&format!("email = ${}, ", param_index));
            params.push(email); // 同上
            param_index += 1;
        }

        // 删除多余的逗号和空格，添加 WHERE 子句
        if params.is_empty() {
            return Ok(0); // 如果没有需要更新的字段，则直接返回
        }
        query.truncate(query.len() - 2);
        query.push_str(&format!(" WHERE account_id = ${}", param_index));
        params.push(&account_id);

        debug!("Generated SQL: {}", query);
        debug!("Params: {:?}", params);

        // 执行动态构建的 SQL 语句
        let res = self.0.execute(&query, &params).await?;
        Ok(res)
    }

    pub async fn add_area_by(
        &self,
        area_name: &str,
        house_id: i32,
        account_id: i32,
    ) -> Result<u64, Error> {
        self.0.execute(
            "INSERT INTO area (area_name, house_id, created_by) VALUES ($1, $2, $3)",
            &[&area_name, &house_id, &account_id],
        )
            .await
    }
    pub async fn add_house_by(&self, house_name: &str, account_id: i32) -> Result<u64, Error> {
        self.0.execute(
            "INSERT INTO house (house_name, created_by) VALUES ($1, $2)",
            &[&house_name, &account_id],
        )
            .await
    }

    pub async fn add_device_by(
        &self,
        area_id: i32,
        device_name: &str,
        efuse_mac: &str,
        model_id: i32,
        account_id: i32,
    ) -> Result<u64, Error> {
        self.0.execute(
            "INSERT INTO device\
            (device_name, efuse_mac, area_id, created_by, model_id) \
            VALUES ($1, $2, $3, $4, $5, $6)",
            &[&device_name, &efuse_mac, &area_id, &account_id, &model_id],
        )
            .await
    }

    pub async fn get_device_mac_by_id(&self, device_id: i32) -> Result<String, PoolError> {
        let row = self.0.query_one(
            "SELECT efuse_mac FROM device WHERE device_id = $1",
            &[&device_id],
        )
            .await?;
        Ok(row.get("efuse_mac"))
    }
    pub async fn get_device_id_by_mac(&self, efuse_mac: &str) -> Result<i32, PoolError> {
        let row = self.0.query_one(
            "SELECT device_id FROM device WHERE efuse_mac = $1",
            &[&efuse_mac],
        )
            .await?;
        Ok(row.get("device_id"))
    }

    // TODO: use account_devices_view instead
    pub async fn get_device_info(&self, device_id: i32) -> Result<DeviceInfo, Box<dyn std::error::Error>> {
        let rows = self.0.query("SELECT * FROM device d \
        JOIN device_model m USING(model_id) \
        JOIN device_type t USING(type_id) \
        JOIN device_control c USING(model_id) \
        WHERE device_id = $1", &[&device_id]).await?;
        let mut row_iter = rows.into_iter();
        let first = row_iter.next();
        if let None = first { return Err(format!("no such device {}", device_id).into()); }
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
}

#[derive(Default, Clone)]
struct Cache {
    inner: Arc<HashMap<String, i32>>,
}

struct CacheSession {
    session: Session,
    cache: Cache,
}

impl Deref for CacheSession {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl CacheSession {

}

pub struct CachedDataBase {
    db: DataBase,
    cache: Cache,
}

impl Deref for CachedDataBase {
    type Target = DataBase;
    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl CachedDataBase {
    pub async fn new(config: &DataBaseConfig) -> Self {
        Self {
            db: DataBase::new(config).await,
            cache: Cache::default(),
        }
    }

    pub async fn get_session(&self) -> Result<CacheSession, PoolError> {
        self.db.get_session().await.map(|s| CacheSession { session: s, cache: self.cache.clone() })
    }
}
