pub(crate) fn repository(file_name: &str) -> String {
    let mut name = file_name.to_lowercase();
    let type_name = name.get_mut(0..1).unwrap().to_uppercase() + name.get_mut(1..).unwrap();
    let name_upper_plural = type_name.clone() + "s";

    r#"use diesel::prelude::*;

use crate::connection::Pool;
use crate::models::REPLACE_NAME::REPLACE_TYPE;

use api_errors::ServiceError;
use api_types::REPLACE_NAME::NewREPLACE_TYPE;

use crate::repository::{Repository, RepositoryResult};

#[derive(Clone)]
pub struct REPLACE_NAME_PLURIELRepository {
    conn: Pool,
}

impl REPLACE_NAME_PLURIELRepository {
    pub fn new(conn: Pool) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl Repository<REPLACE_TYPE, NewREPLACE_TYPE> for REPLACE_NAME_PLURIELRepository {
    async fn get(&self, id: i32) -> RepositoryResult<REPLACE_TYPE> {
        todo!()
    }

    async fn get_all(&self) -> RepositoryResult<Vec<REPLACE_TYPE>> {
        todo!()
    }

    async fn create(&self, item: &NewREPLACE_TYPE) -> RepositoryResult<REPLACE_TYPE> {
        todo!()
    }

    async fn update(&self, id: i32, item: &REPLACE_TYPE) -> RepositoryResult<REPLACE_TYPE> {
        todo!()
    }

    async fn delete(&self, id: i32) -> RepositoryResult<usize> {
        todo!()
    }
}
        "#
    .replace("REPLACE_NAME_PLURIEL", &name_upper_plural)
    .replace("REPLACE_TYPE", &type_name)
    .replace("REPLACE_NAME", &name)
}

pub(crate) fn empty_type(type_name: &str) -> String {
    r#"#[derive(Serialize, Deserialize)]
pub struct NewTYPE;"#
        .replace("TYPE", type_name)
}

pub(crate) fn empty_model(type_name: &str, pluriel_name: &str) -> String {
    r#"#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = PLURIEL_NAME)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TYPE;"#
        .replace("TYPE", type_name)
        .replace("PLURIEL_NAME", pluriel_name)
}

pub(crate) fn handler() -> String {
    r#"use actix_web::{web, Error, HttpResponse};

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/index").route(web::get().to(index)));
}

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .json("the server is alive."))
}"#
    .to_string()
}
