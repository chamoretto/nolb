#[derive(Debug, derive_more::Display)]
pub(crate) enum FileFormat {
    #[display(fmt = "JSON")]
    Json,
    #[display(fmt = "YAML")]
    Yaml,
    #[display(fmt = "TOML")]
    Toml,
}
