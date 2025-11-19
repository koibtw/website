#[cfg(debug_assertions)]
pub(crate) const HOST: &str = "http://localhost:8000";
#[cfg(not(debug_assertions))]
pub(crate) const HOST: &str = "https://adam.qpon";

#[cfg(debug_assertions)]
pub(crate) const MAIN_HOST: &str = "http://localhost:8000";
#[cfg(not(debug_assertions))]
pub(crate) const MAIN_HOST: &str = "https://adamperkowski.dev";

pub(crate) const GIT_URL: &str = "https://github.com/adamperkowski";

pub(crate) const ENV_VARS: &[&str] = &["JELLYFIN_SECRET", "MATRIX_URL", "MATRIX_TOKEN"];

pub(crate) const INT_REDIRECTS: &[(&str, &str)] = &[
    ("/favicon.ico", "/static/favicon.ico"),
    ("/styles.css", "/static/styles.css"),
];
pub(crate) const EXT_REDIRECTS: &[(&str, &str)] = &[
    ("/pronouns", "https://pronouns.cc/@adamperkowski"),
    (
        "/legal",
        "https://github.com/adamperkowski/website/blob/old/templates/pages/legal.tera",
    ),
    (
        "/api/nixdle",
        "https://github.com/adamperkowski/nixdle"
    )
];
