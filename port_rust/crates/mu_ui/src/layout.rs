use crate::routes::{UiRoute, UiRouteGroup};

const STARTUP_OUTER_MARGIN: f32 = 48.0;
const STARTUP_PANEL_GAP: f32 = 24.0;
const STARTUP_SIDEBAR_WIDTH: f32 = 0.0;
const STARTUP_CONTENT_MAX_WIDTH: f32 = 960.0;
const STARTUP_FOOTER_HEIGHT: f32 = 32.0;

const AUTH_OUTER_MARGIN: f32 = 40.0;
const AUTH_PANEL_GAP: f32 = 20.0;
const AUTH_SIDEBAR_WIDTH: f32 = 0.0;
const AUTH_CONTENT_MAX_WIDTH: f32 = 1024.0;
const AUTH_FOOTER_HEIGHT: f32 = 36.0;

const CHARACTER_OUTER_MARGIN: f32 = 32.0;
const CHARACTER_PANEL_GAP: f32 = 18.0;
const CHARACTER_SIDEBAR_WIDTH: f32 = 0.0;
const CHARACTER_CONTENT_MAX_WIDTH: f32 = 1100.0;
const CHARACTER_FOOTER_HEIGHT: f32 = 36.0;

const WORLD_OUTER_MARGIN: f32 = 16.0;
const WORLD_PANEL_GAP: f32 = 12.0;
const WORLD_SIDEBAR_WIDTH: f32 = 360.0;
const WORLD_CONTENT_MAX_WIDTH: f32 = 1280.0;
const WORLD_FOOTER_HEIGHT: f32 = 0.0;

const GAMEPLAY_OUTER_MARGIN: f32 = 20.0;
const GAMEPLAY_PANEL_GAP: f32 = 16.0;
const GAMEPLAY_SIDEBAR_WIDTH: f32 = 320.0;
const GAMEPLAY_CONTENT_MAX_WIDTH: f32 = 1200.0;
const GAMEPLAY_FOOTER_HEIGHT: f32 = 36.0;

const ADMIN_OUTER_MARGIN: f32 = 24.0;
const ADMIN_PANEL_GAP: f32 = 18.0;
const ADMIN_SIDEBAR_WIDTH: f32 = 420.0;
const ADMIN_CONTENT_MAX_WIDTH: f32 = 1280.0;
const ADMIN_FOOTER_HEIGHT: f32 = 32.0;

const ERROR_OUTER_MARGIN: f32 = 48.0;
const ERROR_PANEL_GAP: f32 = 24.0;
const ERROR_SIDEBAR_WIDTH: f32 = 0.0;
const ERROR_CONTENT_MAX_WIDTH: f32 = 960.0;
const ERROR_FOOTER_HEIGHT: f32 = 32.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiShellLayout {
    pub group: UiRouteGroup,
    pub outer_margin: f32,
    pub panel_gap: f32,
    pub sidebar_width: f32,
    pub content_max_width: f32,
    pub footer_height: f32,
}

impl UiShellLayout {
    pub fn for_route(route: UiRoute) -> Self {
        Self::for_group(route.group())
    }

    pub const fn for_group(group: UiRouteGroup) -> Self {
        match group {
            UiRouteGroup::Startup => Self::new(
                group,
                STARTUP_OUTER_MARGIN,
                STARTUP_PANEL_GAP,
                STARTUP_SIDEBAR_WIDTH,
                STARTUP_CONTENT_MAX_WIDTH,
                STARTUP_FOOTER_HEIGHT,
            ),
            UiRouteGroup::Authentication => Self::new(
                group,
                AUTH_OUTER_MARGIN,
                AUTH_PANEL_GAP,
                AUTH_SIDEBAR_WIDTH,
                AUTH_CONTENT_MAX_WIDTH,
                AUTH_FOOTER_HEIGHT,
            ),
            UiRouteGroup::Character => Self::new(
                group,
                CHARACTER_OUTER_MARGIN,
                CHARACTER_PANEL_GAP,
                CHARACTER_SIDEBAR_WIDTH,
                CHARACTER_CONTENT_MAX_WIDTH,
                CHARACTER_FOOTER_HEIGHT,
            ),
            UiRouteGroup::World => Self::new(
                group,
                WORLD_OUTER_MARGIN,
                WORLD_PANEL_GAP,
                WORLD_SIDEBAR_WIDTH,
                WORLD_CONTENT_MAX_WIDTH,
                WORLD_FOOTER_HEIGHT,
            ),
            UiRouteGroup::Gameplay => Self::new(
                group,
                GAMEPLAY_OUTER_MARGIN,
                GAMEPLAY_PANEL_GAP,
                GAMEPLAY_SIDEBAR_WIDTH,
                GAMEPLAY_CONTENT_MAX_WIDTH,
                GAMEPLAY_FOOTER_HEIGHT,
            ),
            UiRouteGroup::Admin => Self::new(
                group,
                ADMIN_OUTER_MARGIN,
                ADMIN_PANEL_GAP,
                ADMIN_SIDEBAR_WIDTH,
                ADMIN_CONTENT_MAX_WIDTH,
                ADMIN_FOOTER_HEIGHT,
            ),
            UiRouteGroup::Error => Self::new(
                group,
                ERROR_OUTER_MARGIN,
                ERROR_PANEL_GAP,
                ERROR_SIDEBAR_WIDTH,
                ERROR_CONTENT_MAX_WIDTH,
                ERROR_FOOTER_HEIGHT,
            ),
        }
    }

    pub const fn new(
        group: UiRouteGroup,
        outer_margin: f32,
        panel_gap: f32,
        sidebar_width: f32,
        content_max_width: f32,
        footer_height: f32,
    ) -> Self {
        Self {
            group,
            outer_margin,
            panel_gap,
            sidebar_width,
            content_max_width,
            footer_height,
        }
    }

    pub fn has_sidebar(self) -> bool {
        self.sidebar_width > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::UiShellLayout;
    use crate::routes::{UiRoute, UiRouteGroup};

    #[test]
    fn world_layout_uses_sidebar_and_no_footer() {
        let layout = UiShellLayout::for_route(UiRoute::World);

        assert_eq!(layout.group, UiRouteGroup::World);
        assert!(layout.has_sidebar());
        assert_eq!(layout.footer_height, 0.0);
    }

    #[test]
    fn login_layout_keeps_sidebar_hidden() {
        let layout = UiShellLayout::for_route(UiRoute::Login);

        assert_eq!(layout.group, UiRouteGroup::Authentication);
        assert!(!layout.has_sidebar());
        assert!(layout.content_max_width >= 1024.0);
    }
}
