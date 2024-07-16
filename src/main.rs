use clap::{Arg, Command};

mod templates;

fn main() {
    let matches = Command::new("echo")
        .version("1.0")
        .author("DemoDevv")
        .about("echo est l'outil de production de EF-backend pour générer des fichiers de code")
        .subcommand(
            Command::new("create")
                .about("Crée un nouveau composant")
                .arg(
                    Arg::new("composant")
                        .help("Le type de composant à créer")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("name")
                        .help("Le nom du composant à créer")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    if let Some(create_matches) = matches.subcommand_matches("create") {
        let file_composant = create_matches.get_one::<String>("composant").unwrap();
        let file_name = create_matches.get_one::<String>("name").unwrap();

        println!("Création d'un {} nommé '{}'", file_composant, file_name);

        // trouver le dossier api au niveau du pwd
        let api_folder = std::env::current_dir().unwrap().join("api");

        // if !api_folder.exists() {
        //     println!("Le dossier api n'existe pas");
        //     return;
        // }

        match file_composant.as_str() {
            "repository" => {
                // un repository et un type vide
                // ajoute au fichier model.rs la publication du nouveau module et de même pour le repository
                templates::create_repository(&api_folder, &file_name);
            }
            "extractor" => {
                unimplemented!("Création d'extractor non implémentée")
            }
            "handler" => {
                // créer un handler REST ou alors un handler vide avec uniquement la fonction de configuration des routes vides
                // a voir si on peut ajouter l'appel à la fonction de configuration des routes dans le fichier routes.rs
                templates::create_handler(&api_folder, &file_name);
            }
            "middleware" => {
                unimplemented!("Création de middleware non implémentée")
            }
            _ => {
                println!("Type de composant inconnu");
            }
        }
    }
}
