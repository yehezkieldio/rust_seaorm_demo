use entity::post;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use seaorm_demo::connect;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = connect().await?;

    let post = post::Entity::find_by_id(1).one(&db).await?;
    let mut post: post::ActiveModel = post.unwrap().into();
    post.title = Set("Updated title".to_owned());
    let post: post::Model = post.update(&db).await?;

    println!("Post updated: {:?}", post);

    Ok(())
}
