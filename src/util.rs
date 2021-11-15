pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const GIT_BUILD_HASH: &str = env!("GIT_HASH");
pub const GIT_LOG: &str = env!("GIT_LOG");

pub const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://ashisbored.github.io)",
);

#[derive(Debug, thiserror::Error)]
pub enum StarlightError {
    #[error("not found")]
    NotFound,
    #[error("unknown server")]
    UnknownServer,
}
