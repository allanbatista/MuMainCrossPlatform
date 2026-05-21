use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_ui::{
    guild_screen, GuildMemberEntry, GuildMemberRole, GuildScreen, GuildScreenState,
    GuildUnionEntry, UiRoute, UiShellState,
};

use crate::bootstrap_runtime::{BootstrapRuntime, GuildRosterSnapshot, GuildUnionRosterSnapshot};
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
const GUILD_ACCENT: Color = Color::srgb(0.38, 0.66, 0.96);
const NO_GUILD_ACCENT: Color = Color::srgb(0.58, 0.62, 0.68);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GuildShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuildShellKey {
    route: UiRoute,
    phase: SessionPhase,
    control_http_state: Option<GuildScreenState>,
    live_roster: Option<GuildRosterSnapshot>,
    live_union_roster: Option<GuildUnionRosterSnapshot>,
}

#[derive(Debug, Default, Resource)]
struct GuildShellState {
    root: Option<Entity>,
    key: Option<GuildShellKey>,
    guild_list_requested: bool,
    guild_alliance_list_requested: bool,
}

#[derive(Debug, Clone)]
struct GuildShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct GuildShellRoot;

impl Plugin for GuildShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuildShellState>()
            .add_systems(PostUpdate, sync_guild_shell_system);
    }
}

fn sync_guild_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    bootstrap: Res<BootstrapRuntime>,
    control_http: Option<Res<ControlHttpState>>,
    mut state: ResMut<GuildShellState>,
) {
    if session_state.phase() != SessionPhase::LoggedIn {
        state.guild_list_requested = false;
        state.guild_alliance_list_requested = false;
    }

    let control_http_state = control_http
        .as_deref()
        .and_then(guild_screen_state_for_control_http);
    let live_roster = bootstrap.guild_roster_snapshot();
    let live_union_roster = bootstrap.guild_union_roster_snapshot();
    let current = guild_shell_key(
        ui_shell.current(),
        session_state.phase(),
        control_http_state,
        live_roster.clone(),
        live_union_roster.clone(),
    );

    let Some(key) = current else {
        clear_guild_shell(&mut commands, &mut state);
        state.guild_list_requested = false;
        state.guild_alliance_list_requested = false;
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        maybe_queue_guild_list_request(
            &bootstrap,
            session_state.phase(),
            &mut state.guild_list_requested,
        );
        maybe_queue_guild_alliance_list_request(
            &bootstrap,
            session_state.phase(),
            key.control_http_state,
            &mut state.guild_alliance_list_requested,
        );
        return;
    }

    clear_guild_shell(&mut commands, &mut state);

    let control_http_state = key.control_http_state;
    let Some(view) = guild_shell_view(
        key.route,
        key.phase,
        control_http_state,
        key.live_roster.clone(),
        key.live_union_roster.clone(),
    ) else {
        return;
    };

    let root = spawn_guild_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
    maybe_queue_guild_list_request(
        &bootstrap,
        session_state.phase(),
        &mut state.guild_list_requested,
    );
    maybe_queue_guild_alliance_list_request(
        &bootstrap,
        session_state.phase(),
        control_http_state,
        &mut state.guild_alliance_list_requested,
    );
}

fn guild_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    control_http_state: Option<GuildScreenState>,
    live_roster: Option<GuildRosterSnapshot>,
    live_union_roster: Option<GuildUnionRosterSnapshot>,
) -> Option<GuildShellKey> {
    if !guild_shell_visible(route, phase) {
        return None;
    }

    Some(GuildShellKey {
        route,
        phase,
        control_http_state,
        live_roster,
        live_union_roster,
    })
}

fn guild_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Guild && phase != SessionPhase::Disconnected
}

fn maybe_queue_guild_list_request(
    bootstrap: &BootstrapRuntime,
    phase: SessionPhase,
    guild_list_requested: &mut bool,
) {
    if !guild_shell_should_queue_live_request(phase, *guild_list_requested) {
        return;
    }

    if bootstrap.queue_guild_list_request() {
        *guild_list_requested = true;
    }
}

fn maybe_queue_guild_alliance_list_request(
    bootstrap: &BootstrapRuntime,
    phase: SessionPhase,
    control_http_state: Option<GuildScreenState>,
    guild_alliance_list_requested: &mut bool,
) {
    if control_http_state != Some(GuildScreenState::Union) {
        *guild_alliance_list_requested = false;
        return;
    }

    if !guild_shell_should_queue_live_request(phase, *guild_alliance_list_requested) {
        return;
    }

    if bootstrap.queue_guild_alliance_list_request() {
        *guild_alliance_list_requested = true;
    }
}

fn guild_shell_should_queue_live_request(phase: SessionPhase, requested: bool) -> bool {
    phase == SessionPhase::LoggedIn && !requested
}

fn guild_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    control_http_state: Option<GuildScreenState>,
    live_roster: Option<GuildRosterSnapshot>,
    live_union_roster: Option<GuildUnionRosterSnapshot>,
) -> Option<GuildShellView> {
    if !guild_shell_visible(route, phase) {
        return None;
    }

    let state = control_http_state.unwrap_or_else(|| guild_screen_state_for_phase(phase));
    let mut screen = guild_screen(state);
    apply_live_guild_roster(&mut screen, live_roster.as_ref());
    apply_live_guild_union_roster(&mut screen, live_union_roster.as_ref());

    Some(GuildShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: guild_shell_body(&screen, phase),
        accent: accent_for_state(screen.state),
    })
}

fn apply_live_guild_roster(screen: &mut GuildScreen, live_roster: Option<&GuildRosterSnapshot>) {
    let Some(live_roster) = live_roster else {
        return;
    };

    let selected_member_name = screen.selected_member_name.as_deref();
    let self_member_name = screen
        .members
        .iter()
        .find(|member| member.is_self)
        .map(|member| member.name.clone());
    let mut members = live_roster.members.clone();

    for member in &mut members {
        member.selected = selected_member_name == Some(member.name.as_str());
        member.is_self = self_member_name.as_deref() == Some(member.name.as_str());
    }

    if !members.iter().any(|member| member.selected) {
        if let Some(first) = members.first_mut() {
            first.selected = true;
        }
    }

    let selected_member = members.iter().find(|member| member.selected).cloned();

    screen.member_count = Some(members.len());
    screen.guild_score = Some(live_roster.total_score);
    screen.rival_guild_name = live_roster.rival_guild_name.clone();
    screen.guild_master_name = members
        .iter()
        .find(|member| member.role == GuildMemberRole::Master)
        .map(|member| member.name.clone());
    screen.sub_master_name = members
        .iter()
        .find(|member| member.role == GuildMemberRole::SubMaster)
        .map(|member| member.name.clone());
    screen.battle_master_name = members
        .iter()
        .find(|member| member.role == GuildMemberRole::BattleMaster)
        .map(|member| member.name.clone());
    screen.members = members;
    screen.selected_member_name = selected_member.as_ref().map(|member| member.name.clone());
    screen.selected_member_role = selected_member.as_ref().map(|member| member.role);
    screen.selected_member_server = selected_member.and_then(|member| member.server);
}

fn apply_live_guild_union_roster(
    screen: &mut GuildScreen,
    live_union_roster: Option<&GuildUnionRosterSnapshot>,
) {
    let Some(live_union_roster) = live_union_roster else {
        return;
    };

    if screen.state != GuildScreenState::Union {
        return;
    }

    let selected_union_name = screen.selected_union_name.as_deref();
    let mut unions = live_union_roster.unions.clone();

    for union in &mut unions {
        union.selected = selected_union_name == Some(union.name.as_str());
    }

    if !unions.iter().any(|union| union.selected) {
        if let Some(first) = unions.first_mut() {
            first.selected = true;
        }
    }

    let selected_union = unions.iter().find(|union| union.selected).cloned();

    screen.unions = unions;
    screen.selected_union_name = selected_union.as_ref().map(|union| union.name.clone());
    screen.selected_union_member_count = selected_union.map(|union| union.member_count);
}

fn guild_screen_state_for_phase(phase: SessionPhase) -> GuildScreenState {
    match phase {
        SessionPhase::ReadyForLogin => GuildScreenState::NoGuild,
        SessionPhase::LoggedIn => GuildScreenState::Summary,
        SessionPhase::Disconnected => GuildScreenState::NoGuild,
    }
}

fn guild_screen_state_for_control_http(
    control_http: &ControlHttpState,
) -> Option<GuildScreenState> {
    control_http.snapshot().guild_screen_state
}

fn guild_shell_body(screen: &GuildScreen, phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Guild shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Guild",
        &format!(
            "state={} | player_role={:?} | guild_type={:?} | guild_name={:?} | guild_score={:?} | member_count={:?} | rival_guild_name={:?}",
            screen.state.as_str(),
            screen.player_role,
            screen.guild_type,
            screen.guild_name,
            screen.guild_score,
            screen.member_count,
            screen.rival_guild_name,
        ),
    );
    push_paragraph(
        &mut body,
        "Leaders",
        &format!(
            "guild_master_name={:?} | sub_master_name={:?} | battle_master_name={:?}",
            screen.guild_master_name, screen.sub_master_name, screen.battle_master_name,
        ),
    );
    push_paragraph(
        &mut body,
        "Flags",
        &format!(
            "break_up_caption={:?} | break_union_caption={:?} | ban_union_enabled={} | member_actions_visible={} | union_actions_visible={} | request_union_list={}",
            screen.break_up_caption,
            screen.break_union_caption,
            screen.ban_union_enabled,
            screen.member_actions_visible,
            screen.union_actions_visible,
            screen.request_union_list,
        ),
    );
    push_paragraph(
        &mut body,
        "Selection",
        &format!(
            "selected_member_name={:?} | selected_member_role={:?} | selected_member_server={:?} | selected_union_name={:?} | selected_union_member_count={:?}",
            screen.selected_member_name,
            screen.selected_member_role,
            screen.selected_member_server,
            screen.selected_union_name,
            screen.selected_union_member_count,
        ),
    );

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Notices",
        screen.notices.iter().cloned(),
        Some("No notices."),
    );
    push_lines(
        &mut body,
        "Members",
        screen.members.iter().map(format_member_entry),
        Some("No current members."),
    );
    push_lines(
        &mut body,
        "Unions",
        screen.unions.iter().map(format_union_entry),
        Some("No current unions."),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn format_member_entry(entry: &GuildMemberEntry) -> String {
    format!(
        "name={} | number={} | server={:?} | role={} | selected={} | is_self={}",
        entry.name,
        entry.number,
        entry.server,
        guild_role_label(entry.role),
        entry.selected,
        entry.is_self,
    )
}

fn format_union_entry(entry: &GuildUnionEntry) -> String {
    format!(
        "name={} | member_count={} | selected={}",
        entry.name, entry.member_count, entry.selected,
    )
}

fn guild_role_label(role: GuildMemberRole) -> &'static str {
    match role {
        GuildMemberRole::Master => "master",
        GuildMemberRole::SubMaster => "sub-master",
        GuildMemberRole::BattleMaster => "battle-master",
        GuildMemberRole::Member => "member",
    }
}

fn accent_for_state(state: GuildScreenState) -> Color {
    match state {
        GuildScreenState::Summary | GuildScreenState::Members | GuildScreenState::Union => {
            GUILD_ACCENT
        }
        GuildScreenState::NoGuild => NO_GUILD_ACCENT,
        GuildScreenState::Error => ERROR_ACCENT,
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

fn spawn_guild_shell(commands: &mut Commands, view: &GuildShellView) -> Entity {
    let root = commands
        .spawn((
            GuildShellRoot,
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

fn clear_guild_shell(commands: &mut Commands, state: &mut GuildShellState) {
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
        guild_screen_state_for_phase, guild_shell_key, guild_shell_view, guild_shell_visible,
        GuildShellPlugin, GuildShellRoot,
    };
    use crate::bootstrap_runtime::{
        BootstrapRuntime, GuildRosterSnapshot, GuildUnionRosterSnapshot,
    };
    use crate::control_http::{ControlHttpState, ControlSnapshot};
    use crate::{AppState, SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_ui::{
        GuildMemberEntry, GuildMemberRole, GuildScreenState, GuildUnionEntry, UiRoute, UiShellState,
    };
    use std::sync::{Arc, Mutex};

    fn guild_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&GuildShellRoot>();
        query.iter(world).count()
    }

    fn guild_list_request_count(world: &bevy::prelude::World) -> usize {
        world
            .resource::<BootstrapRuntime>()
            .guild_list_request_count()
    }

    fn guild_alliance_list_request_count(world: &bevy::prelude::World) -> usize {
        world
            .resource::<BootstrapRuntime>()
            .guild_alliance_list_request_count()
    }

    fn live_guild_roster() -> GuildRosterSnapshot {
        GuildRosterSnapshot {
            result: 0x52,
            count: 2,
            total_score: 12_500,
            score: 7,
            rival_guild_name: Some("Red".to_owned()),
            members: vec![
                GuildMemberEntry {
                    name: "Astra".to_owned(),
                    number: 11,
                    server: Some(3),
                    role: GuildMemberRole::Master,
                    selected: false,
                    is_self: true,
                },
                GuildMemberEntry {
                    name: "Blade".to_owned(),
                    number: 22,
                    server: Some(5),
                    role: GuildMemberRole::SubMaster,
                    selected: false,
                    is_self: false,
                },
            ],
        }
    }

    fn live_guild_union_roster() -> GuildUnionRosterSnapshot {
        GuildUnionRosterSnapshot {
            result: 1,
            count: 2,
            rival_count: 1,
            union_count: 2,
            unions: vec![
                GuildUnionEntry {
                    name: "Alliance".to_owned(),
                    member_count: 14,
                    selected: false,
                },
                GuildUnionEntry {
                    name: "Wardens".to_owned(),
                    member_count: 18,
                    selected: false,
                },
            ],
        }
    }

    fn control_http_state(guild_screen_state: GuildScreenState) -> ControlHttpState {
        let snapshot = Arc::new(Mutex::new(ControlSnapshot::new(AppState::ReadyForLogin)));
        snapshot
            .lock()
            .expect("control snapshot poisoned")
            .guild_screen_state = Some(guild_screen_state);
        ControlHttpState::new(snapshot)
    }

    #[test]
    fn guild_shell_visibility_follows_route_and_disconnect() {
        assert!(guild_shell_visible(UiRoute::Guild, SessionPhase::LoggedIn));
        assert!(!guild_shell_visible(
            UiRoute::Guild,
            SessionPhase::Disconnected
        ));
        assert!(!guild_shell_visible(
            UiRoute::Friend,
            SessionPhase::LoggedIn
        ));
    }

    #[test]
    fn guild_shell_state_follows_phase() {
        assert_eq!(
            guild_screen_state_for_phase(SessionPhase::LoggedIn),
            GuildScreenState::Summary
        );
        assert_eq!(
            guild_screen_state_for_phase(SessionPhase::ReadyForLogin),
            GuildScreenState::NoGuild
        );

        let view = guild_shell_view(UiRoute::Guild, SessionPhase::LoggedIn, None, None, None)
            .expect("guild view");
        assert!(view.body.contains("state=summary"));

        let view = guild_shell_view(
            UiRoute::Guild,
            SessionPhase::ReadyForLogin,
            None,
            None,
            None,
        )
        .expect("guild view");
        assert!(view.body.contains("state=no-guild"));
    }

    #[test]
    fn guild_shell_prefers_control_http_override() {
        let view = guild_shell_view(
            UiRoute::Guild,
            SessionPhase::LoggedIn,
            Some(GuildScreenState::Union),
            None,
            None,
        )
        .expect("guild view");

        assert!(view.body.contains("state=union"));
    }

    #[test]
    fn guild_shell_overlays_live_roster_snapshot() {
        let view = guild_shell_view(
            UiRoute::Guild,
            SessionPhase::LoggedIn,
            None,
            Some(live_guild_roster()),
            None,
        )
        .expect("guild view");

        assert!(view.body.contains("guild_score=Some(12500)"));
        assert!(view.body.contains("rival_guild_name=Some(\"Red\")"));
        assert!(view.body.contains("guild_master_name=Some(\"Astra\")"));
        assert!(view.body.contains("sub_master_name=Some(\"Blade\")"));
        assert!(view.body.contains("battle_master_name=None"));
        assert!(view.body.contains("name=Astra"));
        assert!(view.body.contains("role=master"));
        assert!(view.body.contains("name=Blade"));
        assert!(view.body.contains("role=sub-master"));
    }

    #[test]
    fn guild_shell_key_changes_with_live_roster_snapshot() {
        let roster_a = Some(live_guild_roster());
        let mut roster_b = live_guild_roster();
        roster_b.members[1].role = GuildMemberRole::BattleMaster;

        let key_a = guild_shell_key(UiRoute::Guild, SessionPhase::LoggedIn, None, roster_a, None)
            .expect("guild key");
        let key_b = guild_shell_key(
            UiRoute::Guild,
            SessionPhase::LoggedIn,
            None,
            Some(roster_b),
            None,
        )
        .expect("guild key");

        assert_ne!(key_a, key_b);
    }

    #[test]
    fn guild_shell_overlays_live_union_roster_snapshot() {
        let view = guild_shell_view(
            UiRoute::Guild,
            SessionPhase::LoggedIn,
            Some(GuildScreenState::Union),
            None,
            Some(live_guild_union_roster()),
        )
        .expect("guild view");

        assert!(view.body.contains("state=union"));
        assert!(view.body.contains("name=Alliance"));
        assert!(view.body.contains("member_count=14"));
        assert!(view.body.contains("name=Wardens"));
        assert!(view.body.contains("member_count=18"));
        assert!(view.body.contains("selected_union_name=Some(\"Alliance\")"));
        assert!(view.body.contains("selected_union_member_count=Some(14)"));
        assert!(!view.body.contains("Vanert Vanguard"));
    }

    #[test]
    fn guild_shell_key_changes_with_live_union_roster_snapshot() {
        let roster_a = Some(live_guild_union_roster());
        let mut roster_b = live_guild_union_roster();
        roster_b.unions[1].member_count = 19;

        let key_a = guild_shell_key(
            UiRoute::Guild,
            SessionPhase::LoggedIn,
            Some(GuildScreenState::Union),
            None,
            roster_a,
        )
        .expect("guild key");
        let key_b = guild_shell_key(
            UiRoute::Guild,
            SessionPhase::LoggedIn,
            Some(GuildScreenState::Union),
            None,
            Some(roster_b),
        )
        .expect("guild key");

        assert_ne!(key_a, key_b);
    }

    #[test]
    fn guild_shell_requests_live_data_once_per_logged_in_activation() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, GuildShellPlugin));

        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::Guild);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        app.insert_resource(crate::bootstrap_runtime::BootstrapRuntime::test_stub());
        app.insert_resource(control_http_state(GuildScreenState::Summary));

        app.update();

        assert_eq!(guild_shell_root_count(app.world_mut()), 1);
        assert_eq!(guild_list_request_count(app.world()), 1);
        assert_eq!(guild_alliance_list_request_count(app.world()), 0);

        {
            let shared_snapshot = app.world().resource::<ControlHttpState>().shared_snapshot();
            shared_snapshot
                .lock()
                .expect("control snapshot poisoned")
                .guild_screen_state = Some(GuildScreenState::Union);
        }
        app.update();

        assert_eq!(guild_shell_root_count(app.world_mut()), 1);
        assert_eq!(guild_list_request_count(app.world()), 1);
        assert_eq!(guild_alliance_list_request_count(app.world()), 1);

        app.update();

        assert_eq!(guild_shell_root_count(app.world_mut()), 1);
        assert_eq!(guild_list_request_count(app.world()), 1);
        assert_eq!(guild_alliance_list_request_count(app.world()), 1);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(guild_shell_root_count(app.world_mut()), 0);
        assert_eq!(guild_list_request_count(app.world()), 1);
        assert_eq!(guild_alliance_list_request_count(app.world()), 1);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Guild);
        app.update();

        assert_eq!(guild_shell_root_count(app.world_mut()), 1);
        assert_eq!(guild_list_request_count(app.world()), 2);
        assert_eq!(guild_alliance_list_request_count(app.world()), 2);

        app.world_mut().resource_mut::<SessionState>().logout();
        app.update();

        assert_eq!(guild_shell_root_count(app.world_mut()), 1);
        assert_eq!(guild_list_request_count(app.world()), 2);
        assert_eq!(guild_alliance_list_request_count(app.world()), 2);

        app.world_mut()
            .resource_mut::<SessionState>()
            .login_success();
        app.update();

        assert_eq!(guild_shell_root_count(app.world_mut()), 1);
        assert_eq!(guild_list_request_count(app.world()), 3);
        assert_eq!(guild_alliance_list_request_count(app.world()), 3);
    }
}
