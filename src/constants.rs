#[cfg(debug_assertions)]
pub(crate) const HOST: &str = "http://localhost:8000";
#[cfg(not(debug_assertions))]
pub(crate) const HOST: &str = "https://adam.qpon";

#[cfg(debug_assertions)]
pub(crate) const MAIN_HOST: &str = "http://localhost:8000";
#[cfg(not(debug_assertions))]
pub(crate) const MAIN_HOST: &str = "https://adamperkowski.dev";

pub(crate) const GIT_URL: &str = "https://github.com/adamperkowski";
pub(crate) const LEGAL_URL: &str =
    "https://github.com/adamperkowski/website/blob/old/templates/pages/legal.tera";

pub(crate) const ENV_VARS: &[&str] = &["JELLYFIN_SECRET"];
