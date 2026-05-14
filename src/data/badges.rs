use serde::Serialize;

#[derive(Serialize)]
pub struct Badge {
  pub src: &'static str,
  pub alt: &'static str,
  pub href: Option<&'static str>,
}

pub const MIMI_BADGE: Badge = Badge {
  src: "mimi-the-car.gif",
  alt: "mimi the car (koi's site)",
  href: Some(crate::constants::HOST),
};

pub const FRIENDS: &[Badge] = &[
  Badge {
    src: "robin.gif",
    alt: "robin",
    href: Some("https://robinwobin.dev"),
  },
  Badge {
    src: "sebaguardian.gif",
    alt: "sebaguardian's site",
    href: Some("https://seba.ebil.club"),
  },
  Badge {
    src: "nelliel.gif",
    alt: "nelliel",
    href: Some("https://nelliel.cv"),
  },
  Badge {
    src: "evelyn.webp",
    alt: "garfunkles.space",
    href: Some("https://evelyn.willalways.top"),
  },
  Badge {
    src: "ari.png",
    alt: "ari-web",
    href: Some("https://ari.lt"),
  },
  Badge {
    src: "dvdznf.gif",
    alt: "dvdznf",
    href: Some("https://dvdznf.xyz"),
  },
  Badge {
    src: "nunikii.gif",
    alt: "nunikii",
    href: Some("https://nunikii.ebil.club"),
  },
  Badge {
    src: "thegail.gif",
    alt: "the gail",
    href: Some("https://thegail.site"),
  },
  Badge {
    src: "tired.moe.gif",
    alt: "tired.moe",
    href: Some("https://tired.moe"),
  },
];

pub const COOL_SITES: &[Badge] = &[
  Badge {
    src: "espy.gif",
    alt: "espy.world",
    href: Some("https://espy.world"),
  },
  Badge {
    src: "appledust.gif",
    alt: "appledust (lejla's site)",
    href: Some("https://lejlart.com/apple.html"),
  },
  Badge {
    src: "lilithdev.gif",
    alt: "lilith",
    href: Some("https://lilithdev.neocities.org"),
  },
  Badge {
    src: "dinpixels.gif",
    alt: "dinpixels",
    href: Some("https://dinpixels.neocities.org"),
  },
  Badge {
    src: "oyaswmi.gif",
    alt: "oyaswmi",
    href: Some("https://oyaswmi.net"),
  },
  Badge {
    src: "fulvern.gif",
    alt: "fulvern",
    href: Some("https://fulvern.neocities.org"),
  },
];
