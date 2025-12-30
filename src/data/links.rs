use crate::constants;
use serde::Serialize;

#[derive(Serialize)]
pub struct Link {
    pub rel: &'static str,
    pub href: &'static str,
    pub name: Option<&'static str>,
    pub title: Option<&'static str>,
    pub ltype: Option<&'static str>,
    pub sizes: Option<&'static str>,
}

pub const LINKS: &[Link] = &[
    Link {
        rel: "stylesheet",
        href: "/static/styles.css",
        name: None,
        title: None,
        ltype: None,
        sizes: None,
    },
    Link {
        rel: "icon",
        href: "/img/favicon.ico",
        name: None,
        title: None,
        ltype: Some("image/x-icon"),
        sizes: Some("32x32 64x64 96x96 128x128 256x256"),
    },
    Link {
        rel: "me",
        href: "https://github.com/adamperkowski",
        name: Some("@adamperkowski"),
        title: Some("github"),
        ltype: None,
        sizes: None,
    },
    Link {
        rel: "me",
        href: constants::GIT_URL,
        name: Some("@adamperkowski"),
        title: Some("codeberg"),
        ltype: None,
        sizes: None,
    },
    Link {
        rel: "me",
        href: "mailto:me@adamperkowski.dev",
        name: Some("me@adamperkowski.dev"),
        title: Some("email"),
        ltype: None,
        sizes: None,
    },
    Link {
        rel: "me",
        href: "https://matrix.to/#/@adam:system72.dev",
        name: Some("@adam:system72.dev"),
        title: Some("matrix"),
        ltype: None,
        sizes: None,
    },
    Link {
        rel: "me",
        href: "https://discord.com/users/1400922134355644458",
        name: Some("@catpotatoburger"),
        title: Some("discord"),
        ltype: None,
        sizes: None,
    },
    Link {
        rel: "me atproto",
        href: "https://bsky.app/profile/did:plc:b26ewgkrnx3yvsp2cdao3ntu",
        name: Some("@adamperkowski.dev"),
        title: Some("bluesky"),
        ltype: None,
        sizes: None,
    },
    Link {
        rel: "me",
        href: "https://last.fm/user/adamperkowski",
        name: Some("adamperkowski"),
        title: Some("last.fm"),
        ltype: None,
        sizes: None,
    },
];
