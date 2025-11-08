use serde::Serialize;

#[derive(Serialize)]
pub struct Badge {
    pub src: &'static str,
    pub alt: &'static str,
    pub href: Option<&'static str>,
}

pub const MIMI_BADGE: Badge = Badge {
    src: "/img/badges/mimi-the-car.gif",
    alt: "mimi the car (adam's site)",
    href: Some(crate::constants::HOST),
};

pub const FRIENDS: &[Badge] = &[Badge {
    src: "/img/badges/system72.gif",
    alt: "system72",
    href: Some("https://system72.dev"),
}];
