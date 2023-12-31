use poise::serenity_prelude::{self as serenity, client, FutureExt};
use serde::{Deserialize, Serialize};
use tokio;
use tokio_postgres::{
    tls::{NoTlsStream, TlsConnect},
    Client, Connection, Error, NoTls, Row, Socket,
};
// type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct Data {}
type Context<'a> = poise::Context<'a, Data, Error>;

// DB test command
#[poise::command(slash_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    /* 返答用string */
    let mut response = String::new();
    /* 接続実行 */
    let (client, conn) = db_conn().await?;

    // 接続タスク実行
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection err: {}", e);
        }
    });

    /* DBテーブル取得 */
    let rows = client.query("select * from users", &[]).await?;
    // 表示とdiscord返信
    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        response += &format!("id: {}, name: {}\n", id, name);
    }
    let _ = ctx.say(response).await;
    Ok(())
}

/* データベース接続 */
pub async fn db_conn() -> Result<(Client, Connection<Socket, NoTlsStream>), Error> {
    let (client, conn) = tokio_postgres::Config::new()
        .user("postgres")
        .password("password")
        .host("localhost")
        .port(5432)
        .dbname("testdb")
        .connect(tokio_postgres::NoTls)
        .await?;

    Ok((client, conn))
}
