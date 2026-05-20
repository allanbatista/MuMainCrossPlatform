use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{DuelChannelInfo, DuelManager, DuelPlayerInfo};
use mu_ui::{duel_screen, DuelScreen, DuelScreenState, UiRoute, UiShellState};

use crate::SessionPhase;

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
const CHALLENGE_ACCENT: Color = Color::srgb(0.34, 0.69, 0.84);
const CHANNEL_ACCENT: Color = Color::srgb(0.42, 0.82, 0.58);
const WATCHING_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct DuelShellPlugin;

#[derive(Debug, Clone, PartialEq)]
struct DuelShellKey {
    route: UiRoute,
    phase: SessionPhase,
    manager: DuelManager,
}

#[derive(Debug, Default, Resource)]
struct DuelShellState {
    root: Option<Entity>,
    key: Option<DuelShellKey>,
}

#[derive(Debug, Clone)]
struct DuelShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct DuelShellRoot;

impl Plugin for DuelShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DuelShellState>()
            .add_systems(PostUpdate, sync_duel_shell_system);
    }
}

fn sync_duel_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    duel_manager: Res<DuelManager>,
    mut state: ResMut<DuelShellState>,
) {
    let current = duel_shell_key(ui_shell.current(), session_state.phase(), &duel_manager);

    let Some(key) = current else {
        clear_duel_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_duel_shell(&mut commands, &mut state);

    let Some(view) = duel_shell_view(key.route, key.phase, &key.manager) else {
        return;
    };

    let root = spawn_duel_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn duel_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    manager: &DuelManager,
) -> Option<DuelShellKey> {
    if !duel_shell_visible(route, phase) {
        return None;
    }

    Some(DuelShellKey {
        route,
        phase,
        manager: manager.clone(),
    })
}

fn duel_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Duel && phase != SessionPhase::Disconnected
}

fn duel_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    manager: &DuelManager,
) -> Option<DuelShellView> {
    if !duel_shell_visible(route, phase) {
        return None;
    }

    let state = duel_screen_state_for_phase_and_manager(phase, manager);
    let screen = duel_screen(state);

    Some(DuelShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: duel_shell_body(&screen, phase, manager),
        accent: accent_for_state(screen.state),
    })
}

fn duel_screen_state_for_phase_and_manager(
    phase: SessionPhase,
    manager: &DuelManager,
) -> DuelScreenState {
    if phase == SessionPhase::ReadyForLogin {
        return DuelScreenState::Error;
    }

    if !manager.duel_watch_users().is_empty() {
        DuelScreenState::Watching
    } else if manager.current_channel() >= 0 {
        DuelScreenState::ChannelList
    } else {
        DuelScreenState::Challenge
    }
}

fn duel_shell_body(screen: &DuelScreen, phase: SessionPhase, manager: &DuelManager) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Duel shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Duel",
        &format!(
            "state={} | duel_enabled={} | pet_duel_enabled={} | fighter_regenerated={} | current_channel={}",
            screen.state.as_str(),
            screen.duel_enabled,
            screen.pet_duel_enabled,
            screen.fighter_regenerated,
            screen.current_channel,
        ),
    );
    push_paragraph(
        &mut body,
        "Contest",
        &format!(
            "hero_id={:?} | enemy_id={:?} | hero_score={:?} | enemy_score={:?} | hero_hp_rate={:?} | enemy_hp_rate={:?} | hero_sd_rate={:?} | enemy_sd_rate={:?}",
            screen.hero_id,
            screen.enemy_id,
            screen.hero_score,
            screen.enemy_score,
            screen.hero_hp_rate,
            screen.enemy_hp_rate,
            screen.hero_sd_rate,
            screen.enemy_sd_rate,
        ),
    );

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Players",
        manager
            .players()
            .iter()
            .enumerate()
            .map(format_player_entry),
        Some("No duel players."),
    );
    push_lines(
        &mut body,
        "Channels",
        manager
            .channels()
            .iter()
            .enumerate()
            .map(format_channel_entry),
        Some("No duel channels."),
    );
    push_lines(
        &mut body,
        "Watch users",
        manager.duel_watch_users().iter().cloned(),
        Some("No duel watchers."),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn format_player_entry((index, player): (usize, &DuelPlayerInfo)) -> String {
    format!(
        "slot={} | index={} | id={} | score={} | hp_rate={} | sd_rate={}",
        index, player.index, player.id, player.score, player.hp_rate, player.sd_rate,
    )
}

fn format_channel_entry((index, channel): (usize, &DuelChannelInfo)) -> String {
    format!(
        "slot={} | enabled={} | joinable={} | player_one_id={} | player_two_id={}",
        index, channel.enabled, channel.joinable, channel.player_one_id, channel.player_two_id,
    )
}

fn accent_for_state(state: DuelScreenState) -> Color {
    match state {
        DuelScreenState::Challenge => CHALLENGE_ACCENT,
        DuelScreenState::ChannelList => CHANNEL_ACCENT,
        DuelScreenState::Watching => WATCHING_ACCENT,
        DuelScreenState::Error => ERROR_ACCENT,
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

fn spawn_duel_shell(commands: &mut Commands, view: &DuelShellView) -> Entity {
    let root = commands
        .spawn((
            DuelShellRoot,
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

fn clear_duel_shell(commands: &mut Commands, state: &mut DuelShellState) {
    if let Some(root) = state.root.take() {
        commands.entity(root).despawn();
    }
    state.key = None;
}

fn push_paragraph(output: &mut String, heading: &str, text: &str) {
    if !output.is_empty() {
        output.push('\n');
    }

    output.push_str(heading);
    output.push('\n');
    output.push_str(text);
}

fn push_lines<I, T>(output: &mut String, heading: &str, values: I, empty_label: Option<&str>)
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    let mut values = values.into_iter().map(Into::into).peekable();
    if !output.is_empty() {
        output.push('\n');
    }

    output.push_str(heading);
    output.push('\n');

    if values.peek().is_none() {
        output.push_str(empty_label.unwrap_or("<none>"));
        return;
    }

    for value in values {
        output.push_str("- ");
        output.push_str(&value);
        output.push('\n');
    }
    output.pop();
}

#[cfg(test)]
mod tests {
    use super::{duel_screen_state_for_phase_and_manager, duel_shell_view, duel_shell_visible};
    use crate::SessionPhase;
    use mu_gameplay::DuelManager;
    use mu_ui::{DuelScreenState, UiRoute};

    #[test]
    fn duel_shell_visibility_follows_route_and_disconnect() {
        assert!(duel_shell_visible(UiRoute::Duel, SessionPhase::LoggedIn));
        assert!(!duel_shell_visible(
            UiRoute::Duel,
            SessionPhase::Disconnected
        ));
        assert!(!duel_shell_visible(UiRoute::Friend, SessionPhase::LoggedIn));
    }

    #[test]
    fn duel_shell_state_follows_manager_state() {
        let manager = DuelManager::new();
        assert_eq!(
            duel_screen_state_for_phase_and_manager(SessionPhase::LoggedIn, &manager),
            DuelScreenState::Challenge
        );
        let view = duel_shell_view(UiRoute::Duel, SessionPhase::LoggedIn, &manager)
            .expect("duel shell view");
        assert!(view.body.contains("state=challenge"));

        let mut manager = DuelManager::new();
        manager.set_current_channel(2);
        let view = duel_shell_view(UiRoute::Duel, SessionPhase::LoggedIn, &manager)
            .expect("duel shell view");
        assert_eq!(
            duel_screen_state_for_phase_and_manager(SessionPhase::LoggedIn, &manager),
            DuelScreenState::ChannelList
        );
        assert!(view.body.contains("state=channel-list"));

        let mut manager = DuelManager::new();
        manager.add_duel_watch_user(Some("Watcher"));
        let view = duel_shell_view(UiRoute::Duel, SessionPhase::LoggedIn, &manager)
            .expect("duel shell view");
        assert_eq!(
            duel_screen_state_for_phase_and_manager(SessionPhase::LoggedIn, &manager),
            DuelScreenState::Watching
        );
        assert!(view.body.contains("state=watching"));

        let manager = DuelManager::new();
        let view = duel_shell_view(UiRoute::Duel, SessionPhase::ReadyForLogin, &manager)
            .expect("duel shell view");
        assert_eq!(
            duel_screen_state_for_phase_and_manager(SessionPhase::ReadyForLogin, &manager),
            DuelScreenState::Error
        );
        assert!(view.body.contains("state=error"));
    }
}
