use crate::routes::{UiRoute, UiRouteGroup};

const STARTUP_WIDGETS: &[UiShellWidgetKind] = &[
    UiShellWidgetKind::Header,
    UiShellWidgetKind::Body,
    UiShellWidgetKind::Footer,
];
const AUTH_WIDGETS: &[UiShellWidgetKind] = &[
    UiShellWidgetKind::Header,
    UiShellWidgetKind::Body,
    UiShellWidgetKind::MessageRail,
    UiShellWidgetKind::Footer,
];
const CHARACTER_WIDGETS: &[UiShellWidgetKind] = &[
    UiShellWidgetKind::Header,
    UiShellWidgetKind::Body,
    UiShellWidgetKind::ActionBar,
    UiShellWidgetKind::Footer,
];
const WORLD_WIDGETS: &[UiShellWidgetKind] = &[
    UiShellWidgetKind::Body,
    UiShellWidgetKind::Sidebar,
    UiShellWidgetKind::StatusStrip,
    UiShellWidgetKind::Overlay,
];
const GAMEPLAY_WIDGETS: &[UiShellWidgetKind] = &[
    UiShellWidgetKind::Body,
    UiShellWidgetKind::Sidebar,
    UiShellWidgetKind::ActionBar,
    UiShellWidgetKind::Overlay,
];
const ADMIN_WIDGETS: &[UiShellWidgetKind] = &[
    UiShellWidgetKind::Header,
    UiShellWidgetKind::Body,
    UiShellWidgetKind::Sidebar,
    UiShellWidgetKind::Footer,
];
const ERROR_WIDGETS: &[UiShellWidgetKind] = &[
    UiShellWidgetKind::Header,
    UiShellWidgetKind::Body,
    UiShellWidgetKind::Footer,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiShellWidgetKind {
    Header,
    Body,
    Sidebar,
    Footer,
    Overlay,
    MessageRail,
    ActionBar,
    StatusStrip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiShellWidgetSet {
    pub group: UiRouteGroup,
    widgets: &'static [UiShellWidgetKind],
}

impl UiShellWidgetSet {
    pub fn for_route(route: UiRoute) -> Self {
        Self::for_group(route.group())
    }

    pub const fn for_group(group: UiRouteGroup) -> Self {
        match group {
            UiRouteGroup::Startup => Self::new(group, STARTUP_WIDGETS),
            UiRouteGroup::Authentication => Self::new(group, AUTH_WIDGETS),
            UiRouteGroup::Character => Self::new(group, CHARACTER_WIDGETS),
            UiRouteGroup::World => Self::new(group, WORLD_WIDGETS),
            UiRouteGroup::Gameplay => Self::new(group, GAMEPLAY_WIDGETS),
            UiRouteGroup::Admin => Self::new(group, ADMIN_WIDGETS),
            UiRouteGroup::Error => Self::new(group, ERROR_WIDGETS),
        }
    }

    pub const fn new(group: UiRouteGroup, widgets: &'static [UiShellWidgetKind]) -> Self {
        Self { group, widgets }
    }

    pub fn widgets(self) -> &'static [UiShellWidgetKind] {
        self.widgets
    }

    pub fn has(self, kind: UiShellWidgetKind) -> bool {
        self.widgets.contains(&kind)
    }
}

#[cfg(test)]
mod tests {
    use super::{UiShellWidgetKind, UiShellWidgetSet};
    use crate::routes::{UiRoute, UiRouteGroup};

    #[test]
    fn world_widget_set_includes_sidebar_and_overlay() {
        let widgets = UiShellWidgetSet::for_route(UiRoute::World);

        assert_eq!(widgets.group, UiRouteGroup::World);
        assert!(widgets.has(UiShellWidgetKind::Sidebar));
        assert!(widgets.has(UiShellWidgetKind::Overlay));
        assert!(widgets.has(UiShellWidgetKind::StatusStrip));
    }

    #[test]
    fn login_widget_set_keeps_navigation_widgets_simple() {
        let widgets = UiShellWidgetSet::for_route(UiRoute::Login);

        assert_eq!(widgets.group, UiRouteGroup::Authentication);
        assert!(widgets.has(UiShellWidgetKind::Header));
        assert!(widgets.has(UiShellWidgetKind::MessageRail));
        assert!(!widgets.has(UiShellWidgetKind::Sidebar));
    }
}
