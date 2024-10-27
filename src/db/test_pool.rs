
use log::info;
use crate::db::create_connection_pool;

async fn connect_database() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_connection_pool().await?;
    let client = pool.get().await?;
    // 执行查询
    let rows = client.query("SELECT * FROM student", &[]).await?;
    // 处理查询结果
    for row in rows {
        let id: &str = row.get(0);
        let name: &str = row.get(1);

        info!("找到记录: id = {}, name = {}", id, name);
    }
    Ok(())
}
