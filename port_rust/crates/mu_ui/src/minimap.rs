use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const MINIMAP_TITLE: &str = "Minimap";
const MINIMAP_LAYER_DEPTH: f32 = 8.1;
const MAP_SIZES: &[u16] = &[800, 1000, 1200, 1400, 1600, 1800];
const DEFAULT_MARKERS: &[MiniMapMarker] = &[
    MiniMapMarker::new(MiniMapMarkerKind::Npc, "Potion Merchant", (228, 48), 0),
    MiniMapMarker::new(MiniMapMarkerKind::Portal, "Lorencia Gate", (180, 96), 45),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiniMapMarkerKind {
    Npc,
    Portal,
}

impl MiniMapMarkerKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Npc => "npc",
            Self::Portal => "portal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiniMapMarker {
    pub kind: MiniMapMarkerKind,
    pub label: &'static str,
    pub location: (u16, u16),
    pub rotation: u16,
}

impl MiniMapMarker {
    pub const fn new(
        kind: MiniMapMarkerKind,
        label: &'static str,
        location: (u16, u16),
        rotation: u16,
    ) -> Self {
        Self {
            kind,
            label,
            location,
            rotation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MiniMapScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub title: &'static str,
    pub layer_depth: f32,
    pub loaded: bool,
    pub map_name: &'static str,
    pub scale_index: usize,
    pub hero_position: (u16, u16),
    pub map_sizes: &'static [u16],
    pub markers: &'static [MiniMapMarker],
    pub exit_button_enabled: bool,
}

pub fn minimap_screen() -> MiniMapScreen {
    MiniMapScreen {
        route: UiRoute::Minimap,
        layout: UiShellLayout::for_route(UiRoute::Minimap),
        widgets: UiShellWidgetSet::for_route(UiRoute::Minimap),
        title: MINIMAP_TITLE,
        layer_depth: MINIMAP_LAYER_DEPTH,
        loaded: true,
        map_name: "Lorencia",
        scale_index: 0,
        hero_position: (150, 128),
        map_sizes: MAP_SIZES,
        markers: DEFAULT_MARKERS,
        exit_button_enabled: true,
    }
}

impl MiniMapScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|title={}|layer_depth={}|loaded={}|map_name={}|scale_index={}|hero_position={:?}|map_sizes={:?}|markers={:?}|exit_button_enabled={}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.title,
            self.layer_depth,
            self.loaded,
            self.map_name,
            self.scale_index,
            self.hero_position,
            self.map_sizes,
            self.markers,
            self.exit_button_enabled,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::minimap_screen;

    #[test]
    fn minimap_snapshot_matches_legacy_loaded_panel() {
        assert_eq!(
            minimap_screen().snapshot(),
            "route=minimap|group=world|title=Minimap|layer_depth=8.1|loaded=true|map_name=Lorencia|scale_index=0|hero_position=(150, 128)|map_sizes=[800, 1000, 1200, 1400, 1600, 1800]|markers=[MiniMapMarker { kind: Npc, label: \"Potion Merchant\", location: (228, 48), rotation: 0 }, MiniMapMarker { kind: Portal, label: \"Lorencia Gate\", location: (180, 96), rotation: 45 }]|exit_button_enabled=true|layout=UiShellLayout { group: World, outer_margin: 16.0, panel_gap: 12.0, sidebar_width: 360.0, content_max_width: 1280.0, footer_height: 0.0 }|widgets=[Body, Sidebar, StatusStrip, Overlay]"
        );
    }
}
