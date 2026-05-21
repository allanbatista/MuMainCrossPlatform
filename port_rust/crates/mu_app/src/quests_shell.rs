use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{QuestManager, QuestMode};
use mu_ui::{quests_screen, QuestScreen, QuestScreenState, UiRoute, UiShellState};

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
const QUESTS_ACCENT: Color = Color::srgb(0.38, 0.66, 0.96);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct QuestsShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct QuestsShellKey {
    route: UiRoute,
    phase: SessionPhase,
    manager: QuestManager,
}

#[derive(Debug, Default, Resource)]
struct QuestsShellState {
    root: Option<Entity>,
    key: Option<QuestsShellKey>,
}

#[derive(Debug, Clone)]
struct QuestsShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct QuestsShellRoot;

impl Plugin for QuestsShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QuestsShellState>()
            .add_systems(PostUpdate, sync_quests_shell_system);
    }
}

fn sync_quests_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    quest_manager: Res<QuestManager>,
    mut state: ResMut<QuestsShellState>,
) {
    let current = quests_shell_key(
        ui_shell.current(),
        session_state.phase(),
        quest_manager.clone(),
    );

    let Some(key) = current else {
        clear_quests_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_quests_shell(&mut commands, &mut state);

    let Some(view) = quests_shell_view(key.route, key.phase, &key.manager) else {
        return;
    };

    let root = spawn_quests_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn quests_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    manager: QuestManager,
) -> Option<QuestsShellKey> {
    if quests_shell_visible(route, phase) {
        Some(QuestsShellKey {
            route,
            phase,
            manager,
        })
    } else {
        None
    }
}

fn quests_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Quests && phase != SessionPhase::Disconnected
}

fn quests_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    manager: &QuestManager,
) -> Option<QuestsShellView> {
    if !quests_shell_visible(route, phase) {
        return None;
    }

    let screen = quests_screen(quest_screen_state(manager.mode()));

    Some(QuestsShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: quests_shell_body(&screen, phase, manager),
        accent: accent_for_state(screen.state),
    })
}

fn quest_screen_state(mode: QuestMode) -> QuestScreenState {
    match mode {
        QuestMode::Journal => QuestScreenState::Journal,
        QuestMode::Dialogue => QuestScreenState::Dialogue,
        QuestMode::Reward => QuestScreenState::Reward,
        QuestMode::ByEtc => QuestScreenState::Etc,
        QuestMode::Error => QuestScreenState::Error,
    }
}

fn quests_shell_body(screen: &QuestScreen, phase: SessionPhase, manager: &QuestManager) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Quests shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_paragraph(
        &mut body,
        "Quest Mode",
        &format!(
            "mode={} | screen_state={}",
            quest_mode_label(manager.mode()),
            screen.state.as_str(),
        ),
    );
    push_paragraph(
        &mut body,
        "NPC",
        &format!(
            "npc_index={:?} | npc_name={}",
            manager.npc_index(),
            manager.npc_name()
        ),
    );
    push_paragraph(
        &mut body,
        "Selection",
        &format!("selected_quest_index={:?}", manager.selected_quest_index()),
    );
    push_lines(
        &mut body,
        "Current Quests",
        manager
            .current_quests()
            .iter()
            .map(|quest| quest.to_string()),
        Some("No current quests."),
    );
    push_lines(
        &mut body,
        "Etc Quests",
        manager.etc_quests().iter().map(|quest| quest.to_string()),
        Some("No etc quests."),
    );

    if let Some(dialogue) = manager.dialogue() {
        push_paragraph(
            &mut body,
            "Dialogue",
            &format!(
                "quest_index={} | npc_index={} | npc_name={} | page={}/{} | selected_answer={:?} | subject={}",
                dialogue.quest_index,
                dialogue.npc_index,
                dialogue.npc_name,
                dialogue.current_page,
                dialogue.max_page,
                dialogue.selected_answer,
                dialogue.subject,
            ),
        );
    }

    if let Some(reward) = manager.reward() {
        push_paragraph(
            &mut body,
            "Reward",
            &format!(
                "quest_index={} | visible={} | request_reward_text_lines={}",
                reward.quest_index,
                reward.visible,
                reward.request_reward_text.len(),
            ),
        );
    }

    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn accent_for_state(state: QuestScreenState) -> Color {
    match state {
        QuestScreenState::Error => ERROR_ACCENT,
        QuestScreenState::Journal
        | QuestScreenState::Dialogue
        | QuestScreenState::Reward
        | QuestScreenState::Etc => QUESTS_ACCENT,
    }
}

fn quest_mode_label(mode: QuestMode) -> &'static str {
    match mode {
        QuestMode::Journal => "journal",
        QuestMode::Dialogue => "dialogue",
        QuestMode::Reward => "reward",
        QuestMode::ByEtc => "etc",
        QuestMode::Error => "error",
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

fn spawn_quests_shell(commands: &mut Commands, view: &QuestsShellView) -> Entity {
    let root = commands
        .spawn((
            QuestsShellRoot,
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

fn clear_quests_shell(commands: &mut Commands, state: &mut QuestsShellState) {
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
    use super::{quests_shell_view, QuestsShellPlugin, QuestsShellRoot};
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::{QuestDialogueState, QuestManager, QuestPlugin, QuestRewardState};
    use mu_ui::{UiRoute, UiShellState};

    fn quests_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&QuestsShellRoot>();
        query.iter(world).count()
    }

    fn quests_shell_root_entity(world: &mut bevy::prelude::World) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<QuestsShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn quests_shell_view_renders_expected_bodies() {
        let mut manager = QuestManager::new();
        manager.set_current_npc(18, "Elpis");
        manager.set_current_quests([1001, 1002, 1008]);
        manager.set_etc_quests([2001, 2002]);
        manager.set_dialogue(QuestDialogueState {
            quest_index: 1002,
            npc_index: 18,
            npc_name: "Elpis".to_owned(),
            current_page: 1,
            max_page: 4,
            selected_answer: Some(2),
            subject: "Castle prep".to_owned(),
            summary: "Review the castle defense prep.".to_owned(),
            npc_words: vec!["Need a hand?".to_owned()],
            player_words: vec!["Ready.".to_owned()],
            answers: vec![1, 2, 3],
        });

        let view = quests_shell_view(UiRoute::Quests, SessionPhase::LoggedIn, &manager).unwrap();

        assert_eq!(view.title, "Quests");
        assert!(view.status.contains("route=quests | group=gameplay"));
        assert!(view.body.contains("Quests shell is active."));
        assert!(view.body.contains("mode=dialogue | screen_state=dialogue"));
        assert!(view.body.contains("npc_index=Some(18) | npc_name=Elpis"));
        assert!(view.body.contains("selected_quest_index=Some(1002)"));
        assert!(view.body.contains("1001"));
        assert!(view.body.contains("Dialogue"));
        assert!(view.body.contains("Actions"));

        manager.reset();
        manager.set_current_npc(18, "Elpis");
        manager.set_current_quests([1001, 1002, 1008]);
        manager.set_reward(QuestRewardState {
            quest_index: 1008,
            visible: true,
            request_reward_text: vec!["Reward me.".to_owned()],
        });
        manager.mark_by_etc();

        let reward_view =
            quests_shell_view(UiRoute::Quests, SessionPhase::LoggedIn, &manager).unwrap();

        assert!(reward_view.body.contains("mode=etc | screen_state=etc"));
        assert!(reward_view.body.contains("Reward"));
        assert!(reward_view.body.contains("request_reward_text_lines=1"));

        assert!(quests_shell_view(UiRoute::Quests, SessionPhase::Disconnected, &manager).is_none());
        assert!(quests_shell_view(UiRoute::World, SessionPhase::LoggedIn, &manager).is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, QuestsShellPlugin, QuestPlugin));

        let mut ui_shell = mu_ui::UiShellState::default();
        ui_shell.set_route(UiRoute::Quests);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        {
            let mut manager = app.world_mut().resource_mut::<QuestManager>();
            manager.set_current_npc(18, "Elpis");
            manager.set_current_quests([1001, 1002, 1008]);
            manager.set_etc_quests([2001, 2002]);
            manager.set_dialogue(QuestDialogueState {
                quest_index: 1002,
                npc_index: 18,
                npc_name: "Elpis".to_owned(),
                current_page: 1,
                max_page: 4,
                selected_answer: Some(2),
                subject: "Castle prep".to_owned(),
                summary: "Review the castle defense prep.".to_owned(),
                npc_words: vec!["Need a hand?".to_owned()],
                player_words: vec!["Ready.".to_owned()],
                answers: vec![1, 2, 3],
            });
        }

        app.update();

        assert_eq!(quests_shell_root_count(app.world_mut()), 1);
        let first_root = quests_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut().resource_mut::<QuestManager>().reset();
        app.world_mut()
            .resource_mut::<QuestManager>()
            .set_current_quests([1001, 1002, 1008]);
        app.world_mut().resource_mut::<QuestManager>().mark_by_etc();
        app.update();

        assert_eq!(quests_shell_root_count(app.world_mut()), 1);
        let second_root = quests_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(quests_shell_root_count(app.world_mut()), 0);
    }
}
