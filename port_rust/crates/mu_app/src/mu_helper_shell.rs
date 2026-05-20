use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::MuHelperRuntime;
use mu_ui::{mu_helper_screen, MuHelperScreen, MuHelperScreenState, UiRoute, UiShellState};

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
const INACTIVE_ACCENT: Color = Color::srgb(0.34, 0.69, 0.84);
const ACTIVE_ACCENT: Color = Color::srgb(0.48, 0.82, 0.58);
const INVALID_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);
const LIMITED_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);
const SERVER_LIMITED_ACCENT: Color = Color::srgb(0.58, 0.62, 0.68);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct MuHelperShellPlugin;

#[derive(Debug, Clone, PartialEq)]
struct MuHelperShellKey {
    route: UiRoute,
    phase: SessionPhase,
    runtime: MuHelperRuntime,
}

#[derive(Debug, Default, Resource)]
struct MuHelperShellState {
    root: Option<Entity>,
    key: Option<MuHelperShellKey>,
}

#[derive(Debug, Clone)]
struct MuHelperShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct MuHelperShellRoot;

impl Plugin for MuHelperShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MuHelperShellState>()
            .add_systems(PostUpdate, sync_mu_helper_shell_system);
    }
}

fn sync_mu_helper_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    runtime: Res<MuHelperRuntime>,
    mut state: ResMut<MuHelperShellState>,
) {
    let current = mu_helper_shell_key(ui_shell.current(), session_state.phase(), &runtime);

    let Some(key) = current else {
        clear_mu_helper_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_mu_helper_shell(&mut commands, &mut state);

    let Some(view) = mu_helper_shell_view(key.route, key.phase, &key.runtime) else {
        return;
    };

    let root = spawn_mu_helper_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn mu_helper_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    runtime: &MuHelperRuntime,
) -> Option<MuHelperShellKey> {
    if !mu_helper_shell_visible(route, phase) {
        return None;
    }

    Some(MuHelperShellKey {
        route,
        phase,
        runtime: runtime.clone(),
    })
}

fn mu_helper_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::MuHelper && phase != SessionPhase::Disconnected
}

fn mu_helper_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    runtime: &MuHelperRuntime,
) -> Option<MuHelperShellView> {
    if !mu_helper_shell_visible(route, phase) {
        return None;
    }

    let screen = mu_helper_screen(runtime);

    Some(MuHelperShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: mu_helper_shell_body(&screen, phase),
        accent: accent_for_state(screen.state),
    })
}

fn mu_helper_shell_body(screen: &MuHelperScreen, phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "MU Helper shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "State",
        &format!(
            "state={} | total_cost={}",
            screen.state.as_str(),
            screen.total_cost,
        ),
    );
    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }
    if let Some(detail) = screen.status_detail.as_deref() {
        push_paragraph(&mut body, "Detail", detail);
    }
    push_paragraph(
        &mut body,
        "Config",
        &format!(
            "hunting_range={} | obtaining_range={} | max_seconds_away={} | extra_items={}",
            screen.config.hunting_range,
            screen.config.obtaining_range,
            screen.config.max_seconds_away,
            screen.config.extra_items,
        ),
    );
    push_paragraph(
        &mut body,
        "Skills",
        &format!(
            "basic_skill_id={} | activation_skill_ids={:?} | buff_skill_ids={:?} | use_combo={} | buff_cast_interval={}",
            screen.config.basic_skill_id,
            screen.config.activation_skill_ids,
            screen.config.buff_skill_ids,
            screen.config.use_combo,
            screen.config.buff_cast_interval,
        ),
    );
    push_paragraph(
        &mut body,
        "Recovery",
        &format!(
            "potion_threshold={} | heal_threshold={} | heal_party_threshold={} | use_heal_potion={} | use_drain_life={}",
            screen.config.potion_threshold,
            screen.config.heal_threshold,
            screen.config.heal_party_threshold,
            screen.config.use_heal_potion,
            screen.config.use_drain_life,
        ),
    );
    push_paragraph(
        &mut body,
        "Flags",
        &format!(
            "use_dark_raven={} | dark_raven_mode={:?} | repair_item={} | pick_extra_items={}",
            screen.config.use_dark_raven,
            screen.config.dark_raven_mode,
            screen.config.repair_item,
            screen.config.pick_extra_items,
        ),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn accent_for_state(state: MuHelperScreenState) -> Color {
    match state {
        MuHelperScreenState::Inactive => INACTIVE_ACCENT,
        MuHelperScreenState::Active => ACTIVE_ACCENT,
        MuHelperScreenState::InvalidConfig => INVALID_ACCENT,
        MuHelperScreenState::ResourceLimited => LIMITED_ACCENT,
        MuHelperScreenState::ServerLimited => SERVER_LIMITED_ACCENT,
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

fn spawn_mu_helper_shell(commands: &mut Commands, view: &MuHelperShellView) -> Entity {
    let root = commands
        .spawn((
            MuHelperShellRoot,
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

fn clear_mu_helper_shell(commands: &mut Commands, state: &mut MuHelperShellState) {
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
    use super::{mu_helper_shell_view, MuHelperShellPlugin, MuHelperShellRoot};
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::{MuHelperConfig, MuHelperRuntime};
    use mu_ui::{UiRoute, UiShellState};

    fn mu_helper_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&MuHelperShellRoot>();
        query.iter(world).count()
    }

    fn mu_helper_shell_root_entity(
        world: &mut bevy::prelude::World,
    ) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<MuHelperShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn mu_helper_shell_view_renders_expected_bodies() {
        let runtime = MuHelperRuntime::new();
        let inactive = mu_helper_shell_view(UiRoute::MuHelper, SessionPhase::LoggedIn, &runtime)
            .expect("mu helper inactive view missing");

        assert_eq!(inactive.title, "MU Helper");
        assert!(inactive.status.contains("route=mu-helper | group=gameplay"));
        assert!(inactive.body.contains("MU Helper shell is active."));
        assert!(inactive.body.contains("state=inactive | total_cost=0"));
        assert!(inactive.body.contains("Actions"));
        assert_eq!(inactive.accent, super::INACTIVE_ACCENT);

        let mut runtime = MuHelperRuntime::new();
        let error = runtime
            .load_config(MuHelperConfig {
                hunting_range: 7,
                ..MuHelperConfig::default()
            })
            .expect_err("invalid helper config should fail");
        let invalid = mu_helper_shell_view(UiRoute::MuHelper, SessionPhase::LoggedIn, &runtime)
            .expect("mu helper invalid view missing");

        assert_eq!(invalid.accent, super::INVALID_ACCENT);
        assert!(invalid.body.contains("state=invalid-config"));
        assert!(invalid.body.contains(&error.to_string()));
        assert!(invalid.body.contains("Detail"));

        assert!(
            mu_helper_shell_view(UiRoute::MuHelper, SessionPhase::Disconnected, &runtime).is_none()
        );
        assert!(mu_helper_shell_view(UiRoute::World, SessionPhase::LoggedIn, &runtime).is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, MuHelperShellPlugin));

        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::MuHelper);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);
        app.insert_resource(MuHelperRuntime::new());

        app.update();

        assert_eq!(mu_helper_shell_root_count(app.world_mut()), 1);
        assert!(mu_helper_shell_root_entity(app.world_mut()).is_some());

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(mu_helper_shell_root_count(app.world_mut()), 0);
    }
}
