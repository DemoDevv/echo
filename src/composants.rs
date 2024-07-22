#[derive(Clone)]
pub(crate) enum Composant {
    Repository(String),
    Type,
    Model,
    Handler(String),
}
