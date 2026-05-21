use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::GuildCache;
use mu_ui::{
    siege_screen, SiegeBattleSkill, SiegeCommanderEntry, SiegeMemberLocation, SiegeScreen,
    SiegeScreenState, UiRoute, UiShellState,
};

use crate::{control_http::ControlHttpState, SessionPhase};

const SCREEN_PADDING: f32 = 28.0;
const CARD_MAX_WIDTH: f32 = 960.0;
const CARD_PADDING: f32 = 28.0;
const CARD_GAP: f32 = 18.0;
const CONTENT_GAP: f32 = 14.0;
const ACCENT_BAR_HEIGHT: f32 = 4.0;
const TITLE_FONT_SIZE: f32 = 42.0;
const STATUS_FONT_SIZE: f32 = 16.0;
const BODY_FONT_SIZE: f32 = 21.0;
const BACKDROP_COLOR: Color = Color::srgb(0.03, 0.04, 0.06);
const CARD_COLOR: Color = Color::srgb(0.08, 0.10, 0.14);
const PANEL_COLOR: Color = Color::srgb(0.11, 0.13, 0.18);
const TEXT_COLOR: Color = Color::srgb(0.95, 0.96, 0.98);
const MUTED_TEXT_COLOR: Color = Color::srgb(0.72, 0.78, 0.84);
const INACTIVE_ACCENT: Color = Color::srgb(0.58, 0.62, 0.68);
const OBSERVER_ACCENT: Color = Color::srgb(0.34, 0.69, 0.84);
const SOLDIER_ACCENT: Color = Color::srgb(0.42, 0.82, 0.58);
const COMMANDER_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct SiegeShellPlugin;

#[derive(Debug, Clone, PartialEq)]
struct SiegeShellKey {
    phase: SessionPhase,
    screen: SiegeScreen,
}

#[derive(Debug, Default, Resource)]
struct SiegeShellState {
    root: Option<Entity>,
    key: Option<SiegeShellKey>,
}

#[derive(Debug, Clone)]
struct SiegeShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct SiegeShellRoot;

impl Plugin for SiegeShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SiegeShellState>()
            .add_systems(PostUpdate, sync_siege_shell_system);
    }
}

fn sync_siege_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    guild_cache: Res<GuildCache>,
    control_http: Option<Res<ControlHttpState>>,
    mut state: ResMut<SiegeShellState>,
) {
    let current = siege_shell_key(
        ui_shell.current(),
        session_state.phase(),
        siege_screen_state_for_control_http(control_http.as_deref()),
        &guild_cache,
    );

    let Some(key) = current else {
        clear_siege_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_siege_shell(&mut commands, &mut state);

    let Some(view) = siege_shell_view(key.phase, &key.screen) else {
        return;
    };

    let root = spawn_siege_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn siege_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    screen_state: SiegeScreenState,
    guild_cache: &GuildCache,
) -> Option<SiegeShellKey> {
    if !siege_shell_visible(route, phase) {
        return None;
    }

    Some(SiegeShellKey {
        phase,
        screen: siege_screen(screen_state, guild_cache),
    })
}

fn siege_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Siege && phase != SessionPhase::Disconnected
}

fn siege_screen_state_for_control_http(
    control_http: Option<&ControlHttpState>,
) -> SiegeScreenState {
    control_http
        .map(|control_http| control_http.snapshot().siege_screen_state)
        .flatten()
        .unwrap_or(SiegeScreenState::Inactive)
}

fn siege_shell_view(phase: SessionPhase, screen: &SiegeScreen) -> Option<SiegeShellView> {
    if !siege_shell_visible(screen.route, phase) {
        return None;
    }

    Some(SiegeShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: siege_shell_body(screen, phase),
        accent: accent_for_state(screen.state),
    })
}

fn siege_shell_body(screen: &SiegeScreen, phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Siege shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Siege",
        &format!(
            "state={} | battle_castle_active={} | created={} | guild_status={:?} | guild_mark_index={:?}",
            screen.state.as_str(),
            screen.battle_castle_active,
            screen.created,
            screen.guild_status,
            screen.guild_mark_index,
        ),
    );
    push_paragraph(
        &mut body,
        "Map",
        &format!(
            "time={:?} | mini_map_alpha={:.1} | skill_ui_visible={} | skill_tooltip_visible={} | command_controls_visible={} | mouse_in_minimap={}",
            screen.time,
            screen.mini_map_alpha,
            screen.skill_ui_visible,
            screen.skill_tooltip_visible,
            screen.command_controls_visible,
            screen.mouse_in_minimap,
        ),
    );
    push_paragraph(
        &mut body,
        "Selection",
        &format!(
            "selected_group={:?} | selected_command={:?} | current_battle_skill={:?}",
            screen.selected_group, screen.selected_command, screen.current_battle_skill,
        ),
    );

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Command Buffer",
        screen.command_buffer.iter().map(format_siege_command_entry),
        None,
    );
    push_lines(
        &mut body,
        "Member Locations",
        screen
            .guild_member_locations
            .iter()
            .map(format_siege_member_location),
        None,
    );
    push_lines(
        &mut body,
        "Battle Skills",
        screen.battle_skills.iter().map(format_siege_battle_skill),
        None,
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn format_siege_command_entry(entry: &SiegeCommanderEntry) -> String {
    format!(
        "team={} | command={:?} | x={} | y={} | life_time={}",
        entry.team, entry.command, entry.x, entry.y, entry.life_time
    )
}

fn format_siege_member_location(entry: &SiegeMemberLocation) -> String {
    format!(
        "marker_type={} | x={} | y={}",
        entry.marker_type, entry.x, entry.y
    )
}

fn format_siege_battle_skill(skill: &SiegeBattleSkill) -> String {
    format!("{skill:?}")
}

fn accent_for_state(state: SiegeScreenState) -> Color {
    match state {
        SiegeScreenState::Inactive => INACTIVE_ACCENT,
        SiegeScreenState::Observer => OBSERVER_ACCENT,
        SiegeScreenState::Soldier => SOLDIER_ACCENT,
        SiegeScreenState::Commander => COMMANDER_ACCENT,
    }
}

fn status_line(route: UiRoute, phase: SessionPhase) -> String {
    format!(
        "route={} | group={} | session={}",
        route.slug(),
        route.group().as_str(),
        phase.as_str()
    )
}

fn spawn_siege_shell(commands: &mut Commands, view: &SiegeShellView) -> Entity {
    let root = commands
        .spawn((
            SiegeShellRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(SCREEN_PADDING)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(BACKDROP_COLOR),
        ))
        .id();

    let card = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(CARD_MAX_WIDTH),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(CARD_PADDING)),
                row_gap: Val::Px(CARD_GAP),
                ..Default::default()
            },
            BackgroundColor(CARD_COLOR),
        ))
        .id();
    commands.entity(root).add_child(card);

    let accent = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(ACCENT_BAR_HEIGHT),
                ..Default::default()
            },
            BackgroundColor(view.accent),
        ))
        .id();
    commands.entity(card).add_child(accent);

    let title = spawn_text_block(commands, view.title, TITLE_FONT_SIZE, view.accent);
    commands.entity(card).add_child(title);

    let status = spawn_text_block(commands, &view.status, STATUS_FONT_SIZE, MUTED_TEXT_COLOR);
    commands.entity(card).add_child(status);

    let panel = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(CARD_PADDING * 0.75)),
                row_gap: Val::Px(CONTENT_GAP),
                ..Default::default()
            },
            BackgroundColor(PANEL_COLOR),
        ))
        .id();
    commands.entity(card).add_child(panel);

    let body = spawn_text_block(commands, &view.body, BODY_FONT_SIZE, TEXT_COLOR);
    commands.entity(panel).add_child(body);

    root
}

fn spawn_text_block(
    commands: &mut Commands,
    text: impl Into<String>,
    font_size: f32,
    color: Color,
) -> Entity {
    commands
        .spawn((
            Text::new(text),
            TextFont {
                font_size,
                ..Default::default()
            },
            TextColor(color),
        ))
        .id()
}

fn push_paragraph(body: &mut String, label: &str, value: &str) {
    use std::fmt::Write;

    let _ = writeln!(body, "{label}: {value}");
}

fn push_lines<I>(body: &mut String, label: &str, items: I, limit: Option<usize>)
where
    I: IntoIterator<Item = String>,
{
    use std::fmt::Write;

    let _ = writeln!(body, "{label}:");
    for (index, item) in items.into_iter().enumerate() {
        if matches!(limit, Some(limit) if index >= limit) {
            let _ = writeln!(body, "  ...");
            break;
        }

        let _ = writeln!(body, "  - {item}");
    }
}

fn clear_siege_shell(commands: &mut Commands, state: &mut SiegeShellState) {
    if let Some(root) = state.root.take() {
        commands.entity(root).despawn();
    }

    state.key = None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SessionPhase;

    #[test]
    fn siege_shell_visibility_requires_the_siege_route_and_active_session() {
        assert!(siege_shell_visible(UiRoute::Siege, SessionPhase::LoggedIn));
        assert!(!siege_shell_visible(UiRoute::Gate, SessionPhase::LoggedIn));
        assert!(!siege_shell_visible(
            UiRoute::Siege,
            SessionPhase::Disconnected
        ));
    }

    #[test]
    fn siege_shell_key_maps_the_control_http_state() {
        let cache = GuildCache::default();
        let key = siege_shell_key(
            UiRoute::Siege,
            SessionPhase::LoggedIn,
            SiegeScreenState::Commander,
            &cache,
        )
        .expect("siege key");

        assert_eq!(key.phase, SessionPhase::LoggedIn);
        assert_eq!(key.screen.state, SiegeScreenState::Commander);
    }

    #[test]
    fn siege_shell_state_defaults_to_inactive_without_control_http_override() {
        assert_eq!(
            siege_screen_state_for_control_http(None),
            SiegeScreenState::Inactive
        );
    }
}
