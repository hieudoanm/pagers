//! YAML schema for landify configs — port of `internal/landify/config.go`
//! and `sections.go` from the Go implementation.

use serde::{Deserialize, Serialize};

/// Root config loaded from `landify.yaml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "type", default)]
    pub page_type: String,
    #[serde(default)]
    pub site: Site,
    #[serde(default)]
    pub theme: Theme,
    #[serde(default)]
    pub hero: Hero,
    #[serde(default)]
    pub features: Features,
    #[serde(default)]
    pub demo: Demo,
    #[serde(default)]
    pub cta: CTA,
    #[serde(default)]
    pub waitlist: Waitlist,
    #[serde(default)]
    pub event: Event,
    #[serde(default)]
    pub download: Download,
    #[serde(default)]
    pub pricing: Pricing,
    #[serde(default)]
    pub app: App,
    #[serde(default)]
    pub portfolio: Portfolio,
    #[serde(default)]
    pub docs: Docs,
    #[serde(default)]
    pub faq: FAQ,
    #[serde(default)]
    pub team: Team,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub linktree: Linktree,
    #[serde(default)]
    pub footer: Footer,
}

impl Config {
    /// Open Graph metadata resolved against the site block.
    pub fn social(&self) -> OpenGraph {
        self.site.open_graph.resolved(&self.site)
    }
}

/// Theme palette: eight authored colors plus radius. Empty fields fall back
/// to `Theme::default()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub primary: String,
    #[serde(default)]
    pub secondary: String,
    #[serde(default)]
    pub neutral: String,
    #[serde(default)]
    pub info: String,
    #[serde(default)]
    pub warning: String,
    #[serde(default)]
    pub success: String,
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub radius: String,
}

impl Theme {
    /// Fill empty fields from `d` (the built-in defaults).
    pub fn merged(&self, d: &Theme) -> Theme {
        let pick = |a: &str, b: &str| {
            if a.is_empty() {
                b.to_string()
            } else {
                a.to_string()
            }
        };
        Theme {
            base: pick(&self.base, &d.base),
            primary: pick(&self.primary, &d.primary),
            secondary: pick(&self.secondary, &d.secondary),
            neutral: pick(&self.neutral, &d.neutral),
            info: pick(&self.info, &d.info),
            warning: pick(&self.warning, &d.warning),
            success: pick(&self.success, &d.success),
            error: pick(&self.error, &d.error),
            radius: pick(&self.radius, &d.radius),
        }
    }
}

impl Default for Theme {
    /// `DefaultTheme()` from the Go implementation.
    fn default() -> Self {
        Theme {
            base: "#f7f8fa".into(),
            primary: "#0d9488".into(),
            secondary: "#7c3aed".into(),
            neutral: "#555f6e".into(),
            info: "#2563eb".into(),
            warning: "#d97706".into(),
            success: "#16a34a".into(),
            error: "#dc2626".into(),
            radius: "10px".into(),
        }
    }
}

/// Brand identity and global navigation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Site {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub mark: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub nav: Vec<NavItem>,
    #[serde(rename = "og", default)]
    pub open_graph: OpenGraph,
}

/// Social-card metadata: `<meta>` tags plus the copy the generated card draws.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenGraph {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub image: String,
    #[serde(rename = "image_alt", default)]
    pub image_alt: String,
    #[serde(default)]
    pub url: String,
    #[serde(rename = "type", default)]
    pub og_type: String,
    #[serde(rename = "site_name", default)]
    pub site_name: String,
    #[serde(rename = "twitter_card", default)]
    pub twitter_card: String,
    #[serde(default)]
    pub kicker: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl OpenGraph {
    /// Whether the config asks for a card at all.
    pub fn configured(&self) -> bool {
        !(self.title.is_empty()
            && self.description.is_empty()
            && self.image.is_empty()
            && self.image_alt.is_empty()
            && self.url.is_empty()
            && self.og_type.is_empty()
            && self.site_name.is_empty()
            && self.twitter_card.is_empty()
            && self.kicker.is_empty()
            && self.tags.is_empty())
    }

    /// Copy with empty fields filled from the site block.
    pub fn resolved(&self, site: &Site) -> OpenGraph {
        let mut o = self.clone();
        if o.title.is_empty() {
            o.title = site.name.clone();
        }
        if o.description.is_empty() {
            o.description = site.description.clone();
        }
        if o.site_name.is_empty() {
            o.site_name = site.name.clone();
        }
        if o.og_type.is_empty() {
            o.og_type = "website".into();
        }
        if o.twitter_card.is_empty() {
            o.twitter_card = "summary_large_image".into();
        }
        o
    }
}

/// A single link in navbar/footer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavItem {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub href: String,
}

/// First section visitors see.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hero {
    #[serde(default)]
    pub badge: String,
    #[serde(default)]
    pub headline: String,
    #[serde(default)]
    pub subheadline: String,
    #[serde(default)]
    pub primary: Button,
    #[serde(default)]
    pub secondary: Button,
    #[serde(default)]
    pub image: HeroImage,
}

/// Labelled link used for call-to-action buttons.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Button {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub href: String,
}

/// Required 16:9 hero image.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroImage {
    #[serde(default)]
    pub src: String,
    #[serde(default)]
    pub alt: String,
}

/// Feature card grid.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Features {
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub items: Vec<Feature>,
}

/// Single feature card with an emoji icon.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Feature {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
}

/// Video walkthrough in a 16:9 frame.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Demo {
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub video: Video,
}

/// Demo video; `poster` is an optional frame, `track` a WebVTT file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Video {
    #[serde(default)]
    pub src: String,
    #[serde(default)]
    pub poster: String,
    #[serde(default)]
    pub track: String,
}

/// Closing call to action.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CTA {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub button: Button,
}

/// Email-capture panel for the "waitlist" page type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Waitlist {
    #[serde(default)]
    pub launches: String,
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub form: Form,
    #[serde(default)]
    pub social: Vec<NavItem>,
}

/// Waitlist email capture form.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Form {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub placeholder: String,
    #[serde(default)]
    pub button: String,
}

/// Conference-style event page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub venue: EventVenue,
    #[serde(default)]
    pub primary: Button,
    #[serde(rename = "speakers_heading", default)]
    pub speakers_heading: String,
    #[serde(rename = "speakers_sub", default)]
    pub speakers_sub: String,
    #[serde(default)]
    pub agenda: Vec<AgendaItem>,
    #[serde(default)]
    pub speakers: Vec<Speaker>,
}

/// Event location in the meta strip.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventVenue {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub address: String,
}

/// One timed agenda slot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgendaItem {
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub speaker: String,
}

/// Person card in the speaker grid.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Speaker {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub avatar: String,
}

/// Release/download page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Download {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub repo: String,
    #[serde(default)]
    pub install: String,
    #[serde(default)]
    pub platforms: Vec<Platform>,
}

/// One OS/arch download button.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Platform {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub href: String,
}

/// Copyright line and link set.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Footer {
    #[serde(default)]
    pub copyright: String,
    #[serde(default)]
    pub links: Vec<NavItem>,
}

// ---- Section types (config.go sibling `sections.go`) ----

/// Pricing tier grid.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pricing {
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub tiers: Vec<PricingTier>,
}

/// One card in a pricing grid.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PricingTier {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub period: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub cta: Button,
    #[serde(default)]
    pub perks: Vec<String>,
}

/// App-store style page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct App {
    #[serde(default)]
    pub stores: Vec<AppStore>,
    #[serde(default)]
    pub ratings: String,
    #[serde(default)]
    pub reviews: String,
    #[serde(default)]
    pub shots: Vec<Shot>,
}

/// Store download link.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppStore {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub href: String,
}

/// Portrait (9:16) app screenshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shot {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub src: String,
}

/// Personal portfolio page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Portfolio {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub about: String,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub projects: Vec<Project>,
}

/// Portfolio project card.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub href: String,
}

/// Documentation landing page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Docs {
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub sample: String,
    #[serde(default)]
    pub packages: Vec<DocCard>,
}

/// Linked topic card on a docs landing page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocCard {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub href: String,
}

/// Question-and-answer list (`<details>`/`<summary>`, no JS).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FAQ {
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub items: Vec<FAQItem>,
}

/// Single FAQ question + answer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FAQItem {
    #[serde(default)]
    pub question: String,
    #[serde(default)]
    pub answer: String,
}

/// Team page: values strip + member grid.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Team {
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub values: Vec<TeamValue>,
    #[serde(default)]
    pub members: Vec<TeamMember>,
}

/// One principle card in a team values strip.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TeamValue {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
}

/// One person card on a team page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TeamMember {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub avatar: String,
}

/// Service-status page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub updated: String,
    #[serde(default)]
    pub announcement: String,
    #[serde(default)]
    pub stats: Vec<StatusStat>,
    #[serde(default)]
    pub incidents: Vec<StatusTicket>,
}

/// One uptime/health figure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusStat {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
}

/// One recent incident row.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusTicket {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub body: String,
}

/// Compact link-in-bio page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Linktree {
    #[serde(default)]
    pub heading: String,
    #[serde(default)]
    pub sub: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub cards: Vec<LinkCard>,
    #[serde(default)]
    pub social: Vec<NavItem>,
}

/// One big rounded button on a linktree page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkCard {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub href: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub note: String,
}
