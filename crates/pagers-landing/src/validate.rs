//! Schema validation: `Errors` reports every problem in a Config.
//! Port of `internal/landify/validate.go`.

use crate::color::parse_hex;
use crate::config::Config;

/// Supported page types, alphabetical.
pub const KNOWN_TYPES: &[&str] = &[
    "app",
    "docs",
    "download",
    "event",
    "faq",
    "linktree",
    "portfolio",
    "pricing",
    "product",
    "status",
    "team",
    "waitlist",
];

/// Canonicalize `typ`: empty → "product"; unknown values pass through so
/// validation can reject them.
pub fn normalize_type(typ: &str) -> &str {
    match typ {
        "" | "product" => "product",
        other => other,
    }
}

pub fn is_known_type(typ: &str) -> bool {
    KNOWN_TYPES.contains(&typ)
}

/// Every schema problem found in `cfg`; empty means valid.
pub fn errors(cfg: &Config) -> Vec<String> {
    let mut errs = Vec::new();
    let kind = normalize_type(&cfg.page_type);
    if !is_known_type(kind) {
        errs.push(format!(
            "type {kind:?} is not supported (available: {})",
            KNOWN_TYPES.join(", ")
        ));
        return errs;
    }

    let require = |value: &str, field: String, errs: &mut Vec<String>| {
        if value.trim().is_empty() {
            errs.push(format!("{field} is required"));
        }
    };
    macro_rules! req {
        ($v:expr, $f:expr) => {
            require($v, $f.to_string(), &mut errs)
        };
    }
    macro_rules! add {
        ($($a:tt)*) => {
            errs.push(format!($($a)*))
        };
    }

    req!(&cfg.site.name, "site.name");
    req!(&cfg.site.description, "site.description");
    if cfg.site.nav.is_empty() {
        add!("site.nav must contain at least one link");
    }
    for (i, item) in cfg.site.nav.iter().enumerate() {
        req!(&item.label, format!("site.nav[{i}].label"));
        req!(&item.href, format!("site.nav[{i}].href"));
    }
    req!(&cfg.footer.copyright, "footer.copyright");

    match kind {
        "waitlist" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            req!(&cfg.waitlist.heading, "waitlist.heading");
            req!(&cfg.waitlist.body, "waitlist.body");
            req!(&cfg.waitlist.form.action, "waitlist.form.action");
            req!(&cfg.waitlist.form.button, "waitlist.form.button");
            for (i, s) in cfg.waitlist.social.iter().enumerate() {
                req!(&s.label, format!("waitlist.social[{i}].label"));
                req!(&s.href, format!("waitlist.social[{i}].href"));
            }
        }
        "event" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            req!(&cfg.event.date, "event.date");
            req!(&cfg.event.venue.name, "event.venue.name");
            req!(&cfg.event.primary.label, "event.primary.label");
            req!(&cfg.event.primary.href, "event.primary.href");
            if cfg.event.agenda.is_empty() {
                add!("event.agenda must contain at least one item");
            }
            for (i, item) in cfg.event.agenda.iter().enumerate() {
                req!(&item.time, format!("event.agenda[{i}].time"));
                req!(&item.title, format!("event.agenda[{i}].title"));
            }
            if cfg.event.speakers.is_empty() {
                add!("event.speakers must contain at least one speaker");
            }
            for (i, sp) in cfg.event.speakers.iter().enumerate() {
                req!(&sp.name, format!("event.speakers[{i}].name"));
                req!(&sp.role, format!("event.speakers[{i}].role"));
            }
        }
        "download" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            req!(&cfg.download.version, "download.version");
            req!(&cfg.download.repo, "download.repo");
            if cfg.download.platforms.is_empty() {
                add!("download.platforms must contain at least one platform");
            }
            for (i, p) in cfg.download.platforms.iter().enumerate() {
                req!(&p.name, format!("download.platforms[{i}].name"));
                req!(&p.href, format!("download.platforms[{i}].href"));
            }
            if cfg.features.items.is_empty() {
                add!("features.items must contain at least one feature");
            }
            for (i, f) in cfg.features.items.iter().enumerate() {
                req!(&f.title, format!("features.items[{i}].title"));
                req!(&f.body, format!("features.items[{i}].body"));
            }
            req!(&cfg.cta.heading, "cta.heading");
            req!(&cfg.cta.body, "cta.body");
            req!(&cfg.cta.button.label, "cta.button.label");
            req!(&cfg.cta.button.href, "cta.button.href");
        }
        "app" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            if cfg.app.stores.is_empty() {
                add!("app.stores must contain at least one store");
            }
            for (i, s) in cfg.app.stores.iter().enumerate() {
                req!(&s.name, format!("app.stores[{i}].name"));
                req!(&s.href, format!("app.stores[{i}].href"));
            }
            for (i, sh) in cfg.app.shots.iter().enumerate() {
                req!(&sh.src, format!("app.shots[{i}].src"));
            }
        }
        "docs" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            if cfg.docs.packages.is_empty() {
                add!("docs.packages must contain at least one card");
            }
            for (i, c) in cfg.docs.packages.iter().enumerate() {
                req!(&c.title, format!("docs.packages[{i}].title"));
                req!(&c.href, format!("docs.packages[{i}].href"));
            }
        }
        "portfolio" => {
            req!(&cfg.portfolio.name, "portfolio.name");
            req!(&cfg.portfolio.about, "portfolio.about");
            if cfg.portfolio.projects.is_empty() {
                add!("portfolio.projects must contain at least one project");
            }
            for (i, p) in cfg.portfolio.projects.iter().enumerate() {
                req!(&p.title, format!("portfolio.projects[{i}].title"));
                req!(&p.body, format!("portfolio.projects[{i}].body"));
                req!(&p.href, format!("portfolio.projects[{i}].href"));
            }
        }
        "faq" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            if cfg.faq.items.is_empty() {
                add!("faq.items must contain at least one item");
            }
            for (i, item) in cfg.faq.items.iter().enumerate() {
                req!(&item.question, format!("faq.items[{i}].question"));
                req!(&item.answer, format!("faq.items[{i}].answer"));
            }
        }
        "team" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            if cfg.team.members.is_empty() {
                add!("team.members must contain at least one member");
            }
            for (i, m) in cfg.team.members.iter().enumerate() {
                req!(&m.name, format!("team.members[{i}].name"));
                req!(&m.role, format!("team.members[{i}].role"));
            }
            for (i, v) in cfg.team.values.iter().enumerate() {
                req!(&v.title, format!("team.values[{i}].title"));
                req!(&v.body, format!("team.values[{i}].body"));
            }
        }
        "status" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            req!(&cfg.status.state, "status.state");
            if !cfg.status.state.is_empty()
                && !["operational", "degraded", "outage", "maintenance"]
                    .contains(&cfg.status.state.as_str())
            {
                add!("status.state must be one of operational | degraded | outage | maintenance");
            }
            for (i, s) in cfg.status.stats.iter().enumerate() {
                req!(&s.label, format!("status.stats[{i}].label"));
                req!(&s.value, format!("status.stats[{i}].value"));
            }
            for (i, t) in cfg.status.incidents.iter().enumerate() {
                req!(&t.date, format!("status.incidents[{i}].date"));
                req!(&t.title, format!("status.incidents[{i}].title"));
                if !t.state.is_empty()
                    && !["investigating", "monitoring", "resolved"].contains(&t.state.as_str())
                {
                    add!("status.incidents[{i}].state must be one of investigating | monitoring | resolved");
                }
            }
        }
        "linktree" => {
            if cfg.linktree.cards.is_empty() {
                add!("linktree.cards must contain at least one card");
            }
            for (i, c) in cfg.linktree.cards.iter().enumerate() {
                req!(&c.title, format!("linktree.cards[{i}].title"));
                req!(&c.href, format!("linktree.cards[{i}].href"));
            }
            for (i, s) in cfg.linktree.social.iter().enumerate() {
                req!(&s.label, format!("linktree.social[{i}].label"));
                req!(&s.href, format!("linktree.social[{i}].href"));
            }
        }
        "pricing" => {
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            if cfg.pricing.tiers.is_empty() {
                add!("pricing.tiers must contain at least one tier");
            }
            for (i, t) in cfg.pricing.tiers.iter().enumerate() {
                req!(&t.name, format!("pricing.tiers[{i}].name"));
                req!(&t.price, format!("pricing.tiers[{i}].price"));
                req!(&t.cta.label, format!("pricing.tiers[{i}].cta.label"));
                req!(&t.cta.href, format!("pricing.tiers[{i}].cta.href"));
            }
        }
        _ => {
            // product — the original layout keeps its original requirements.
            req!(&cfg.hero.headline, "hero.headline");
            req!(&cfg.hero.subheadline, "hero.subheadline");
            req!(&cfg.hero.primary.label, "hero.primary.label");
            req!(&cfg.hero.primary.href, "hero.primary.href");
            req!(&cfg.hero.image.src, "hero.image.src");
            req!(&cfg.demo.video.src, "demo.video.src");
            if cfg.features.items.is_empty() {
                add!("features.items must contain at least one feature");
            }
            for (i, f) in cfg.features.items.iter().enumerate() {
                req!(&f.title, format!("features.items[{i}].title"));
                req!(&f.body, format!("features.items[{i}].body"));
            }
            req!(&cfg.cta.heading, "cta.heading");
            req!(&cfg.cta.body, "cta.body");
            req!(&cfg.cta.button.label, "cta.button.label");
            req!(&cfg.cta.button.href, "cta.button.href");
        }
    }

    for (name, value) in [
        ("base", &cfg.theme.base),
        ("primary", &cfg.theme.primary),
        ("secondary", &cfg.theme.secondary),
        ("neutral", &cfg.theme.neutral),
        ("info", &cfg.theme.info),
        ("warning", &cfg.theme.warning),
        ("success", &cfg.theme.success),
        ("error", &cfg.theme.error),
    ] {
        if parse_hex(value).is_err() {
            add!("theme.{name} ({value}) must be a #RRGGBB color");
        }
    }

    errs
}

/// Whether `cfg` satisfies the schema.
pub fn valid(cfg: &Config) -> bool {
    errors(cfg).is_empty()
}
