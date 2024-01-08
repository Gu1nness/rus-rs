use super::schema::urls;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rand::{self, Rng};


#[derive(Deserialize, Clone, Debug)]
pub struct UrlSchema {
    pub short: Option<String>,
    pub destination: String, 
}


#[derive(Queryable, Insertable, Selectable, Serialize, AsChangeset, Clone, Debug)]
#[diesel(table_name = urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub short: String,
    pub destination: String,
}

fn generate_short_url() -> String {
    const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let size = 24;
    let mut id = String::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        id.push(BASE62[rng.gen::<usize>() % 62] as char);
    }
    id
}

impl Url {
    pub fn from_schema(schema: &Box<UrlSchema>) -> Self{
        Url {
            short: match &schema.short {
                Some(value) => String::from(value),
                None => generate_short_url(),
            },
            destination: String::from(&schema.destination),
        }
    }
}
