use crate::composants::Composant;

pub(crate) fn create_repository(api_folder: &std::path::Path, file: Composant) {
    println!("Création d'un repository");

    let file_name = if let Composant::Repository(name) = file {
        name
    } else {
        return;
    };

    let mut name = file_name.to_lowercase();

    let type_name = name.get_mut(0..1).unwrap().to_uppercase() + name.get_mut(1..).unwrap();

    let name_upper_plural = type_name.clone() + "s";

    // template d'un fichier repository.rs
    let template_repository_file = r#"use diesel::prelude::*;

use crate::connection::Pool;
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
    .replace("REPLACE_NAME", &name);

    println!("{}", template_repository_file);
}

pub(crate) fn create_handler(api_folder: &std::path::Path, file: Composant) {
    println!("Création d'un handler");
}
