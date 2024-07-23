use std::{
    fs::File,
    io::{Error, ErrorKind, Result as IoResult, Write},
    path::{Path, PathBuf},
};

use crate::composants::Composant;

pub(crate) fn find_api_file(
    api_folder: &Path,
    composant_type: Composant,
    file_name: &str,
) -> Result<PathBuf, Error> {
    match composant_type {
        Composant::Repository(_) => {
            let db_path = api_folder.join("db/src");
            let repository_path = db_path.clone().join("repositories");

            if db_path.join(file_name).exists() {
                return Ok(db_path.join(file_name));
            } else if repository_path.join(file_name).exists() {
                return Ok(repository_path.join(file_name));
            }

            return Err(Error::new(ErrorKind::NotFound, "Fichier non trouvé"));
        }
        Composant::Type => {
            let type_path = api_folder.join("types/src");

            if type_path.join(file_name).exists() {
                return Ok(type_path.join(file_name));
            }

            return Err(Error::new(ErrorKind::NotFound, "Fichier non trouvé"));
        }
        Composant::Model => {
            let db_path = api_folder.join("db/src");
            let model_path = db_path.clone().join("models");

            if db_path.join(file_name).exists() {
                return Ok(db_path.join(file_name));
            } else if model_path.join(file_name).exists() {
                return Ok(model_path.join(file_name));
            }

            return Err(Error::new(ErrorKind::NotFound, "Fichier non trouvé"));
        }
        Composant::Handler(_) => {
            let handler_path = api_folder.join("handlers/src");

            if handler_path.join(file_name).exists() {
                return Ok(handler_path.join(file_name));
            }

            return Err(Error::new(ErrorKind::NotFound, "Fichier non trouvé"));
        }
    }
}

pub(crate) fn publish_new_module(path: &Path, template: &str) -> IoResult<()> {
    let mut file = File::options().append(true).open(path)?;
    writeln!(&mut file, "{}", template)
}

pub(crate) fn create_file_with_content(path: &Path, template: &str) -> IoResult<()> {
    let mut file = File::options().create(true).write(true).open(path)?;
    write!(&mut file, "{}", template)
}
