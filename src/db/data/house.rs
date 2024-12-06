use crate::db::database::Session;
use crate::dto::entity::simple::HouseInfo;
use crate::dto::http::request::HouseUpdate;
use deadpool_postgres::{GenericClient, PoolError};
use tokio_postgres::Error;

impl Session {
    pub async fn update_house_info(&self, house_id: i32, data: HouseUpdate) -> Result<(), PoolError> {
        self.0.execute("UPDATE house SET house_name = $1 WHERE house_id = $2", &[&data.house_name, &house_id]).await?;
        Ok(())
    }

    pub async fn get_house_info(
        &self,
        house_id: i32,
    ) -> Result<HouseInfo, Box<dyn std::error::Error>> {
        let row = self
            .0
            .query_one("SELECT * FROM house WHERE house_id = $1", &[&house_id])
            .await?;
        Ok(HouseInfo {
            house_id,
            house_name: row.get("house_name"),
        })
    }

    pub async fn is_house_created_by(&self, house_id: i32, account_id: i32) -> Result<bool, PoolError> {
        let creator = self.0.query_one("SELECT created_by FROM house WHERE house_id = $1", &[&house_id]).await?;
        let created_by: i32 = creator.get("created_by");
        Ok(created_by == account_id)
    }
    pub async fn delete_house(&self, house_id: i32) -> Result<u64, Error> {
        self.0
            .execute("DELETE FROM house WHERE house_id = $1", &[&house_id])
            .await
    }
    pub async fn add_house(&mut self, house_name: &str, account_id: i32) -> Result<i32, Error> {
        let trans = self.0.transaction().await?;
        let row = trans
            .query_one(
                "INSERT INTO house (house_name, created_by) VALUES ($1, $2) RETURNING house_id",
                &[&house_name, &account_id],
            )
            .await?;
        let house_id: i32 = row.get("house_id");
        trans
            .query_one(
                "INSERT INTO member (house_id, account_id) VALUES ($1, $2) ",
                &[&house_id, &account_id],
            )
            .await?;
        trans.commit().await?;
        Ok(house_id)
    }

    pub async fn get_all_house_info(&self, account_id: i32) -> Result<Vec<HouseInfo>, PoolError> {
        let rows = self
            .0
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
}
