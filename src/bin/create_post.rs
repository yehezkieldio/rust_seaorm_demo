use entity::post;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, Set};
use seaorm_demo::connect;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = connect().await?;

    let post = post::ActiveModel {
        title: Set(String::from("Title 1")),
        text: Set(String::from("Text 1")),
        ..Default::default()
    };

    let post: post::Model = post.insert(&db).await?;

    println!("Post created: {:?}", post);

    Ok(())
}
