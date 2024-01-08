extern crate diesel;
extern crate rocket;

use crate::catchers::already_present;
use crate::models::*;
use rocket::fairing::AdHoc;
use rocket::response::{status::{Created, BadRequest}, Redirect};
use rocket::serde::json::{Json, Value};
use rocket::{get, post};

use rocket_db_pools::diesel::{prelude::*, PgPool};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("rus")]
pub struct Db(PgPool);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        rocket.attach(Db::init())
    })
}

#[post("/", data = "<url>")]
pub async fn create(mut db: Connection<Db>, url: Json<UrlSchema>) -> Result<Created<Json<Url>>, BadRequest<Json<Value>>> {
    use crate::schema::urls::dsl::*;
    let new_entry = Url::from_schema(&Box::new(url.into_inner()));
    //let url_bis = db.transaction(|mut conn| Box::pin(async move {
    //    diesel::insert_into(urls).values(&new_entry).returning(Url::as_returning()).execute(&mut conn).await?;
    //    Ok::<_, diesel::result::Error>(new_entry)
    //})).await?;
    match diesel::insert_into(urls)
        .values(&new_entry)
        .returning(Url::as_returning())
        .get_result(&mut db).await {
            Ok(value) => Ok(Created::new("/").body(Json(value))),
            Err(_) => Err(BadRequest(already_present())),
        }
    
}

#[get("/<id>")]
pub async fn get(mut db: Connection<Db>, id: &str) -> Option<Redirect> {
    use crate::schema::urls::dsl::*;
    let result: Option<Url> = urls
        .select(Url::as_returning())
        .filter(short.eq(id))
        .first(&mut db)
        .await
        .ok();
    match result {
        Some(dest) => Some(Redirect::to(dest.destination)),
        None => None,
    }
}
