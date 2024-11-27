use crate::db::database::Session;
use crate::dto::http::request::SceneAdd;
use tokio_postgres::Error;

impl Session {
    // TODO:
    pub async fn add_scene(&mut self, data: SceneAdd) -> Result<(), Error> {
        Ok(())
    }

    pub async fn delete_scene(&mut self, scene_id: i32) -> Result<(), Error> {
        Ok(())
    }

    pub async fn get_scene(&self, account_id: i32) -> Result<(), Error> {
        Ok(())
    }
}
