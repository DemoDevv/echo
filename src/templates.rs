use std::path::PathBuf;

use crate::{
    composants::Composant,
    utils::{create_file_with_content, find_api_file, publish_new_module},
};

pub fn repository(file_name: &str) -> String {
    let mut name = file_name.to_lowercase();
    let type_name = name.get_mut(0..1).unwrap().to_uppercase() + name.get_mut(1..).unwrap();
    let name_upper_plural = type_name.clone() + "s";

    r#"use diesel::prelude::*;

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

// TODO: You have to create your model REPLACE_TYPE in the models dir
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

fn mod_repository(file_name: &str) -> String {
    let name_plural = file_name.to_lowercase() + "s";
    r#"pub mod NAME_repository;"#.replace("NAME", &name_plural)
}

pub(crate) fn create_repository(api_folder: &std::path::Path, file: Composant) {
    let file_name = if let Composant::Repository(name) = file {
        name
    } else {
        return;
    };

    let repositories_folder_path = api_folder.join("db/src/repositories");

    if !repositories_folder_path.exists() {
        println!("Le dossier repositories n'existe pas.");
        return;
    }

    let name_plural = file_name.to_lowercase() + "s";
    let repository_file_name = format!("{}_repository.rs", name_plural);
    let repository_path_buf = repositories_folder_path.join(repository_file_name);

    if repository_path_buf.exists() {
        println!("Ce repository existe déjà.");
        return;
    }

    create_file_with_content(&repository_path_buf, &repository(&file_name))
        .expect("Erreur lors de la création du nouveau repository");

    let mod_repository_path = find_api_file(
        api_folder,
        Composant::Repository("".to_string()),
        "repositories.rs",
    )
    .expect("Le fichier repositories.rs est introuvable");

    publish_new_module(&mod_repository_path, &mod_repository(&file_name))
        .expect("Erreur lors de l'écriture dans le fichier repositories.rs");
}

pub(crate) fn create_handler(api_folder: &std::path::Path, file: Composant) {
    unimplemented!()
}
