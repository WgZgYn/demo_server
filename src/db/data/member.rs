use crate::db::database::Session;
use crate::dto::entity::simple::{AccountInfo, HouseInfo, HouseMember, MemberInfo};
use deadpool_postgres::GenericClient;
use tokio_postgres::Error;

impl Session {
    pub async fn delete_member(&self, house_id: i32, account_id: i32) -> Result<u64, Error> {
        self.0.execute(
            "DELETE FROM member WHERE house_id = $1 AND account_id = $2",
            &[&house_id, &account_id],
        ).await
    }

    pub async fn add_member(&self, house_id: i32, account_id: i32) -> Result<u64, Error> {
        self.0.execute(
            "INSERT INTO member (house_id, account_id) VALUES ($1, $2)",
            &[&house_id, &account_id],
        ).await
    }

    pub async fn get_member(&self, account_id: i32) -> Result<MemberInfo, Error> {
        let rows = self.0.query("\
        SELECT h.house_id, h.house_name, a.account_id, a.username \
        FROM (SELECT house_id FROM member m WHERE m.account_id = $1) \
        JOIN member USING(house_id) \
        JOIN house h USING(house_id) \
        JOIN account a USING(account_id) ", &[&account_id]).await?;
        let mut houses = Vec::<HouseMember>::new();
        for row in &rows {
            let account_id: i32 = row.get("account_id");
            let username: String = row.get("username");
            let house_id: i32 = row.get("house_id");
            let house_name: String = row.get("house_name");

            match houses.iter_mut().find(|h| h.house_info.house_id == house_id) {
                Some(v) => {
                    v.account.push(AccountInfo { account_id, username })
                }
                None => {
                    houses.push(HouseMember {
                        house_info: HouseInfo {
                            house_id,
                            house_name,
                        },
                        account: vec![AccountInfo {
                            account_id,
                            username
                        }],
                    })
                }
            }
        }
        Ok(MemberInfo { houses_member: houses })
    }

    pub async fn get_member_by_house_id(&self, house_id: i32) -> Result<MemberInfo, Error> {
        let rows = self.0.query("\
        SELECT h.house_id, h.house_name, a.account_id, a.username \
        FROM (SELECT house_id FROM member m WHERE m.house_id = $1) \
        JOIN member USING(house_id) \
        JOIN house h USING(house_id) \
        JOIN account a USING(account_id) ", &[&house_id]).await?;
        let mut houses = Vec::<HouseMember>::new();
        for row in &rows {
            let account_id: i32 = row.get("account_id");
            let username: String = row.get("username");
            let house_id: i32 = row.get("house_id");
            let house_name: String = row.get("house_name");

            match houses.iter_mut().find(|h| h.house_info.house_id == house_id) {
                Some(v) => {
                    v.account.push(AccountInfo { account_id, username })
                }
                None => {
                    houses.push(HouseMember {
                        house_info: HouseInfo {
                            house_id,
                            house_name,
                        },
                        account: vec![AccountInfo {
                            account_id,
                            username
                        }],
                    })
                }
            }
        }
        Ok(MemberInfo { houses_member: houses })
    }

    pub async fn get_account_ids_by_house_id(&self, house_id: i32) -> Result<Vec<i32>, Error> {
        self.0.query("SELECT account_id FROM member WHERE house_id = $1", &[&house_id])
            .await
            .map(|rows| rows.into_iter().map(|row| row.get("account_id")).collect())
    }

    pub async fn get_house_ids_by_account_id(&self, account_id: i32) -> Result<Vec<i32>, Error> {
        self.0.query("SELECT house_id FROM member WHERE account_id = $1", &[&account_id])
            .await
            .map(|rows| rows.into_iter().map(|row| row.get("account_id")).collect())
    }

    pub async fn is_member(&self, account_id: i32, house_id: i32) -> bool {
        self.0.query_one("SELECT 1 FROM member WHERE account_id = $1 AND house_id = $2", &[&account_id, &house_id]).await.is_ok()
    }
}
