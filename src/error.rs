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
    #[error("No adder level provided")]
    AdderLevel,
    #[error("No multiplier level provided")]
    MultiplierLevel,
}
