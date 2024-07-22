pub(crate) fn mod_common(name: &str) -> String {
    r#"pub mod NAME;"#.replace("NAME", name)
}

pub(crate) fn mod_repository(name_plural: &str) -> String {
    r#"pub mod NAME_repository;"#.replace("NAME", &name_plural)
}
