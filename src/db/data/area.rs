use crate::db::database::Session;
use crate::dto::entity::simple::AreaInfo;
use tokio_postgres::Error;

impl Session {
    pub async fn get_all_area_info(&self, account_id: i32) -> Result<Vec<AreaInfo>, Error> {
        let rows = self
            .0
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

    pub async fn add_area_by(
        &self,
        area_name: &str,
        house_id: i32,
        account_id: i32,
    ) -> Result<u64, Error> {
        self.0
            .execute(
                "INSERT INTO area (area_name, house_id, created_by) VALUES ($1, $2, $3)",
                &[&area_name, &house_id, &account_id],
            )
            .await
    }
    pub async fn add_house_by(&self, house_name: &str, account_id: i32) -> Result<u64, Error> {
        self.0
            .execute(
                "INSERT INTO house (house_name, created_by) VALUES ($1, $2)",
                &[&house_name, &account_id],
            )
            .await
    }

    pub async fn add_area(
        &self,
        area_name: &str,
        house_id: i32,
        account_id: i32,
    ) -> Result<i32, Error> {
        let row = self.0
            .query_one(
                "INSERT INTO area (house_id, created_by, area_name) VALUES ($1, $2, $3) RETURNING area_id",
                &[&house_id, &account_id, &area_name],
            )
            .await?;
        let area_id: i32 = row.get("area_id");
        Ok(area_id)
    }

    pub async fn get_area_info(
        &self,
        area_id: i32,
    ) -> Result<AreaInfo, Box<dyn std::error::Error>> {
        let row = self
            .0
            .query_one("SELECT * FROM area WHERE area_id = $1", &[&area_id])
            .await?;
        Ok(AreaInfo {
            area_id,
            area_name: row.get("area_name"),
        })
    }

    pub async fn delete_area(&self, area_id: i32) -> Result<u64, Error> {
        self.0
            .execute("DELETE FROM area WHERE area_id = $1", &[&area_id])
            .await
    }

    pub async fn delete_area_r(&mut self, area_id: i32) -> Result<(), Error> {
        let trans = self.0.transaction().await?;
        trans
            .execute("DELETE FROM device WHERE area_id = $1", &[&area_id])
            .await?;
        trans
            .execute("DELETE FROM area WHERE area_id = $1", &[&area_id])
            .await?;
        trans.commit().await
    }
}
