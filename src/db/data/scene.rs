use crate::db::database::Session;
use crate::dto::http::request::SceneAdd;
use crate::service::event::Scene;
use deadpool_postgres::GenericClient;
use serde_json::Value;
use tokio_postgres::Error;

impl Session {
    pub async fn add_scene(&self, data: SceneAdd) -> Result<(), Box<dyn std::error::Error>> {
        let triggers = serde_json::to_value(data.triggers)?;
        let actions = serde_json::to_value(data.actions)?;

        self.0.execute(
            "INSERT INTO scene (house_id, scene_name, triggers, actions) VALUES ($1, $2, $3, $4)",
            &[&data.house_id, &data.scene_name, &triggers, &actions]).await?;

        Ok(())
    }

    pub async fn delete_scene(&mut self, scene_id: i32) -> Result<u64, Error> {
        self.0
            .execute("DELETE FROM scene WHERE scene_id = $1", &[&scene_id])
            .await
    }

    pub async fn get_scene(&self, house_id: i32) -> Result<Vec<Scene>, Box<dyn std::error::Error>> {
        let rows = self.0.query(
            "SELECT scene_id, house_id, scene_name, triggers, actions FROM scene WHERE house_id = $1", &[&house_id],
        ).await?;

        let mut res = Vec::with_capacity(rows.len());
        for row in rows {
            let scene = Scene {
                scene_id: row.get("scene_id"),
                scene_name: row.get("scene_name"),
                house_id: row.get("house_id"),
                triggers: {
                    let value: Value = row.get("triggers");
                    serde_json::from_value(value)?
                },
                actions: {
                    let value: Value = row.get("actions");
                    serde_json::from_value(value)?
                },
            };
            res.push(scene);
        }
        Ok(res)
    }
}
