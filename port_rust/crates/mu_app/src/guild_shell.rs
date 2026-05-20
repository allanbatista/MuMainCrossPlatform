use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_ui::{
    guild_screen, GuildMemberEntry, GuildMemberRole, GuildScreen, GuildScreenState,
    GuildUnionEntry, UiRoute, UiShellState,
};

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
const GUILD_ACCENT: Color = Color::srgb(0.38, 0.66, 0.96);
const NO_GUILD_ACCENT: Color = Color::srgb(0.58, 0.62, 0.68);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GuildShellPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GuildShellKey {
    route: UiRoute,
    phase: SessionPhase,
}

#[derive(Debug, Default, Resource)]
struct GuildShellState {
    root: Option<Entity>,
    key: Option<GuildShellKey>,
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
    mut state: ResMut<GuildShellState>,
) {
    let current = guild_shell_key(ui_shell.current(), session_state.phase());

    let Some(key) = current else {
        clear_guild_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_guild_shell(&mut commands, &mut state);

    let Some(view) = guild_shell_view(key.route, key.phase) else {
        return;
    };

    let root = spawn_guild_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn guild_shell_key(route: UiRoute, phase: SessionPhase) -> Option<GuildShellKey> {
    if !guild_shell_visible(route, phase) {
        return None;
    }

    Some(GuildShellKey { route, phase })
}

fn guild_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Guild && phase != SessionPhase::Disconnected
}

fn guild_shell_view(route: UiRoute, phase: SessionPhase) -> Option<GuildShellView> {
    if !guild_shell_visible(route, phase) {
        return None;
    }

    let screen = guild_screen(guild_screen_state_for_phase(phase));

    Some(GuildShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: guild_shell_body(&screen, phase),
        accent: accent_for_state(screen.state),
    })
}

fn guild_screen_state_for_phase(phase: SessionPhase) -> GuildScreenState {
    match phase {
        SessionPhase::ReadyForLogin => GuildScreenState::NoGuild,
        SessionPhase::LoggedIn => GuildScreenState::Summary,
        SessionPhase::Disconnected => GuildScreenState::NoGuild,
    }
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
    use super::{guild_screen_state_for_phase, guild_shell_view, guild_shell_visible};
    use crate::SessionPhase;
    use mu_ui::{GuildScreenState, UiRoute};

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

        let view = guild_shell_view(UiRoute::Guild, SessionPhase::LoggedIn).expect("guild view");
        assert!(view.body.contains("state=summary"));

        let view =
            guild_shell_view(UiRoute::Guild, SessionPhase::ReadyForLogin).expect("guild view");
        assert!(view.body.contains("state=no-guild"));
    }
}
