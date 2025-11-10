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

pub const FRIENDS: &[Badge] = &[
    Badge {
        src: "/img/badges/system72.gif",
        alt: "system72",
        href: Some("https://system72.dev"),
    },
    Badge {
        src: "/img/badges/sebaguardian.gif",
        alt: "sebaguardian",
        href: Some("https://sebaguardian.github.io"),
    },
    Badge {
        src: "/img/badges/ari.png",
        alt: "ari-web",
        href: Some("https://ari.lt"),
    },
    Badge {
        src: "/img/badges/tnixc.webp",
        alt: "tnixc (enoch's site)",
        href: Some("https://enochlau.com"),
    },
    Badge {
        src: "/img/badges/kolpix.gif",
        alt: "kolpix",
        href: Some("https://kolpix.nekoweb.org"),
    },
];

pub const COOL_SITES: &[Badge] = &[
    Badge {
        src: "/img/badges/espy.gif",
        alt: "espy.world",
        href: Some("https://espy.world"),
    },
    Badge {
        src: "/img/badges/lilithdev.gif",
        alt: "lilith",
        href: Some("https://lilithdev.neocities.org"),
    },
    Badge {
        src: "/img/badges/oyaswmi.gif",
        alt: "oyaswmi",
        href: Some("https://oyaswmi.net"),
    },
    Badge {
        src: "/img/badges/appledust.gif",
        alt: "appledust (lejla's site)",
        href: Some("https://lejlart.com/apple.html"),
    },
    Badge {
        src: "/img/badges/fulvern.gif",
        alt: "fulvern",
        href: Some("https://fulvern.neocities.org"),
    },
];
