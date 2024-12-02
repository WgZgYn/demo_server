use crate::db::database::Session;
use crate::db::DataBase;
use crate::dto::http::response::AccountDevices;
use crate::utils::config::DataBaseConfig;
use deadpool_postgres::{Pool, PoolError};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct Cache {
    device_mac2id: Arc<RwLock<HashMap<String, i32>>>,
    device_id2mac: Arc<RwLock<HashMap<i32, String>>>,
    account_id2device_ids: Arc<RwLock<HashMap<i32, HashSet<i32>>>>,
    pub device_id2account_ids: Arc<RwLock<HashMap<i32, HashSet<i32>>>>,
}

pub struct CacheSession {
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
    pub async fn get_account_devices(
        &self,
        account_id: i32,
        username: String,
    ) -> Result<AccountDevices, PoolError> {
        let ads = self
            .session
            .get_account_devices(account_id, username)
            .await?;
        let account_id = ads.account_info.account_id;
        for hd in &ads.houses_devices {
            for ad in &hd.areas_devices {
                for d in &ad.devices {
                    let mac = d.efuse_mac.clone();
                    let device_id = d.device_id;
                    let mut guard = self.cache.device_mac2id.write().await;
                    guard.insert(mac, device_id);
                    let mut guard = self.cache.account_id2device_ids.write().await;
                    guard.entry(account_id).or_default().insert(device_id);
                    let mut guard = self.cache.device_id2account_ids.write().await;
                    guard.entry(device_id).or_default().insert(account_id);
                }
            }
        }
        Ok(ads)
    }

    pub async fn get_device_mac_by_id(&self, device_id: i32) -> Result<String, PoolError> {
        let guard = self.cache.device_id2mac.read().await;
        if let Some(v) = guard.get(&device_id) {
            return Ok(v.clone());
        }
        let mac = self.session.get_device_mac_by_id(device_id).await?;
        let mut guard = self.cache.device_mac2id.write().await;
        guard.insert(mac.clone(), device_id);
        let mut guard = self.cache.device_id2mac.write().await;
        guard.insert(device_id, mac.clone());
        Ok(mac)
    }

    pub async fn get_device_id_by_mac(&self, device_mac: &str) -> Result<i32, PoolError> {
        {
            let guard = self.cache.device_mac2id.read().await;
            if let Some(v) = guard.get(device_mac) {
                return Ok(*v);
            }
        }// necessary to drop the guard
        let device_id = self.session.get_device_id_by_mac(device_mac).await?;
        {
            let mut guard = self.cache.device_mac2id.write().await;
            guard.insert(device_mac.to_string(), device_id);
        }
        {
            let mut guard = self.cache.device_id2mac.write().await;
            guard.insert(device_id, device_mac.to_string());
        }
        Ok(device_id)
    }

    pub async fn get_account_ids_by_device_id(&self, device_id: i32) -> Result<HashSet<i32>, PoolError> {
        {
            let guard = self.cache.device_id2account_ids.read().await;
            if let Some(v) = guard.get(&device_id) {
                return Ok(v.clone());
            }
        }
        let data = self.session.get_account_ids_by_device_id(device_id).await?;
        {
            let mut guard = self.cache.device_id2account_ids.write().await;
            guard.insert(device_id, data.clone());
            Ok(data)
        }
    }
}

pub struct CachedDataBase {
    db: DataBase,
    pub cache: Cache,
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

    pub fn from(pool: Pool) -> Self {
        Self {
            db: DataBase::from(pool),
            cache: Cache::default(),
        }
    }

    pub async fn get_session(&self) -> Result<CacheSession, PoolError> {
        self.db.get_session().await.map(|s| CacheSession {
            session: s,
            cache: self.cache.clone(),
        })
    }
}
