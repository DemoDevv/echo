mod composants;
mod publish;

use std::{collections::HashMap, path::PathBuf, process::exit};

use crate::{
    composants::Composant,
    utils::{create_file_with_content, find_api_file, publish_new_module},
};

pub(crate) fn create_repository(api_folder: &std::path::Path, file: Composant) {
    let file_name = if let Composant::Repository(name) = file {
        name
    } else {
        return;
    };

    let repositories_folder_path = api_folder.join("db/src/repositories");
    let model_folder_path = api_folder.join("db/src/models");
    let types_folder_path = api_folder.join("types/src");

    let folders = [
        &repositories_folder_path,
        &model_folder_path,
        &types_folder_path,
    ];

    folders.iter().for_each(|folder| {
        if !folder.exists() {
            println!(
                "le dossier {} n'existe pas",
                folder.file_name().unwrap().to_str().unwrap()
            );
            exit(1);
        }
    });

    let name_plural = file_name.to_lowercase() + "s";
    let mut name = file_name.to_lowercase();
    let type_name = name.get_mut(0..1).unwrap().to_uppercase() + name.get_mut(1..).unwrap();

    let repository_file_name = format!("{}_repository.rs", name_plural);
    let repository_path_buf = repositories_folder_path.join(repository_file_name);

    let type_file_name = format!("{}.rs", &file_name);
    let type_path_buf = types_folder_path.join(&type_file_name);

    let model_file_name = format!("{}.rs", &file_name);
    let model_path_buf = model_folder_path.join(&model_file_name);

    let mut hash_map_file_check: HashMap<&PathBuf, &str> = HashMap::new();
    hash_map_file_check.insert(&repository_path_buf, "Ce repository existe déjà");
    hash_map_file_check.insert(&type_path_buf, "Ce type existe déjà");
    hash_map_file_check.insert(&model_path_buf, "Ce model existe déjà");

    hash_map_file_check.iter().for_each(|element| {
        if element.0.exists() {
            println!("{}", element.1);
            exit(1);
        }
    });

    let mod_repository_path = find_api_file(
        api_folder,
        Composant::Repository("".to_string()),
        "repositories.rs",
    )
    .expect("Le fichier repositories.rs est introuvable");
    let mod_model_path = find_api_file(api_folder, Composant::Model, "models.rs")
        .expect("Le fichier models.rs est introuvable");
    let mod_type_path = find_api_file(api_folder, Composant::Type, "lib.rs")
        .expect("Le fichier lib.rs est introuvable");

    create_file_with_content(&repository_path_buf, &composants::repository(&file_name))
        .expect("Erreur lors de la création du nouveau repository");
    create_file_with_content(
        &model_path_buf,
        &composants::empty_model(&type_name, &name_plural),
    )
    .expect("Erreur lors de la création du nouveau model");
    create_file_with_content(&type_path_buf, &composants::empty_type(&type_name))
        .expect("Erreur lors de la création du nouveau type");

    publish_new_module(&mod_repository_path, &publish::mod_repository(&name_plural))
        .expect("Erreur lors de l'écriture dans le fichier repositories.rs");
    publish_new_module(&mod_model_path, &publish::mod_common(&file_name))
        .expect("Erreur lors de l'écriture dans le fichier models.rs");
    publish_new_module(&mod_type_path, &publish::mod_common(&file_name))
        .expect("Erreur lors de l'écriture dans le fichier lib.rs");
}

pub(crate) fn create_handler(api_folder: &std::path::Path, file: Composant) {
    unimplemented!()
}
