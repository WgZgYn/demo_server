use crate::db::database::Session;
use crate::dto::entity::simple::{AccountInfo, UserInfo};
use crate::dto::http::request::{AreaUpdate, UserInfoUpdate};
use crate::security::auth::{create_token, Role};
use deadpool_postgres::{GenericClient, PoolError};
use log::debug;
use tokio_postgres::Error;

impl Session {
    pub async fn get_user_info(&self, account_id: i32) -> Result<UserInfo, PoolError> {
        let row = self
            .0
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
        self.0
            .execute(
                "UPDATE account SET last_login=CURRENT_TIMESTAMP WHERE account_id = $1",
                &[&account_id],
            )
            .await?;
        let token = create_token(username, Role::User, account_id);
        Ok(token)
    }

    pub async fn add_account(
        &self,
        username: &str,
        password_hash: &str,
        salt: &[u8],
    ) -> Result<(), PoolError> {
        self.0
            .execute(
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
        self.0
            .execute(
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

    pub async fn delete_account(&mut self, account_id: i32) -> Result<(), PoolError> {
        let trans = self.0.transaction().await?;
        trans
            .execute("DELETE FROM member WHERE account_id = $1", &[&account_id])
            .await?;
        trans
            .execute("DELETE FROM account WHERE account_id = $1", &[&account_id])
            .await?;
        trans.commit().await?;
        Ok(())
    }

    pub async fn update_area_info(&self, area_update: AreaUpdate) -> Result<u64, PoolError> {
        // TODO:
        Ok(0)
    }
}
