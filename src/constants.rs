pub const BIND_ADDR: &str = "127.0.0.1:8000";

#[cfg(debug_assertions)]
pub(crate) const HOST: &str = "http://localhost:8000";
#[cfg(not(debug_assertions))]
pub(crate) const HOST: &str = "https://koi.rip";

pub(crate) const DATABASE_URL: &str = "./website.db";
pub(crate) const GIT_URL: &str = "https://codeberg.org/koibtw";

pub(crate) const ENV_VARS: &[&str] = &["JELLYFIN_SECRET", "MATRIX_URL", "MATRIX_TOKEN"];

pub(crate) const INT_REDIRECTS: &[(&str, &str)] = &[
    ("/favicon.ico", "/img/favicon.ico"),
    ("/styles.css", "/static/styles.css"),
];
pub(crate) const EXT_REDIRECTS: &[(&str, &str)] = &[
    ("/api/nixdle", "https://github.com/nixdle/nixdle"),
];
