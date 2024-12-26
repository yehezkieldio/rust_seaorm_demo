use entity::post;
use migration::DbErr;
use sea_orm::EntityTrait;
use seaorm_demo::connect;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = connect().await?;

    let posts: Vec<post::Model> = post::Entity::find().all(&db).await?;

    println!("All the posts in the db: ");
    for post in posts {
        println!("{:?}", post);
    }

    Ok(())
}
