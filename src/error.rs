use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("No limit provided")]
    Limit,
    #[error("No goal provided")]
    Goal,
    #[error("No belt level provided")]
    BeltLevel,
    #[error("No extractor level provided")]
    ExtractorLevel,
    #[error("No building level provided {0}")]
    BuildingMissing(&'static str),
    #[error ("Unreachable number with ops provided")]
    UnreachableNumber,
}
