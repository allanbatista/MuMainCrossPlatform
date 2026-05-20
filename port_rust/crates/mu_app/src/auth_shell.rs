use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_ui::{
    character_create_screen, character_select_screen, login_screen, options_screen,
    server_select_screen, CharacterCreateAction, CharacterCreateButton, CharacterCreateClassEntry,
    CharacterCreateScreen, CharacterCreateScreenState, CharacterSelectAction,
    CharacterSelectButton, CharacterSelectCharacter, CharacterSelectScreen,
    CharacterSelectScreenState, LoginAction, LoginField, LoginScreen, LoginScreenState,
    OptionsScreen, OptionsScreenState, OptionsSection, OptionsToggle, ServerEntry,
    ServerSelectAction, ServerSelectScreen, ServerSelectScreenState, UiRoute, UiRouteGroup,
    UiShellState,
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
const STARTUP_ACCENT: Color = Color::srgb(0.98, 0.74, 0.36);
const AUTH_ACCENT: Color = Color::srgb(0.34, 0.69, 0.98);
const CHARACTER_ACCENT: Color = Color::srgb(0.36, 0.84, 0.60);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct AuthShellPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AuthShellKey {
    route: UiRoute,
    phase: SessionPhase,
}

#[derive(Debug, Default, Resource)]
struct AuthShellState {
    root: Option<Entity>,
    key: Option<AuthShellKey>,
}

#[derive(Debug, Clone)]
struct AuthShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct AuthShellRoot;

impl Plugin for AuthShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AuthShellState>()
            .add_systems(PostUpdate, sync_auth_shell_system);
    }
}

fn sync_auth_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    bootstrap: Option<Res<BootstrapRuntime>>,
    mut state: ResMut<AuthShellState>,
) {
    let current = auth_shell_key(ui_shell.current(), session_state.phase());

    let Some(key) = current else {
        clear_auth_shell(&mut commands, &mut state);
        return;
    };

    if state.key == Some(key) && state.root.is_some() {
        return;
    }

    clear_auth_shell(&mut commands, &mut state);

    let Some(view) = auth_shell_view(key.route, key.phase, bootstrap.as_deref()) else {
        return;
    };

    let root = spawn_auth_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn auth_shell_key(route: UiRoute, phase: SessionPhase) -> Option<AuthShellKey> {
    if auth_shell_visible(route) {
        Some(AuthShellKey { route, phase })
    } else {
        None
    }
}

fn auth_shell_visible(route: UiRoute) -> bool {
    matches!(
        route,
        UiRoute::Boot
            | UiRoute::Loading
            | UiRoute::Login
            | UiRoute::ServerSelect
            | UiRoute::Options
            | UiRoute::CharacterSelect
            | UiRoute::CharacterCreate
            | UiRoute::Error
    )
}

fn auth_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    bootstrap: Option<&BootstrapRuntime>,
) -> Option<AuthShellView> {
    match route {
        UiRoute::Boot => Some(boot_view(phase)),
        UiRoute::Loading => Some(loading_view(phase)),
        UiRoute::Login => Some(login_view(phase)),
        UiRoute::ServerSelect => Some(server_select_view(phase)),
        UiRoute::Options => Some(options_view(phase)),
        UiRoute::CharacterSelect => Some(character_select_view(phase, bootstrap)),
        UiRoute::CharacterCreate => Some(character_create_view(phase)),
        UiRoute::Error => Some(error_view(phase)),
        _ => None,
    }
}

fn boot_view(phase: SessionPhase) -> AuthShellView {
    AuthShellView {
        title: "Boot",
        status: status_line(UiRoute::Boot, phase),
        body: boot_body(phase),
        accent: accent_for_route(UiRoute::Boot),
    }
}

fn loading_view(phase: SessionPhase) -> AuthShellView {
    AuthShellView {
        title: "Loading",
        status: status_line(UiRoute::Loading, phase),
        body: loading_body(phase),
        accent: accent_for_route(UiRoute::Loading),
    }
}

fn login_view(phase: SessionPhase) -> AuthShellView {
    let screen = login_screen(if phase == SessionPhase::Disconnected {
        LoginScreenState::ServerUnavailable
    } else {
        LoginScreenState::Idle
    });

    AuthShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: login_body(&screen),
        accent: accent_for_route(screen.route),
    }
}

fn server_select_view(phase: SessionPhase) -> AuthShellView {
    let screen = server_select_screen(if phase == SessionPhase::Disconnected {
        ServerSelectScreenState::Error
    } else {
        ServerSelectScreenState::Ready
    });

    AuthShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: server_select_body(&screen),
        accent: accent_for_route(screen.route),
    }
}

fn options_view(phase: SessionPhase) -> AuthShellView {
    let screen = options_screen(OptionsScreenState::Ready);

    AuthShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: options_body(&screen),
        accent: accent_for_route(screen.route),
    }
}

fn character_select_view(
    phase: SessionPhase,
    bootstrap: Option<&BootstrapRuntime>,
) -> AuthShellView {
    let loading_screen = character_select_screen(CharacterSelectScreenState::Loading);
    let ready_screen = character_select_screen(CharacterSelectScreenState::Ready);

    let selected_index = if phase == SessionPhase::Disconnected {
        None
    } else if bootstrap.map_or(true, BootstrapRuntime::character_list_ready) {
        character_select_selected_index(bootstrap, &ready_screen)
    } else {
        None
    };

    let screen = if phase == SessionPhase::Disconnected {
        character_select_screen(CharacterSelectScreenState::Error)
    } else if !bootstrap.map_or(true, BootstrapRuntime::character_list_ready) {
        loading_screen
    } else if character_select_selected_entry(&ready_screen, selected_index)
        .map(|entry| entry.item_blocked)
        .unwrap_or(false)
    {
        character_select_screen(CharacterSelectScreenState::ActionDenied)
    } else {
        ready_screen
    };

    AuthShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: character_select_body(&screen, selected_index),
        accent: accent_for_route(screen.route),
    }
}

fn character_create_view(phase: SessionPhase) -> AuthShellView {
    let screen = if phase == SessionPhase::Disconnected {
        character_create_screen(CharacterCreateScreenState::Error)
    } else {
        character_create_screen(CharacterCreateScreenState::Ready)
    };

    AuthShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: character_create_body(&screen),
        accent: accent_for_route(screen.route),
    }
}

fn error_view(phase: SessionPhase) -> AuthShellView {
    AuthShellView {
        title: "Error",
        status: status_line(UiRoute::Error, phase),
        body: error_body(phase),
        accent: accent_for_route(UiRoute::Error),
    }
}

fn boot_body(phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Preparing the graphical client.");
    push_paragraph(&mut body, "Session", phase.as_str());

    body
}

fn loading_body(phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Loading the next bootstrap step.");
    push_paragraph(&mut body, "Session", phase.as_str());

    body
}

fn error_body(phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "A safe error surface is active.");
    push_paragraph(&mut body, "Session", phase.as_str());

    body
}

fn login_body(screen: &LoginScreen) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Prompt", screen.prompt);

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Fields",
        screen.fields.iter().copied().map(login_field_label),
        None,
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().copied().map(login_action_label),
        None,
    );

    body
}

fn server_select_body(screen: &ServerSelectScreen) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Prompt", "Choose a server to continue.");

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Servers",
        screen.servers.iter().map(server_entry_label),
        Some("No servers are available."),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().copied().map(server_action_label),
        None,
    );

    body
}

fn character_select_body(screen: &CharacterSelectScreen, selected_index: Option<usize>) -> String {
    let mut body = String::new();

    push_paragraph(
        &mut body,
        "Prompt",
        "Choose a character to enter the world.",
    );

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    if let Some(account_block_notice) = screen.account_block_notice {
        push_paragraph(&mut body, "Account Block", account_block_notice);
    }

    if let Some(empty_slots) = screen.empty_slots {
        push_paragraph(
            &mut body,
            "Roster",
            &format!("Empty slots available: {empty_slots}"),
        );
    }

    if let Some(selected_character) = character_select_selected_entry(screen, selected_index) {
        push_paragraph(
            &mut body,
            "Selection",
            &format!(
                "Selected slot: {} | {}",
                selected_character.slot_index, selected_character.name
            ),
        );
        push_paragraph(
            &mut body,
            "Controls",
            "Use Up/Down or Left/Right to change selection. Press Enter to continue.",
        );
    } else if screen.state == CharacterSelectScreenState::Loading {
        push_paragraph(&mut body, "Selection", "Waiting for the character list.");
    }

    push_lines(
        &mut body,
        "Characters",
        screen
            .characters
            .iter()
            .enumerate()
            .map(|(index, entry)| character_entry_label(entry, selected_index == Some(index))),
        Some("No characters are available."),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.buttons.iter().map(button_label),
        None,
    );

    body
}

fn character_create_body(screen: &CharacterCreateScreen) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Prompt", screen.prompt);

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_paragraph(
        &mut body,
        "Name",
        &format!(
            "{} | length {}-{}",
            if screen.name.is_empty() {
                "<empty>"
            } else {
                screen.name
            },
            screen.name_min_length,
            screen.name_max_length
        ),
    );

    if let Some(selected_class) = character_create_selected_entry(screen) {
        push_paragraph(
            &mut body,
            "Selection",
            &format!(
                "Selected class: {} | {}",
                selected_class.class_.display_name(),
                selected_class.summary
            ),
        );
        push_paragraph(
            &mut body,
            "Stats",
            &character_create_stats_label(selected_class),
        );
    }

    push_lines(
        &mut body,
        "Classes",
        screen.classes.iter().map(character_create_class_label),
        None,
    );
    push_lines(
        &mut body,
        "Actions",
        screen.buttons.iter().map(character_create_button_label),
        None,
    );

    body
}

fn options_body(screen: &OptionsScreen) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Overview", "Review the shared client settings.");

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Sections",
        screen.sections.iter().copied().map(options_section_label),
        None,
    );
    push_lines(
        &mut body,
        "Toggles",
        screen.toggles.iter().copied().map(options_toggle_label),
        None,
    );

    body
}

fn character_select_selected_index(
    bootstrap: Option<&BootstrapRuntime>,
    screen: &CharacterSelectScreen,
) -> Option<usize> {
    let from_bootstrap = bootstrap
        .and_then(BootstrapRuntime::character_select_index)
        .filter(|index| *index < screen.characters.len());

    from_bootstrap.or_else(|| {
        screen
            .characters
            .iter()
            .position(character_select_entry_is_selected)
    })
}

fn character_select_selected_entry<'a>(
    screen: &'a CharacterSelectScreen,
    selected_index: Option<usize>,
) -> Option<&'a CharacterSelectCharacter> {
    selected_index.and_then(|index| screen.characters.get(index))
}

fn character_create_selected_entry(
    screen: &CharacterCreateScreen,
) -> Option<&CharacterCreateClassEntry> {
    screen.classes.iter().find(|entry| entry.selected)
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

fn login_field_label(field: LoginField) -> &'static str {
    match field {
        LoginField::Username => "Username",
        LoginField::Password => "Password",
        LoginField::RememberUsername => "Remember username",
    }
}

fn login_action_label(action: LoginAction) -> &'static str {
    match action {
        LoginAction::Connect => "Connect",
        LoginAction::Options => "Options",
        LoginAction::Exit => "Exit",
    }
}

fn server_action_label(action: ServerSelectAction) -> &'static str {
    match action {
        ServerSelectAction::Refresh => "Refresh",
        ServerSelectAction::Connect => "Connect",
        ServerSelectAction::Back => "Back",
    }
}

fn options_section_label(section: OptionsSection) -> &'static str {
    match section {
        OptionsSection::Video => "Video",
        OptionsSection::Audio => "Audio",
        OptionsSection::Controls => "Controls",
        OptionsSection::Performance => "Performance",
        OptionsSection::Network => "Network",
        OptionsSection::Locale => "Locale",
    }
}

fn options_toggle_label(toggle: OptionsToggle) -> &'static str {
    match toggle {
        OptionsToggle::Fullscreen => "Fullscreen",
        OptionsToggle::VSync => "VSync",
        OptionsToggle::ReduceEffects => "Reduce effects",
        OptionsToggle::MuteAudio => "Mute audio",
        OptionsToggle::RememberServer => "Remember server",
        OptionsToggle::RememberUsername => "Remember username",
    }
}

fn character_action_label(action: CharacterSelectAction) -> &'static str {
    match action {
        CharacterSelectAction::Menu => "Menu",
        CharacterSelectAction::Create => "Create",
        CharacterSelectAction::Connect => "Connect",
        CharacterSelectAction::Delete => "Delete",
    }
}

fn server_entry_label(entry: &ServerEntry) -> String {
    match entry.players {
        Some(players) => format!("{} | {} | {players} players", entry.name, entry.status),
        None => format!("{} | {}", entry.name, entry.status),
    }
}

fn character_entry_label(entry: &CharacterSelectCharacter, selected: bool) -> String {
    let mut label = format!(
        "Slot {} | {} | Lv {} | {} | {}",
        entry.slot_index, entry.name, entry.level, entry.class_name, entry.guild_label
    );

    if selected || entry.selected {
        label.push_str(" | selected");
    }

    if entry.item_blocked {
        label.push_str(" | item blocked");
    }

    label
}

fn character_select_entry_is_selected(entry: &CharacterSelectCharacter) -> bool {
    entry.selected
}

fn button_label(button: &CharacterSelectButton) -> String {
    let state = if button.enabled {
        "enabled"
    } else {
        "disabled"
    };
    format!("{} ({state})", character_action_label(button.action))
}

fn character_create_stats_label(entry: &CharacterCreateClassEntry) -> String {
    format!(
        "STR {} | AGI {} | VIT {} | ENG {} | HP {} | MP {} | SH {} | LvHP {} | LvMP {} | V->HP {} | E->MP {}",
        entry.stats.strength,
        entry.stats.dexterity,
        entry.stats.vitality,
        entry.stats.energy,
        entry.stats.life,
        entry.stats.mana,
        entry.stats.shield,
        entry.stats.level_life,
        entry.stats.level_mana,
        entry.stats.vitality_to_life,
        entry.stats.energy_to_mana,
    )
}

fn character_create_class_label(entry: &CharacterCreateClassEntry) -> String {
    let mut label = format!(
        "{} | {} | {}",
        entry.class_.display_name(),
        entry.summary,
        character_create_stats_label(entry)
    );

    if entry.selected {
        label.push_str(" | selected");
    }

    if !entry.enabled {
        label.push_str(" | disabled");
    }

    label
}

fn character_create_action_label(action: CharacterCreateAction) -> &'static str {
    match action {
        CharacterCreateAction::Create => "Create",
        CharacterCreateAction::Cancel => "Cancel",
    }
}

fn character_create_button_label(button: &CharacterCreateButton) -> String {
    let state = if button.enabled {
        "enabled"
    } else {
        "disabled"
    };

    format!("{} ({state})", character_create_action_label(button.action))
}

fn status_line(route: UiRoute, phase: SessionPhase) -> String {
    format!(
        "route={} | group={} | session={}",
        route.slug(),
        route.group().as_str(),
        phase.as_str()
    )
}

fn accent_for_route(route: UiRoute) -> Color {
    match route.group() {
        UiRouteGroup::Startup => STARTUP_ACCENT,
        UiRouteGroup::Authentication => AUTH_ACCENT,
        UiRouteGroup::Character => CHARACTER_ACCENT,
        UiRouteGroup::Error => ERROR_ACCENT,
        UiRouteGroup::World | UiRouteGroup::Gameplay | UiRouteGroup::Admin => AUTH_ACCENT,
    }
}

fn spawn_auth_shell(commands: &mut Commands, view: &AuthShellView) -> Entity {
    let root = commands
        .spawn((
            AuthShellRoot,
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

fn clear_auth_shell(commands: &mut Commands, state: &mut AuthShellState) {
    if let Some(root) = state.root.take() {
        commands.entity(root).despawn();
    }

    state.key = None;
}

#[cfg(test)]
mod tests {
    use super::{
        auth_shell_view, character_action_label, character_entry_label, login_body,
        login_field_label, login_view, server_action_label, server_entry_label, server_select_view,
        AuthShellPlugin, AuthShellRoot, AuthShellState,
    };
    use crate::bootstrap_runtime::BootstrapRuntime;
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_ui::{
        login_screen, CharacterSelectAction, CharacterSelectButton, CharacterSelectCharacter,
        LoginField, LoginScreenState, ServerEntry, ServerSelectAction, UiRoute, UiShellState,
    };

    #[test]
    fn login_shell_uses_the_server_unavailable_state_when_disconnected() {
        let view = login_view(SessionPhase::Disconnected);

        assert_eq!(view.title, "Login");
        assert!(view.status.contains("route=login"));
        assert!(view.body.contains("The selected server is unavailable."));
    }

    #[test]
    fn login_shell_renders_fields_and_actions() {
        let screen = login_screen(LoginScreenState::Idle);
        let body = login_body(&screen);

        assert!(body.contains("Prompt"));
        assert!(body.contains("Username"));
        assert!(body.contains("Password"));
        assert!(body.contains("Remember username"));
        assert!(body.contains("Connect"));
    }

    #[test]
    fn server_select_shell_lists_servers_and_actions() {
        let view = server_select_view(SessionPhase::ReadyForLogin);

        assert_eq!(view.title, "Server Select");
        assert!(view.body.contains("Alpha"));
        assert!(view.body.contains("Bravo"));
        assert!(view.body.contains("Refresh"));
    }

    #[test]
    fn options_shell_renders_sections_and_toggles() {
        let view = auth_shell_view(UiRoute::Options, SessionPhase::ReadyForLogin, None)
            .expect("options shell missing");

        assert_eq!(view.title, "Options");
        assert!(view.body.contains("Changes will be applied on save."));
        assert!(view.body.contains("Video"));
        assert!(view.body.contains("Remember server"));
    }

    #[test]
    fn character_select_shell_lists_characters_and_buttons() {
        let view = auth_shell_view(UiRoute::CharacterSelect, SessionPhase::LoggedIn, None)
            .expect("character select shell missing");

        assert!(view.body.contains("Astra"));
        assert!(view.body.contains("Selene"));
        assert!(view.body.contains("Create"));
        assert!(view.body.contains("selected"));
        assert!(view.body.contains("Controls"));
    }

    #[test]
    fn character_select_shell_waits_for_the_roster_before_showing_selection() {
        let mut bootstrap = BootstrapRuntime::idle();
        bootstrap.set_character_list_ready(false);

        let view = auth_shell_view(
            UiRoute::CharacterSelect,
            SessionPhase::LoggedIn,
            Some(&bootstrap),
        )
        .expect("character select shell missing");

        assert!(view.body.contains("Loading character list..."));
        assert!(view.body.contains("Waiting for the character list."));
    }

    #[test]
    fn character_select_shell_tracks_manual_selection() {
        let mut bootstrap = BootstrapRuntime::idle();
        bootstrap.set_character_list_ready(true);
        bootstrap.set_character_select_index(Some(1));

        let view = auth_shell_view(
            UiRoute::CharacterSelect,
            SessionPhase::LoggedIn,
            Some(&bootstrap),
        )
        .expect("character select shell missing");

        assert!(view.body.contains("Selected slot: 1 | Selene"));
        assert!(view.body.contains("The selected character cannot be used."));
    }

    #[test]
    fn character_create_shell_renders_class_list_and_actions() {
        let view = auth_shell_view(UiRoute::CharacterCreate, SessionPhase::LoggedIn, None)
            .expect("character create shell missing");

        assert_eq!(view.title, "Character Create");
        assert!(view.status.contains("route=character-create"));
        assert!(view.body.contains("Enter the new character name."));
        assert!(view.body.contains("Choose a class and enter a name."));
        assert!(view.body.contains("Selected class: Dark Knight"));
        assert!(view.body.contains("Front-line fighter."));
        assert!(view.body.contains("Wizard"));
        assert!(view.body.contains("Rage Fighter"));
        assert!(view.body.contains("STR"));
        assert!(view.body.contains("Create"));
        assert!(view.body.contains("Cancel"));
    }

    #[test]
    fn character_create_shell_uses_error_state_when_disconnected() {
        let view = auth_shell_view(UiRoute::CharacterCreate, SessionPhase::Disconnected, None)
            .expect("character create shell missing");

        assert!(view.status.contains("session=disconnected"));
        assert!(view.body.contains("Character creation is unavailable."));
    }

    #[test]
    fn format_helpers_keep_the_view_human_readable() {
        assert_eq!(
            login_field_label(LoginField::RememberUsername),
            "Remember username"
        );
        assert_eq!(login_field_label(LoginField::Password), "Password");
        assert_eq!(
            character_action_label(CharacterSelectAction::Create),
            "Create"
        );
        assert_eq!(server_action_label(ServerSelectAction::Refresh), "Refresh");

        let server = ServerEntry::new("Alpha", "Online", Some(248));
        assert_eq!(server_entry_label(&server), "Alpha | Online | 248 players");

        let character =
            CharacterSelectCharacter::new(1, "Selene", 297, "Fairy Elf", "Night Watch", true, true);
        let label = character_entry_label(&character, true);
        assert!(label.contains("Slot 1"));
        assert!(label.contains("item blocked"));
        assert!(label.contains("selected"));

        let button = CharacterSelectButton::new(CharacterSelectAction::Delete, false);
        assert_eq!(character_action_label(button.action), "Delete");
        assert!(!button.enabled);
    }

    #[test]
    fn non_auth_routes_do_not_render_the_shell() {
        assert!(auth_shell_view(UiRoute::World, SessionPhase::ReadyForLogin, None).is_none());
        assert!(auth_shell_view(UiRoute::Inventory, SessionPhase::ReadyForLogin, None).is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins(AuthShellPlugin);
        app.init_resource::<UiShellState>();
        app.insert_resource(SessionState::default());

        {
            let mut ui_shell = app.world_mut().resource_mut::<UiShellState>();
            ui_shell.set_route(UiRoute::Login);
        }

        app.update();

        let state = app.world().resource::<AuthShellState>();
        let root = state.root.expect("auth shell root missing");
        assert!(app.world().entity(root).contains::<AuthShellRoot>());

        {
            let mut ui_shell = app.world_mut().resource_mut::<UiShellState>();
            ui_shell.set_route(UiRoute::World);
        }

        app.update();

        let state = app.world().resource::<AuthShellState>();
        assert!(state.root.is_none());
    }
}
