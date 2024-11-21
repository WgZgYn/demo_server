use crate::db::database::Session;
use crate::dto::http::request::SceneAdd;
use tokio_postgres::Error;

impl Session {
    pub async fn add_scene(&mut self, data: SceneAdd) -> Result<(), Error> {
        Ok(())
    }

    pub async fn delete_scene(&mut self, scene_id: i32) -> Result<(), Error> {
        Ok(())
    }
}
