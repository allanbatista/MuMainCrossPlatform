use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::PartyManager;
use mu_ui::{
    party_screen, PartyMemberPresence, PartyMemberView, PartyRowColor, PartyScreen,
    PartyScreenState, UiRoute, UiShellState,
};

use crate::{ClientRuntime, SessionPhase};

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
const PARTY_ACCENT: Color = Color::srgb(0.34, 0.69, 0.84);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);
const EMPTY_ACCENT: Color = Color::srgb(0.46, 0.54, 0.64);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct PartyShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartyShellKey {
    route: UiRoute,
    phase: SessionPhase,
    party: PartyManager,
    hero_id: Option<String>,
}

#[derive(Debug, Default, Resource)]
struct PartyShellState {
    root: Option<Entity>,
    key: Option<PartyShellKey>,
}

#[derive(Debug, Clone)]
struct PartyShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct PartyShellRoot;

impl Plugin for PartyShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PartyShellState>()
            .add_systems(PostUpdate, sync_party_shell_system);
    }
}

fn sync_party_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    runtime: Res<ClientRuntime>,
    mut state: ResMut<PartyShellState>,
) {
    let current = party_shell_key(ui_shell.current(), session_state.phase(), &runtime);

    let Some(key) = current else {
        clear_party_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_party_shell(&mut commands, &mut state);

    let Some(view) = party_shell_view(key.route, key.phase, &key.party, key.hero_id.as_deref())
    else {
        return;
    };

    let root = spawn_party_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn party_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    runtime: &ClientRuntime,
) -> Option<PartyShellKey> {
    if !party_shell_visible(route, phase) {
        return None;
    }

    let party = runtime.party().clone();
    let hero_id = party_shell_hero_id(runtime, &party);

    Some(PartyShellKey {
        route,
        phase,
        party,
        hero_id,
    })
}

fn party_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Party && phase != SessionPhase::Disconnected
}

fn party_shell_hero_id(runtime: &ClientRuntime, party: &PartyManager) -> Option<String> {
    let local_player = runtime
        .world_entities()
        .local_player()
        .filter(|player| !player.label.is_empty())?;

    if party
        .members()
        .iter()
        .take(party.party_number())
        .any(|member| member.name == local_player.label)
    {
        Some(local_player.label.clone())
    } else {
        None
    }
}

fn party_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    manager: &PartyManager,
    hero_id: Option<&str>,
) -> Option<PartyShellView> {
    if !party_shell_visible(route, phase) {
        return None;
    }

    let selected_member_index = hero_id.and_then(|hero| {
        manager
            .members()
            .iter()
            .take(manager.party_number())
            .position(|member| member.name == hero)
    });

    let state = if manager.party_number() == 0 {
        PartyScreenState::Empty
    } else if selected_member_index.is_some() {
        PartyScreenState::List
    } else {
        PartyScreenState::Info
    };

    let screen = party_screen(state, manager, hero_id, selected_member_index);

    Some(PartyShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: party_shell_body(&screen, phase),
        accent: accent_for_state(screen.state),
    })
}

fn party_shell_body(screen: &PartyScreen, phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Party shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Party",
        &format!(
            "state={} | party_present={} | party_number={} | selected_member_index={:?} | selected_character_index={:?}",
            screen.state.as_str(),
            screen.party_present,
            screen.party_number,
            screen.selected_member_index,
            screen.selected_character_index,
        ),
    );
    push_paragraph(
        &mut body,
        "Hero",
        &format!(
            "hero_id={:?} | leader_name={:?}",
            screen.hero_id, screen.leader_name,
        ),
    );

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Members",
        screen.members.iter().map(format_member_line),
        Some("No current party members."),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn format_member_line(member: &PartyMemberView) -> String {
    format!(
        "name={} | number={} | map={} | position=({}, {}) | hp={}/{} | step_hp={} | index={} | presence={} | row_color={} | leader={} | selected={} | can_leave={}",
        member.name,
        member.number,
        member.map_name,
        member.position.0,
        member.position.1,
        member.curr_hp,
        member.max_hp,
        member.hp_step,
        member.index,
        party_presence_label(member.presence),
        party_row_color_label(member.row_color),
        member.is_leader,
        member.selected,
        member.can_leave,
    )
}

fn party_presence_label(presence: PartyMemberPresence) -> &'static str {
    presence.as_str()
}

fn party_row_color_label(color: PartyRowColor) -> &'static str {
    color.as_str()
}

fn accent_for_state(state: PartyScreenState) -> Color {
    match state {
        PartyScreenState::Error => ERROR_ACCENT,
        PartyScreenState::Empty => EMPTY_ACCENT,
        PartyScreenState::Info | PartyScreenState::List => PARTY_ACCENT,
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

fn spawn_party_shell(commands: &mut Commands, view: &PartyShellView) -> Entity {
    let root = commands
        .spawn((
            PartyShellRoot,
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
            Node {
                width: Val::Percent(100.0),
                ..Default::default()
            },
        ))
        .id()
}

fn clear_party_shell(commands: &mut Commands, state: &mut PartyShellState) {
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
    output.push('\n');
}

fn push_lines<I, S>(output: &mut String, heading: &str, items: I, empty_message: Option<&str>)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    if !output.is_empty() {
        output.push('\n');
    }

    output.push_str(heading);
    output.push('\n');

    let mut wrote_any = false;
    for item in items {
        wrote_any = true;
        output.push_str("- ");
        output.push_str(item.as_ref());
        output.push('\n');
    }

    if !wrote_any {
        if let Some(empty_message) = empty_message {
            output.push_str("- ");
            output.push_str(empty_message);
            output.push('\n');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{party_shell_view, PartyShellPlugin, PartyShellRoot};
    use crate::{ClientRuntime, SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::PartyMemberInfo;
    use mu_ui::{UiRoute, UiShellState};

    fn party_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&PartyShellRoot>();
        query.iter(world).count()
    }

    fn party_shell_root_entity(world: &mut bevy::prelude::World) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<PartyShellRoot>>();
        query.iter(world).next()
    }

    fn sample_runtime() -> ClientRuntime {
        let mut runtime = ClientRuntime::new();
        runtime.party_mut().set_member(
            0,
            PartyMemberInfo {
                name: "Astra".into(),
                number: 11,
                map: 3,
                x: 12,
                y: 21,
                curr_hp: 480,
                max_hp: 600,
                step_hp: 10,
                index: 4,
            },
        );
        runtime.party_mut().set_member(
            1,
            PartyMemberInfo {
                name: "Blade".into(),
                number: 22,
                map: 6,
                x: 44,
                y: 15,
                curr_hp: 220,
                max_hp: 450,
                step_hp: 7,
                index: -1,
            },
        );
        runtime.party_mut().set_member(
            2,
            PartyMemberInfo {
                name: "Selene".into(),
                number: 33,
                map: 9,
                x: 8,
                y: 3,
                curr_hp: 100,
                max_hp: 380,
                step_hp: 3,
                index: -3,
            },
        );

        runtime
    }

    #[test]
    fn party_shell_view_renders_expected_bodies() {
        let mut manager = sample_runtime().party().clone();
        let view = party_shell_view(
            UiRoute::Party,
            SessionPhase::LoggedIn,
            &manager,
            Some("Astra"),
        )
        .unwrap();

        assert_eq!(view.title, "Party");
        assert!(view.status.contains("route=party | group=gameplay"));
        assert!(view.body.contains("Party shell is active."));
        assert!(view
            .body
            .contains("state=list | party_present=true | party_number=3"));
        assert!(view.body.contains("hero_id=Some(\"Astra\")"));
        assert!(view.body.contains("leader_name=Some(\"Astra\")"));
        assert!(view.body.contains("selected_member_index=Some(0)"));
        assert!(view.body.contains("Members"));
        assert!(view.body.contains("Astra"));
        assert!(view.body.contains("Actions"));

        manager.reset();
        let empty_view =
            party_shell_view(UiRoute::Party, SessionPhase::LoggedIn, &manager, None).unwrap();

        assert!(empty_view
            .body
            .contains("state=empty | party_present=false | party_number=0"));
        assert!(empty_view
            .body
            .contains("Invite a player to start a party."));

        assert!(
            party_shell_view(UiRoute::Party, SessionPhase::Disconnected, &manager, None).is_none()
        );
        assert!(party_shell_view(UiRoute::World, SessionPhase::LoggedIn, &manager, None).is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, PartyShellPlugin));

        let mut ui_shell = mu_ui::UiShellState::default();
        ui_shell.set_route(UiRoute::Party);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);
        app.insert_resource(sample_runtime());

        app.update();

        assert_eq!(party_shell_root_count(app.world_mut()), 1);
        let first_root = party_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut().resource_mut::<SessionState>().disconnect();
        app.update();

        assert_eq!(party_shell_root_count(app.world_mut()), 0);

        app.world_mut()
            .resource_mut::<SessionState>()
            .sync_phase(SessionPhase::LoggedIn);
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Party);
        app.world_mut()
            .resource_mut::<ClientRuntime>()
            .party_mut()
            .reset();
        app.update();

        assert_eq!(party_shell_root_count(app.world_mut()), 1);
        let second_root = party_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(party_shell_root_count(app.world_mut()), 0);
    }
}
