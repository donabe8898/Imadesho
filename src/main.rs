mod imp;

use poise::{framework, serenity_prelude as serenity};
use serde::{Deserialize, Serialize};
use std::env;
use tokio;
use tokio_postgres::{tls::TlsConnect, Client, Connection, Error, NoTls};

// Poise用
// strct.rsへ移動

/*
    TODO: 検索・表示用コマンド実装
        - 全プロジェクト（テーブル）
        - プロジェクト内の全チケット（レコード）
            - 全レコード
            - 完了済みを除くレコード
            - 

    TODO: 挿入コマンド実装
        - null許容の値に注意する

    TODO: 完了、削除機能実装
        - テーブルまるごと削除しない限り、削除されない

*/

struct Data {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    /* Bot起動フェーズ */
    dotenv::dotenv().ok();
    env_logger::init();
    let token = env::var("TOKEN").expect("missing get token");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![imp::test()],
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands)
                    .await
                    .unwrap();
                Ok(imp::Data {})
            })
        });
    framework.run().await.unwrap();
    Ok(())
}

/* 参考
    - 【Rust】 Rust + PostgreSQL + tokio_postgresでDBアクセスする方法
        - https://qiita.com/SakasuRobo/items/a72f916c1e1c8fb63de7
*/
