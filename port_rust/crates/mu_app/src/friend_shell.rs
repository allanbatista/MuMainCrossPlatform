use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{MailManager, MailMode};
use mu_ui::{
    friend_screen, ChatRoomEntry, FriendEntry, FriendPresence, FriendScreen, FriendScreenState,
    LetterEntry, UiRoute, UiShellState,
};

use crate::bootstrap_runtime::{BootstrapRuntime, FriendRosterSnapshot};
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
const ROSTER_ACCENT: Color = Color::srgb(0.34, 0.69, 0.84);
const INBOX_ACCENT: Color = Color::srgb(0.42, 0.82, 0.58);
const COMPOSE_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);
const CHAT_ROOMS_ACCENT: Color = Color::srgb(0.58, 0.62, 0.68);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct FriendShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FriendShellKey {
    route: UiRoute,
    phase: SessionPhase,
    mail: MailManager,
    control_http_state: Option<FriendScreenState>,
    live_roster: Option<FriendRosterSnapshot>,
}

#[derive(Debug, Default, Resource)]
struct FriendShellState {
    root: Option<Entity>,
    key: Option<FriendShellKey>,
    friend_list_requested: bool,
}

#[derive(Debug, Clone)]
struct FriendShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct FriendShellRoot;

impl Plugin for FriendShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FriendShellState>()
            .add_systems(PostUpdate, sync_friend_shell_system);
    }
}

fn sync_friend_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    mail: Res<MailManager>,
    bootstrap: Res<BootstrapRuntime>,
    control_http: Option<Res<ControlHttpState>>,
    mut state: ResMut<FriendShellState>,
) {
    if session_state.phase() != SessionPhase::LoggedIn {
        state.friend_list_requested = false;
    }

    let control_http_state = control_http
        .as_deref()
        .and_then(friend_screen_state_for_control_http);
    let live_roster = bootstrap.friend_roster_snapshot();
    let current = friend_shell_key(
        ui_shell.current(),
        session_state.phase(),
        &mail,
        control_http_state,
        live_roster.clone(),
    );

    let Some(key) = current else {
        clear_friend_shell(&mut commands, &mut state);
        state.friend_list_requested = false;
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        maybe_queue_friend_list_request(
            &bootstrap,
            session_state.phase(),
            &mut state.friend_list_requested,
        );
        return;
    }

    clear_friend_shell(&mut commands, &mut state);

    let Some(view) = friend_shell_view(
        key.route,
        key.phase,
        &key.mail,
        key.control_http_state,
        key.live_roster.clone(),
    ) else {
        return;
    };

    let root = spawn_friend_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
    maybe_queue_friend_list_request(
        &bootstrap,
        session_state.phase(),
        &mut state.friend_list_requested,
    );
}

fn friend_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    mail: &MailManager,
    control_http_state: Option<FriendScreenState>,
    live_roster: Option<FriendRosterSnapshot>,
) -> Option<FriendShellKey> {
    if !friend_shell_visible(route, phase) {
        return None;
    }

    Some(FriendShellKey {
        route,
        phase,
        mail: mail.clone(),
        control_http_state,
        live_roster,
    })
}

fn friend_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Friend && phase != SessionPhase::Disconnected
}

fn maybe_queue_friend_list_request(
    bootstrap: &BootstrapRuntime,
    phase: SessionPhase,
    friend_list_requested: &mut bool,
) {
    if !friend_shell_should_queue_live_request(phase, *friend_list_requested) {
        return;
    }

    if bootstrap.queue_friend_list_request() {
        *friend_list_requested = true;
    }
}

fn friend_shell_should_queue_live_request(phase: SessionPhase, requested: bool) -> bool {
    phase == SessionPhase::LoggedIn && !requested
}

fn friend_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    mail: &MailManager,
    control_http_state: Option<FriendScreenState>,
    live_roster: Option<FriendRosterSnapshot>,
) -> Option<FriendShellView> {
    if !friend_shell_visible(route, phase) {
        return None;
    }

    let state = control_http_state.unwrap_or_else(|| friend_screen_state_for_mail(mail));
    let mut screen = friend_screen(state, mail);
    apply_live_friend_roster(&mut screen, live_roster.as_ref());

    Some(FriendShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: friend_shell_body(&screen, phase, mail),
        accent: accent_for_state(screen.state),
    })
}

fn apply_live_friend_roster(screen: &mut FriendScreen, live_roster: Option<&FriendRosterSnapshot>) {
    let Some(live_roster) = live_roster else {
        return;
    };

    let selected_friend_name = screen.selected_friend.as_deref();
    let mut friends = live_roster.friends.clone();

    for friend in &mut friends {
        friend.selected = selected_friend_name == Some(friend.name.as_str());
    }

    if !friends.iter().any(|friend| friend.selected) {
        if let Some(first) = friends.first_mut() {
            first.selected = true;
        }
    }

    let selected_friend = friends
        .iter()
        .find(|friend| friend.selected)
        .map(|friend| friend.name.clone());
    let selected_friend_server = friends
        .iter()
        .find(|friend| friend.selected)
        .and_then(|friend| friend.server);

    screen.friend_count = friends.len();
    screen.friends = friends;
    screen.selected_friend = selected_friend;
    screen.selected_friend_server = selected_friend_server;
}

fn friend_screen_state_for_mail(mail: &MailManager) -> FriendScreenState {
    match mail.mode() {
        MailMode::Inbox => FriendScreenState::Roster,
        MailMode::Reading => FriendScreenState::Inbox,
        MailMode::Compose => FriendScreenState::Compose,
        MailMode::Error => FriendScreenState::Error,
    }
}

fn friend_screen_state_for_control_http(
    control_http: &ControlHttpState,
) -> Option<FriendScreenState> {
    control_http.snapshot().friend_screen_state
}

fn friend_shell_body(screen: &FriendScreen, phase: SessionPhase, mail: &MailManager) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Friend shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Mail",
        &format!(
            "mode={} | new_mail_alert={} | webzen_mail={}",
            mail_mode_label(mail.mode()),
            mail.new_mail_alert(),
            mail.webzen_mail(),
        ),
    );
    push_paragraph(
        &mut body,
        "Friend",
        &format!(
            "state={} | tab_index={} | chat_reject={} | friend_sort={:?} | letter_sort={:?} | friend_count={} | letter_count={} | chat_room_count={}",
            screen.state.as_str(),
            screen.tab_index,
            screen.chat_reject,
            screen.friend_sort,
            screen.letter_sort,
            screen.friend_count,
            screen.letter_count,
            screen.chat_room_count,
        ),
    );
    push_paragraph(
        &mut body,
        "Selection",
        &format!(
            "selected_friend={:?} | selected_friend_server={:?} | selected_letter_id={:?} | selected_letter_sender={:?} | selected_letter_subject={:?} | selected_chat_room_id={:?} | selected_chat_room_title={:?}",
            screen.selected_friend,
            screen.selected_friend_server,
            screen.selected_letter_id,
            screen.selected_letter_sender,
            screen.selected_letter_subject,
            screen.selected_chat_room_id,
            screen.selected_chat_room_title,
        ),
    );

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    if matches!(screen.state, FriendScreenState::Compose) {
        push_paragraph(
            &mut body,
            "Draft",
            &format!(
                "recipient={:?} | subject={:?} | body={:?}",
                screen.draft_recipient, screen.draft_subject, screen.draft_body,
            ),
        );
    }

    push_lines(
        &mut body,
        "Friends",
        screen.friends.iter().map(format_friend_entry),
        Some("No current friends."),
    );
    push_lines(
        &mut body,
        "Letters",
        screen.letters.iter().map(format_letter_entry),
        Some("No current letters."),
    );
    push_lines(
        &mut body,
        "Chat rooms",
        screen.chat_rooms.iter().map(format_chat_room_entry),
        Some("No current chat rooms."),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn format_friend_entry(entry: &FriendEntry) -> String {
    format!(
        "name={} | server={:?} | presence={} | selected={}",
        entry.name,
        entry.server,
        friend_presence_label(entry.presence),
        entry.selected,
    )
}

fn format_letter_entry(entry: &LetterEntry) -> String {
    format!(
        "id={} | sender={} | subject={} | date={} | time={} | read={} | selected={}",
        entry.id, entry.sender, entry.subject, entry.date, entry.time, entry.read, entry.selected,
    )
}

fn format_chat_room_entry(entry: &ChatRoomEntry) -> String {
    format!(
        "room_id={} | title={} | member_count={} | selected={}",
        entry.room_id, entry.title, entry.member_count, entry.selected,
    )
}

fn friend_presence_label(presence: FriendPresence) -> &'static str {
    match presence {
        FriendPresence::Online => "online",
        FriendPresence::Offline => "offline",
        FriendPresence::Busy => "busy",
        FriendPresence::Locked => "locked",
    }
}

fn mail_mode_label(mode: MailMode) -> &'static str {
    match mode {
        MailMode::Inbox => "inbox",
        MailMode::Compose => "compose",
        MailMode::Reading => "reading",
        MailMode::Error => "error",
    }
}

fn accent_for_state(state: FriendScreenState) -> Color {
    match state {
        FriendScreenState::Roster => ROSTER_ACCENT,
        FriendScreenState::Inbox => INBOX_ACCENT,
        FriendScreenState::Compose => COMPOSE_ACCENT,
        FriendScreenState::ChatRooms => CHAT_ROOMS_ACCENT,
        FriendScreenState::Error => ERROR_ACCENT,
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

fn spawn_friend_shell(commands: &mut Commands, view: &FriendShellView) -> Entity {
    let root = commands
        .spawn((
            FriendShellRoot,
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

fn clear_friend_shell(commands: &mut Commands, state: &mut FriendShellState) {
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
        friend_screen_state_for_mail, friend_shell_key, friend_shell_view, friend_shell_visible,
        FriendShellPlugin, FriendShellRoot,
    };
    use crate::bootstrap_runtime::{BootstrapRuntime, FriendRosterSnapshot};
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::MailManager;
    use mu_gameplay::MailPlugin;
    use mu_ui::{FriendEntry, FriendPresence, FriendScreenState, UiRoute, UiShellState};

    fn friend_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&FriendShellRoot>();
        query.iter(world).count()
    }

    fn friend_list_request_count(world: &bevy::prelude::World) -> usize {
        world
            .resource::<BootstrapRuntime>()
            .friend_list_request_count()
    }

    fn live_friend_roster() -> FriendRosterSnapshot {
        FriendRosterSnapshot {
            memo_count: 1,
            max_memo: 8,
            count: 2,
            friends: vec![
                FriendEntry {
                    name: "Astra".to_owned(),
                    server: Some(3),
                    presence: FriendPresence::Online,
                    selected: false,
                },
                FriendEntry {
                    name: "Blade".to_owned(),
                    server: None,
                    presence: FriendPresence::Busy,
                    selected: false,
                },
            ],
        }
    }

    #[test]
    fn friend_shell_visibility_follows_route_and_disconnect() {
        assert!(friend_shell_visible(
            UiRoute::Friend,
            SessionPhase::LoggedIn
        ));
        assert!(!friend_shell_visible(
            UiRoute::Friend,
            SessionPhase::Disconnected
        ));
        assert!(!friend_shell_visible(
            UiRoute::Guild,
            SessionPhase::LoggedIn
        ));
    }

    #[test]
    fn friend_shell_state_follows_mail_mode() {
        let mail = MailManager::new();
        assert_eq!(
            friend_screen_state_for_mail(&mail),
            FriendScreenState::Roster
        );
        let view = friend_shell_view(UiRoute::Friend, SessionPhase::LoggedIn, &mail, None, None)
            .expect("friend shell view");
        assert!(view.body.contains("state=roster"));

        let mut mail = MailManager::new();
        mail.select_letter(0x0102_0304);
        let view = friend_shell_view(UiRoute::Friend, SessionPhase::LoggedIn, &mail, None, None)
            .expect("friend shell view");
        assert_eq!(
            friend_screen_state_for_mail(&mail),
            FriendScreenState::Inbox
        );
        assert!(view.body.contains("state=inbox"));

        let mut mail = MailManager::new();
        mail.set_compose("Blade", "Re: Castle prep", "Meet at Lorencia.");
        let view = friend_shell_view(UiRoute::Friend, SessionPhase::LoggedIn, &mail, None, None)
            .expect("friend shell view");
        assert_eq!(
            friend_screen_state_for_mail(&mail),
            FriendScreenState::Compose
        );
        assert!(view.body.contains("state=compose"));

        let mut mail = MailManager::new();
        mail.mark_error();
        let view = friend_shell_view(UiRoute::Friend, SessionPhase::LoggedIn, &mail, None, None)
            .expect("friend shell view");
        assert_eq!(
            friend_screen_state_for_mail(&mail),
            FriendScreenState::Error
        );
        assert!(view.body.contains("state=error"));
    }

    #[test]
    fn friend_shell_prefers_control_http_override() {
        let mail = MailManager::new();

        let view = friend_shell_view(
            UiRoute::Friend,
            SessionPhase::LoggedIn,
            &mail,
            Some(FriendScreenState::ChatRooms),
            None,
        )
        .expect("friend shell view");

        assert!(view.body.contains("state=chat-rooms"));
    }

    #[test]
    fn friend_shell_overlays_live_roster_snapshot() {
        let mail = MailManager::new();
        let view = friend_shell_view(
            UiRoute::Friend,
            SessionPhase::LoggedIn,
            &mail,
            None,
            Some(live_friend_roster()),
        )
        .expect("friend shell view");

        assert!(view.body.contains("friend_count=2"));
        assert!(view.body.contains("name=Astra"));
        assert!(view.body.contains("presence=online"));
        assert!(view.body.contains("name=Blade"));
        assert!(view.body.contains("presence=busy"));
    }

    #[test]
    fn friend_shell_key_changes_with_live_roster_snapshot() {
        let mail = MailManager::new();
        let roster_a = Some(live_friend_roster());
        let mut roster_b = live_friend_roster();
        roster_b.friends[1].presence = FriendPresence::Offline;

        let key_a = friend_shell_key(
            UiRoute::Friend,
            SessionPhase::LoggedIn,
            &mail,
            None,
            roster_a,
        )
        .expect("friend key");
        let key_b = friend_shell_key(
            UiRoute::Friend,
            SessionPhase::LoggedIn,
            &mail,
            None,
            Some(roster_b),
        )
        .expect("friend key");

        assert_ne!(key_a, key_b);
    }

    #[test]
    fn friend_shell_requests_live_data_once_per_logged_in_activation() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, FriendShellPlugin, MailPlugin));

        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::Friend);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        app.insert_resource(BootstrapRuntime::test_stub());

        app.update();

        assert_eq!(friend_shell_root_count(app.world_mut()), 1);
        assert_eq!(friend_list_request_count(app.world()), 1);

        app.world_mut().resource_mut::<MailManager>().set_compose(
            "Blade",
            "Re: patrol",
            "Meet at Davias.",
        );
        app.update();

        assert_eq!(friend_shell_root_count(app.world_mut()), 1);
        assert_eq!(friend_list_request_count(app.world()), 1);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(friend_shell_root_count(app.world_mut()), 0);
        assert_eq!(friend_list_request_count(app.world()), 1);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Friend);
        app.update();

        assert_eq!(friend_shell_root_count(app.world_mut()), 1);
        assert_eq!(friend_list_request_count(app.world()), 2);

        app.world_mut().resource_mut::<SessionState>().logout();
        app.update();

        assert_eq!(friend_shell_root_count(app.world_mut()), 1);
        assert_eq!(friend_list_request_count(app.world()), 2);

        app.world_mut()
            .resource_mut::<SessionState>()
            .login_success();
        app.update();

        assert_eq!(friend_shell_root_count(app.world_mut()), 1);
        assert_eq!(friend_list_request_count(app.world()), 3);
    }
}
