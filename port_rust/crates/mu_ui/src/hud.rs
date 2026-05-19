use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const HUD_TITLE: &str = "HUD";
const HUD_LAYER_DEPTH: f32 = 10.2;
const HUD_KEY_EVENT_ORDER: f32 = 7.0;

const READY_GAUGES: &[HudGauge] = &[
    HudGauge::new(HudGaugeKind::Life, 920, 1240),
    HudGauge::new(HudGaugeKind::Mana, 610, 780),
    HudGauge::new(HudGaugeKind::Shield, 430, 960),
    HudGauge::new(HudGaugeKind::Ag, 184, 260),
    HudGauge::new(HudGaugeKind::Experience, 2_481_200, 3_120_000),
];

const READY_BUTTONS: &[HudButton] = &[
    HudButton::new(HudButtonKind::CharacterInfo, false),
    HudButton::new(HudButtonKind::Inventory, false),
    HudButton::new(HudButtonKind::Friend, false),
    HudButton::new(HudButtonKind::Window, false),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudGaugeKind {
    Life,
    Mana,
    Shield,
    Ag,
    Experience,
}

impl HudGaugeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Life => "life",
            Self::Mana => "mana",
            Self::Shield => "shield",
            Self::Ag => "ag",
            Self::Experience => "experience",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HudGauge {
    pub kind: HudGaugeKind,
    pub current: u64,
    pub maximum: u64,
}

impl HudGauge {
    pub const fn new(kind: HudGaugeKind, current: u64, maximum: u64) -> Self {
        Self {
            kind,
            current,
            maximum,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudButtonKind {
    CharacterInfo,
    Inventory,
    Friend,
    Window,
}

impl HudButtonKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CharacterInfo => "character-info",
            Self::Inventory => "inventory",
            Self::Friend => "friend",
            Self::Window => "window",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HudButton {
    pub kind: HudButtonKind,
    pub pressed: bool,
}

impl HudButton {
    pub const fn new(kind: HudButtonKind, pressed: bool) -> Self {
        Self { kind, pressed }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HudScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub title: &'static str,
    pub layer_depth: f32,
    pub key_event_order: f32,
    pub master_level: bool,
    pub exp_effect_active: bool,
    pub gauges: &'static [HudGauge],
    pub buttons: &'static [HudButton],
}

pub fn hud_screen() -> HudScreen {
    HudScreen {
        route: UiRoute::Hud,
        layout: UiShellLayout::for_route(UiRoute::Hud),
        widgets: UiShellWidgetSet::for_route(UiRoute::Hud),
        title: HUD_TITLE,
        layer_depth: HUD_LAYER_DEPTH,
        key_event_order: HUD_KEY_EVENT_ORDER,
        master_level: false,
        exp_effect_active: false,
        gauges: READY_GAUGES,
        buttons: READY_BUTTONS,
    }
}

impl HudScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|title={}|layer_depth={}|key_event_order={}|master_level={}|exp_effect_active={}|gauges={:?}|buttons={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.title,
            self.layer_depth,
            self.key_event_order,
            self.master_level,
            self.exp_effect_active,
            self.gauges,
            self.buttons,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::hud_screen;

    #[test]
    fn hud_snapshot_matches_legacy_world_frame() {
        assert_eq!(
            hud_screen().snapshot(),
            "route=hud|group=world|title=HUD|layer_depth=10.2|key_event_order=7|master_level=false|exp_effect_active=false|gauges=[HudGauge { kind: Life, current: 920, maximum: 1240 }, HudGauge { kind: Mana, current: 610, maximum: 780 }, HudGauge { kind: Shield, current: 430, maximum: 960 }, HudGauge { kind: Ag, current: 184, maximum: 260 }, HudGauge { kind: Experience, current: 2481200, maximum: 3120000 }]|buttons=[HudButton { kind: CharacterInfo, pressed: false }, HudButton { kind: Inventory, pressed: false }, HudButton { kind: Friend, pressed: false }, HudButton { kind: Window, pressed: false }]|layout=UiShellLayout { group: World, outer_margin: 16.0, panel_gap: 12.0, sidebar_width: 360.0, content_max_width: 1280.0, footer_height: 0.0 }|widgets=[Body, Sidebar, StatusStrip, Overlay]"
        );
    }
}
