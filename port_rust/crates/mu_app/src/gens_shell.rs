use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{GensManager, GensMode, GensType};
use mu_protocol::events::GensRankingInfo;
use mu_ui::{
    gens_ranking_screen, GensRankingScreen, GensRankingScreenState, UiRoute, UiShellState,
};

use crate::bootstrap_runtime::BootstrapRuntime;
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
const READY_ACCENT: Color = Color::srgb(0.34, 0.69, 0.84);
const JOIN_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);
const REWARD_ACCENT: Color = Color::srgb(0.42, 0.82, 0.58);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GensShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GensShellKey {
    route: UiRoute,
    phase: SessionPhase,
    manager: GensManager,
}

#[derive(Debug, Default, Resource)]
struct GensShellState {
    root: Option<Entity>,
    key: Option<GensShellKey>,
    last_session_phase: Option<SessionPhase>,
    last_applied_snapshot: Option<GensRankingInfo>,
    gens_ranking_requested: bool,
}

#[derive(Debug, Clone)]
struct GensShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct GensShellRoot;

impl Plugin for GensShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GensShellState>()
            .add_systems(PostUpdate, sync_gens_shell_system);
    }
}

fn sync_gens_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    bootstrap: Option<Res<BootstrapRuntime>>,
    mut gens_manager: ResMut<GensManager>,
    mut state: ResMut<GensShellState>,
) {
    let phase = session_state.phase();
    if state.last_session_phase != Some(phase) {
        gens_manager.reset();
        state.last_applied_snapshot = None;
        state.last_session_phase = Some(phase);
    }

    if phase == SessionPhase::LoggedIn {
        if let Some(bootstrap) = bootstrap.as_deref() {
            if let Some(snapshot) = bootstrap.gens_ranking_snapshot() {
                if state.last_applied_snapshot.as_ref() != Some(&snapshot) {
                    apply_gens_ranking_snapshot(snapshot, &mut gens_manager);
                    state.last_applied_snapshot = Some(snapshot);
                }
            } else if state.last_applied_snapshot.is_some() {
                gens_manager.reset();
                state.last_applied_snapshot = None;
            }
        }
    }

    let current = gens_shell_key(ui_shell.current(), phase, gens_manager.clone());

    let Some(key) = current else {
        clear_gens_shell(&mut commands, &mut state);
        state.gens_ranking_requested = false;
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        maybe_queue_gens_ranking_request(
            bootstrap.as_deref(),
            phase,
            &mut state.gens_ranking_requested,
        );
        return;
    }

    clear_gens_shell(&mut commands, &mut state);

    let Some(view) = gens_shell_view(key.route, key.phase, &key.manager) else {
        return;
    };

    let root = spawn_gens_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
    maybe_queue_gens_ranking_request(
        bootstrap.as_deref(),
        phase,
        &mut state.gens_ranking_requested,
    );
}

fn gens_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    manager: GensManager,
) -> Option<GensShellKey> {
    if gens_shell_visible(route, phase) {
        Some(GensShellKey {
            route,
            phase,
            manager,
        })
    } else {
        None
    }
}

fn gens_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Hud && phase != SessionPhase::Disconnected
}

fn maybe_queue_gens_ranking_request(
    bootstrap: Option<&BootstrapRuntime>,
    phase: SessionPhase,
    gens_ranking_requested: &mut bool,
) {
    if phase != SessionPhase::LoggedIn {
        *gens_ranking_requested = false;
        return;
    }

    if *gens_ranking_requested {
        return;
    }

    let Some(bootstrap) = bootstrap else {
        return;
    };

    if bootstrap.queue_gens_ranking_request() {
        *gens_ranking_requested = true;
    }
}

fn apply_gens_ranking_snapshot(snapshot: GensRankingInfo, gens_manager: &mut GensManager) {
    let gens_type = gens_type_from_influence(snapshot.influence);
    gens_manager.set_gens_type(gens_type);
    gens_manager.set_team_name(gens_team_name_from_influence(snapshot.influence));
    gens_manager.set_ranking(positive_i32_to_option_u16(snapshot.ranking));
    gens_manager.set_contribution(positive_i32_to_u32(snapshot.contribution_point));
    gens_manager.set_next_contribution(positive_i32_to_u32(snapshot.next_contribution_point));
    gens_manager.set_title_name_from_gens_class(snapshot.gens_class);
    gens_manager.mark_ranking();
}

fn gens_type_from_influence(influence: u8) -> GensType {
    match influence {
        1 => GensType::Duprian,
        2 => GensType::Vanert,
        _ => GensType::None,
    }
}

fn gens_team_name_from_influence(influence: u8) -> &'static str {
    match influence {
        1 => "Duprian",
        2 => "Vanert",
        _ => "",
    }
}

fn positive_i32_to_option_u16(value: i32) -> Option<u16> {
    if value <= 0 {
        return None;
    }

    u16::try_from(value).ok()
}

fn positive_i32_to_u32(value: i32) -> u32 {
    if value <= 0 {
        0
    } else {
        u32::try_from(value).unwrap_or(u32::MAX)
    }
}

fn gens_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    manager: &GensManager,
) -> Option<GensShellView> {
    if !gens_shell_visible(route, phase) {
        return None;
    }

    let state = gens_shell_state_for_mode(manager.mode());
    let screen = gens_ranking_screen(state);

    Some(GensShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: gens_shell_body(&screen, phase, manager),
        accent: accent_for_state(screen.state),
    })
}

fn gens_shell_state_for_mode(mode: GensMode) -> GensRankingScreenState {
    match mode {
        GensMode::Idle | GensMode::Leaving | GensMode::Ranking => GensRankingScreenState::Ready,
        GensMode::Joining => GensRankingScreenState::Join,
        GensMode::Rewarding => GensRankingScreenState::Reward,
        GensMode::Error => GensRankingScreenState::Error,
    }
}

fn gens_shell_body(
    screen: &GensRankingScreen,
    phase: SessionPhase,
    manager: &GensManager,
) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Gens shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Gens",
        &format!(
            "mode={} | screen_state={} | gens_type={} | team_name={:?} | ranking={:?} | contribution={} | next_contribution={} | title_name={:?} | reward_available={}",
            gens_mode_label(manager.mode()),
            screen.state.as_str(),
            gens_type_label(manager.gens_type()),
            maybe_text(manager.team_name()),
            manager.ranking(),
            manager.contribution(),
            manager.next_contribution(),
            maybe_text(manager.title_name()),
            manager.reward_available(),
        ),
    );

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn maybe_text(value: &str) -> Option<&str> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn gens_mode_label(mode: GensMode) -> &'static str {
    match mode {
        GensMode::Idle => "idle",
        GensMode::Joining => "joining",
        GensMode::Leaving => "leaving",
        GensMode::Rewarding => "rewarding",
        GensMode::Ranking => "ranking",
        GensMode::Error => "error",
    }
}

fn gens_type_label(gens_type: GensType) -> &'static str {
    match gens_type {
        GensType::None => "none",
        GensType::Duprian => "duprian",
        GensType::Vanert => "vanert",
    }
}

fn accent_for_state(state: GensRankingScreenState) -> Color {
    match state {
        GensRankingScreenState::Ready => READY_ACCENT,
        GensRankingScreenState::Join => JOIN_ACCENT,
        GensRankingScreenState::Reward => REWARD_ACCENT,
        GensRankingScreenState::Error => ERROR_ACCENT,
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

fn spawn_gens_shell(commands: &mut Commands, view: &GensShellView) -> Entity {
    let root = commands
        .spawn((
            GensShellRoot,
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

fn clear_gens_shell(commands: &mut Commands, state: &mut GensShellState) {
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
    use super::{
        apply_gens_ranking_snapshot, gens_shell_state_for_mode, gens_shell_view, GensShellPlugin,
        GensShellRoot,
    };
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::{GensManager, GensMode, GensPlugin, GensType};
    use mu_protocol::events::GensRankingInfo;
    use mu_ui::{GensRankingScreenState, UiRoute, UiShellState};

    fn gens_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&GensShellRoot>();
        query.iter(world).count()
    }

    fn gens_shell_root_entity(world: &mut bevy::prelude::World) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<GensShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn gens_shell_visibility_follows_route_and_disconnect() {
        assert!(super::gens_shell_visible(
            UiRoute::Hud,
            SessionPhase::LoggedIn
        ));
        assert!(!super::gens_shell_visible(
            UiRoute::Hud,
            SessionPhase::Disconnected
        ));
        assert!(!super::gens_shell_visible(
            UiRoute::Friend,
            SessionPhase::LoggedIn
        ));
    }

    #[test]
    fn gens_shell_state_follows_manager_state() {
        let mut manager = GensManager::new();
        manager.set_gens_type(GensType::Duprian);
        manager.set_team_name("NightWatch");
        manager.set_title_name_from_gens_class(12);
        manager.set_contribution(4_300);
        manager.set_next_contribution(5_000);
        manager.set_ranking(Some(12));
        manager.set_reward_available(true);
        manager.mark_ranking();

        assert_eq!(
            gens_shell_state_for_mode(GensMode::Ranking),
            GensRankingScreenState::Ready
        );

        let view = gens_shell_view(UiRoute::Hud, SessionPhase::LoggedIn, &manager)
            .expect("gens shell view");
        assert!(view.body.contains("mode=ranking | screen_state=ready"));
        assert!(view.body.contains("gens_type=duprian"));
        assert!(view.body.contains("team_name=Some(\"NightWatch\")"));
        assert!(view.body.contains("ranking=Some(12)"));
        assert!(view.body.contains("contribution=4300"));
        assert!(view.body.contains("next_contribution=5000"));
        assert!(view.body.contains("title_name=Some(\"Lieutenant\")"));
        assert!(view.body.contains("reward_available=true"));
        assert!(view.body.contains("Review Gens ranking and contribution."));

        manager.mark_joining();
        assert_eq!(
            gens_shell_state_for_mode(manager.mode()),
            GensRankingScreenState::Join
        );
        let view = gens_shell_view(UiRoute::Hud, SessionPhase::LoggedIn, &manager)
            .expect("gens shell view");
        assert!(view.body.contains("mode=joining | screen_state=join"));
        assert!(view.body.contains("Choose a Gens faction."));

        manager.set_title_name_from_gens_class(5);
        manager.mark_rewarding();
        assert_eq!(
            gens_shell_state_for_mode(manager.mode()),
            GensRankingScreenState::Reward
        );
        let view = gens_shell_view(UiRoute::Hud, SessionPhase::LoggedIn, &manager)
            .expect("gens shell view");
        assert!(view.body.contains("mode=rewarding | screen_state=reward"));
        assert!(view.body.contains("title_name=Some(\"Viscount\")"));
        assert!(view.body.contains("Claim the Gens reward."));

        manager.mark_error();
        assert_eq!(
            gens_shell_state_for_mode(manager.mode()),
            GensRankingScreenState::Error
        );
        let view = gens_shell_view(UiRoute::Hud, SessionPhase::LoggedIn, &manager)
            .expect("gens shell view");
        assert!(view.body.contains("mode=error | screen_state=error"));
        assert!(view.body.contains("Gens ranking sync failed."));
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, GensShellPlugin, GensPlugin));

        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::Hud);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        {
            let mut manager = app.world_mut().resource_mut::<GensManager>();
            manager.set_gens_type(GensType::Vanert);
            manager.set_team_name("Vanert Vanguard");
            manager.set_title_name_from_gens_class(10);
            manager.set_contribution(9_500);
            manager.set_next_contribution(10_000);
            manager.set_ranking(Some(5));
            manager.set_reward_available(true);
            manager.mark_rewarding();
        }

        app.update();
        assert_eq!(gens_shell_root_count(app.world_mut()), 1);

        {
            let mut ui_shell = app.world_mut().resource_mut::<UiShellState>();
            ui_shell.set_route(UiRoute::Friend);
        }

        app.update();
        assert_eq!(gens_shell_root_count(app.world_mut()), 0);
        assert!(gens_shell_root_entity(app.world_mut()).is_none());
    }

    #[test]
    fn gens_ranking_snapshot_hydrates_legacy_title_name() {
        let snapshot = GensRankingInfo {
            influence: 1,
            ranking: 9,
            gens_class: 12,
            contribution_point: 100,
            next_contribution_point: 200,
        };

        let mut manager = GensManager::new();
        apply_gens_ranking_snapshot(snapshot, &mut manager);

        assert_eq!(manager.title_name(), "Lieutenant");
    }
}
