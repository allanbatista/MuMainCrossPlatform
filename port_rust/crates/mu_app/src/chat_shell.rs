use crate::chat_composer::ChatComposerState;
use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_ui::{chat_screen, ChatMessageCount, ChatMessageType, ChatScreen, UiRoute, UiShellState};

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
const CHAT_ACCENT: Color = Color::srgb(0.34, 0.69, 0.84);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ChatShellPlugin;

#[derive(Debug, Clone, PartialEq)]
struct ChatShellKey {
    route: UiRoute,
    phase: SessionPhase,
    screen: ChatScreen,
    composer: ChatComposerState,
}

#[derive(Debug, Default, Resource)]
struct ChatShellState {
    root: Option<Entity>,
    key: Option<ChatShellKey>,
}

#[derive(Debug, Clone)]
struct ChatShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct ChatShellRoot;

impl Plugin for ChatShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChatComposerState>()
            .init_resource::<ChatShellState>()
            .add_systems(PostUpdate, sync_chat_shell_system);
    }
}

fn sync_chat_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    composer: Res<ChatComposerState>,
    mut state: ResMut<ChatShellState>,
) {
    let current = chat_shell_key(ui_shell.current(), session_state.phase(), &composer);

    let Some(key) = current else {
        clear_chat_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_chat_shell(&mut commands, &mut state);

    let Some(view) = chat_shell_view(key.route, key.phase, &key.screen, &key.composer) else {
        return;
    };

    let root = spawn_chat_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn chat_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    composer: &ChatComposerState,
) -> Option<ChatShellKey> {
    if !chat_shell_visible(route, phase) {
        return None;
    }

    Some(ChatShellKey {
        route,
        phase,
        screen: chat_screen(),
        composer: composer.clone(),
    })
}

fn chat_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Chat && phase != SessionPhase::Disconnected
}

fn chat_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    screen: &ChatScreen,
    composer: &ChatComposerState,
) -> Option<ChatShellView> {
    if !chat_shell_visible(route, phase) {
        return None;
    }

    Some(ChatShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: chat_shell_body(screen, phase, composer),
        accent: CHAT_ACCENT,
    })
}

fn chat_shell_body(
    screen: &ChatScreen,
    phase: SessionPhase,
    composer: &ChatComposerState,
) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Chat shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "View",
        &format!(
            "state={} | show_chat_log={} | show_frame={} | current_message_type={} | showing_lines={} | render_end_line={} | back_alpha={}",
            screen.state.as_str(),
            screen.show_chat_log,
            screen.show_frame,
            screen.current_message_type.as_str(),
            screen.showing_lines,
            screen.render_end_line,
            screen.back_alpha,
        ),
    );
    push_paragraph(
        &mut body,
        "Pointer",
        &format!("pointed_message_index={:?}", screen.pointed_message_index),
    );
    push_paragraph(
        &mut body,
        "Composer",
        &format!(
            "draft={} | length={}/{}",
            chat_draft_label(composer),
            composer.draft_char_count(),
            crate::chat_composer::CHAT_DRAFT_CHAR_LIMIT
        ),
    );
    push_paragraph(
        &mut body,
        "Feedback",
        composer
            .last_status()
            .unwrap_or("Type a message and press Enter."),
    );
    if let Some(last_sent) = composer.last_sent() {
        push_paragraph(&mut body, "Last sent", last_sent);
    }
    push_paragraph(&mut body, "Layout", &format!("{:?}", screen.layout));
    push_paragraph(
        &mut body,
        "Widgets",
        &format!("{:?}", screen.widgets.widgets()),
    );
    push_lines(
        &mut body,
        "Filters",
        screen.filters.iter().copied(),
        Some("No filters enabled."),
    );
    push_lines(
        &mut body,
        "Message counts",
        screen.message_counts.iter().map(chat_message_count_label),
        None,
    );

    body
}

fn chat_draft_label(composer: &ChatComposerState) -> String {
    if composer.draft().is_empty() {
        "<empty>".to_string()
    } else {
        composer.draft().to_string()
    }
}

fn chat_message_count_label(count: &ChatMessageCount) -> String {
    format!(
        "{} | {}",
        chat_message_type_label(count.message_type),
        count.count
    )
}

fn chat_message_type_label(message_type: ChatMessageType) -> &'static str {
    message_type.as_str()
}

fn status_line(route: UiRoute, phase: SessionPhase) -> String {
    format!(
        "route={} | group={} | session={}",
        route.slug(),
        route.group().as_str(),
        phase.as_str()
    )
}

fn spawn_chat_shell(commands: &mut Commands, view: &ChatShellView) -> Entity {
    let root = commands
        .spawn((
            ChatShellRoot,
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

fn clear_chat_shell(commands: &mut Commands, state: &mut ChatShellState) {
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
    use super::{chat_shell_view, ChatShellPlugin, ChatShellRoot};
    use crate::chat_composer::ChatComposerState;
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_ui::{UiRoute, UiShellState};

    fn chat_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&ChatShellRoot>();
        query.iter(world).count()
    }

    fn chat_shell_root_entity(world: &mut bevy::prelude::World) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<ChatShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn chat_shell_view_renders_expected_bodies() {
        let view = chat_shell_view(
            UiRoute::Chat,
            SessionPhase::LoggedIn,
            &mu_ui::chat_screen(),
            &ChatComposerState::default(),
        )
        .expect("chat view missing");

        assert_eq!(view.title, "Chat");
        assert!(view.status.contains("route=chat | group=world"));
        assert!(view.body.contains("Chat shell is active."));
        assert!(view.body.contains("state=docked"));
        assert!(view.body.contains("current_message_type=all"));
        assert!(view.body.contains("showing_lines=6"));
        assert!(view.body.contains("Layout"));
        assert!(view.body.contains("Widgets"));
        assert!(view.body.contains("Filters"));
        assert!(view.body.contains("Message counts"));
        assert!(view.body.contains("all | 12"));
        assert!(view.body.contains("whisper | 2"));
        assert!(view.body.contains("Composer"));
        assert!(view.body.contains("<empty>"));
        assert!(view.body.contains("Type a message and press Enter."));

        let mut composer = ChatComposerState::default();
        composer.push_text("hello");
        composer.record_status("Typing...");
        let view = chat_shell_view(
            UiRoute::Chat,
            SessionPhase::LoggedIn,
            &mu_ui::chat_screen(),
            &composer,
        )
        .expect("chat view missing");
        assert!(view.body.contains("draft=hello"));
        assert!(view.body.contains("Typing..."));

        let mut composer = ChatComposerState::default();
        composer.mark_sent("Player", "hello");
        let view = chat_shell_view(
            UiRoute::Chat,
            SessionPhase::LoggedIn,
            &mu_ui::chat_screen(),
            &composer,
        )
        .expect("chat view missing");
        assert!(view.body.contains("Last sent"));
        assert!(view.body.contains("Sent as Player."));

        assert!(chat_shell_view(
            UiRoute::Chat,
            SessionPhase::Disconnected,
            &mu_ui::chat_screen(),
            &ChatComposerState::default(),
        )
        .is_none());
        assert!(chat_shell_view(
            UiRoute::World,
            SessionPhase::LoggedIn,
            &mu_ui::chat_screen(),
            &ChatComposerState::default(),
        )
        .is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, ChatShellPlugin));

        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::Chat);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        app.update();

        assert_eq!(chat_shell_root_count(app.world_mut()), 1);
        let first_root = chat_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut()
            .resource_mut::<ChatComposerState>()
            .push_text("hello");
        app.update();

        assert_eq!(chat_shell_root_count(app.world_mut()), 1);
        let composer_root = chat_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, composer_root);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Login);
        app.update();

        assert_eq!(chat_shell_root_count(app.world_mut()), 0);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Chat);
        app.update();

        assert_eq!(chat_shell_root_count(app.world_mut()), 1);
        let second_root = chat_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut().resource_mut::<SessionState>().disconnect();
        app.update();

        assert_eq!(chat_shell_root_count(app.world_mut()), 0);
    }
}
