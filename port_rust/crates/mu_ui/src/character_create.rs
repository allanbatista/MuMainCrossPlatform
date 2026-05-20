use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_gameplay::{base_class_attributes, CharacterClass, ClassAttributes, MAX_USERNAME_SIZE};

const CHARACTER_CREATE_TITLE: &str = "Character Create";
const CHARACTER_CREATE_PROMPT: &str = "Enter the new character name.";
const READY_NOTICE: &str = "Choose a class and enter a name.";
const SUBMITTING_NOTICE: &str = "Creating character...";
const INVALID_NAME_NOTICE: &str = "Name must be at least 4 characters.";
const ERROR_NOTICE: &str = "Character creation is unavailable.";

const CHARACTER_CREATE_NAME_MIN_LENGTH: usize = 4;

const CHARACTER_CLASSES: [CharacterClass; 7] = [
    CharacterClass::Knight,
    CharacterClass::Wizard,
    CharacterClass::Elf,
    CharacterClass::MagicGladiator,
    CharacterClass::DarkLord,
    CharacterClass::Summoner,
    CharacterClass::RageFighter,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterCreateScreenState {
    Ready,
    Submitting,
    InvalidName,
    Error,
}

impl CharacterCreateScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Submitting => "submitting",
            Self::InvalidName => "invalid-name",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Ready => Some(READY_NOTICE),
            Self::Submitting => Some(SUBMITTING_NOTICE),
            Self::InvalidName => Some(INVALID_NAME_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterCreateAction {
    Create,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterCreateButton {
    pub action: CharacterCreateAction,
    pub enabled: bool,
}

impl CharacterCreateButton {
    pub const fn new(action: CharacterCreateAction, enabled: bool) -> Self {
        Self { action, enabled }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterCreateClassEntry {
    pub class_: CharacterClass,
    pub selected: bool,
    pub enabled: bool,
    pub stats: ClassAttributes,
    pub summary: &'static str,
}

impl CharacterCreateClassEntry {
    pub const fn new(
        class_: CharacterClass,
        selected: bool,
        enabled: bool,
        stats: ClassAttributes,
        summary: &'static str,
    ) -> Self {
        Self {
            class_,
            selected,
            enabled,
            stats,
            summary,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterCreateScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: CharacterCreateScreenState,
    pub title: &'static str,
    pub prompt: &'static str,
    pub notice: Option<&'static str>,
    pub name: &'static str,
    pub name_min_length: usize,
    pub name_max_length: usize,
    pub selected_class: CharacterClass,
    pub classes: [CharacterCreateClassEntry; 7],
    pub buttons: [CharacterCreateButton; 2],
}

pub fn character_create_screen(state: CharacterCreateScreenState) -> CharacterCreateScreen {
    let selected_class = CharacterClass::Knight;
    let classes_enabled = state != CharacterCreateScreenState::Submitting;
    let buttons = match state {
        CharacterCreateScreenState::Ready => [
            CharacterCreateButton::new(CharacterCreateAction::Create, true),
            CharacterCreateButton::new(CharacterCreateAction::Cancel, true),
        ],
        CharacterCreateScreenState::Submitting => [
            CharacterCreateButton::new(CharacterCreateAction::Create, false),
            CharacterCreateButton::new(CharacterCreateAction::Cancel, false),
        ],
        CharacterCreateScreenState::InvalidName => [
            CharacterCreateButton::new(CharacterCreateAction::Create, false),
            CharacterCreateButton::new(CharacterCreateAction::Cancel, true),
        ],
        CharacterCreateScreenState::Error => [
            CharacterCreateButton::new(CharacterCreateAction::Create, false),
            CharacterCreateButton::new(CharacterCreateAction::Cancel, true),
        ],
    };

    let classes = std::array::from_fn(|index| {
        let class_ = CHARACTER_CLASSES[index];

        CharacterCreateClassEntry::new(
            class_,
            class_ == selected_class,
            classes_enabled,
            base_class_attributes(class_),
            class_summary(class_),
        )
    });

    CharacterCreateScreen {
        route: UiRoute::CharacterCreate,
        layout: UiShellLayout::for_route(UiRoute::CharacterCreate),
        widgets: UiShellWidgetSet::for_route(UiRoute::CharacterCreate),
        state,
        title: CHARACTER_CREATE_TITLE,
        prompt: CHARACTER_CREATE_PROMPT,
        notice: state.notice(),
        name: "",
        name_min_length: CHARACTER_CREATE_NAME_MIN_LENGTH,
        name_max_length: MAX_USERNAME_SIZE,
        selected_class,
        classes,
        buttons,
    }
}

impl CharacterCreateScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|prompt={}|notice={:?}|name={:?}|name_min_length={}|name_max_length={}|selected_class={:?}|classes={:?}|buttons={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.prompt,
            self.notice,
            self.name,
            self.name_min_length,
            self.name_max_length,
            self.selected_class,
            self.classes,
            self.buttons,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

fn class_summary(class_: CharacterClass) -> &'static str {
    match class_ {
        CharacterClass::Knight | CharacterClass::BladeKnight | CharacterClass::BladeMaster => {
            "Front-line fighter."
        }
        CharacterClass::Wizard | CharacterClass::SoulMaster | CharacterClass::GrandMaster => {
            "Arcane burst damage."
        }
        CharacterClass::Elf | CharacterClass::MuseElf | CharacterClass::HighElf => {
            "Flexible ranged support."
        }
        CharacterClass::MagicGladiator | CharacterClass::DuelMaster => {
            "Hybrid melee caster."
        }
        CharacterClass::DarkLord | CharacterClass::LordEmperor => "Commanding dark warrior.",
        CharacterClass::Summoner
        | CharacterClass::BloodySummoner
        | CharacterClass::DimensionMaster => "Debuff-heavy magic user.",
        CharacterClass::RageFighter | CharacterClass::TempleKnight => {
            "Fast close-range striker."
        }
        CharacterClass::Undefined => "Unknown class.",
    }
}

#[cfg(test)]
fn class_stats_label(stats: ClassAttributes) -> String {
    format!(
        "STR {} | AGI {} | VIT {} | ENG {} | HP {} | MP {} | SH {} | LvHP {} | LvMP {} | V->HP {} | E->MP {}",
        stats.strength,
        stats.dexterity,
        stats.vitality,
        stats.energy,
        stats.life,
        stats.mana,
        stats.shield,
        stats.level_life,
        stats.level_mana,
        stats.vitality_to_life,
        stats.energy_to_mana,
    )
}

#[cfg(test)]
fn class_label(entry: &CharacterCreateClassEntry) -> String {
    let mut label = format!(
        "{} | {} | {}",
        entry.class_.display_name(),
        entry.summary,
        class_stats_label(entry.stats)
    );

    if entry.selected {
        label.push_str(" | selected");
    }

    if !entry.enabled {
        label.push_str(" | disabled");
    }

    label
}

#[cfg(test)]
mod tests {
    use super::{
        character_create_screen, class_label, CharacterCreateAction, CharacterCreateButton,
        CharacterCreateScreenState,
    };
    use crate::UiRoute;
    use mu_gameplay::{base_class_attributes, CharacterClass, MAX_USERNAME_SIZE};

    #[test]
    fn ready_snapshot_keeps_the_legacy_class_order_and_limits() {
        let screen = character_create_screen(CharacterCreateScreenState::Ready);

        assert_eq!(screen.route, UiRoute::CharacterCreate);
        assert_eq!(screen.title, "Character Create");
        assert_eq!(screen.prompt, "Enter the new character name.");
        assert_eq!(screen.notice, Some("Choose a class and enter a name."));
        assert_eq!(screen.name, "");
        assert_eq!(screen.name_min_length, 4);
        assert_eq!(screen.name_max_length, MAX_USERNAME_SIZE);
        assert_eq!(screen.selected_class, CharacterClass::Knight);
        assert_eq!(screen.classes[0].class_, CharacterClass::Knight);
        assert_eq!(screen.classes[1].class_, CharacterClass::Wizard);
        assert_eq!(screen.classes[2].class_, CharacterClass::Elf);
        assert_eq!(screen.classes[3].class_, CharacterClass::MagicGladiator);
        assert_eq!(screen.classes[4].class_, CharacterClass::DarkLord);
        assert_eq!(screen.classes[5].class_, CharacterClass::Summoner);
        assert_eq!(screen.classes[6].class_, CharacterClass::RageFighter);
        assert_eq!(
            screen.classes[0].stats,
            base_class_attributes(CharacterClass::Knight)
        );
        assert!(screen.classes[0].selected);
        assert!(screen.classes.iter().skip(1).all(|entry| !entry.selected));
        assert!(screen.classes.iter().all(|entry| entry.enabled));
        assert_eq!(
            screen.buttons,
            [
                CharacterCreateButton::new(CharacterCreateAction::Create, true),
                CharacterCreateButton::new(CharacterCreateAction::Cancel, true),
            ]
        );

        let snapshot = screen.snapshot();
        assert!(snapshot.contains("route=character-create|group=character|state=ready"));
        assert!(snapshot.contains(r#"notice=Some("Choose a class and enter a name.")"#));
        assert!(snapshot.contains("selected_class=Knight"));
        assert!(snapshot.contains("classes=[CharacterCreateClassEntry { class_: Knight"));
        assert!(snapshot.contains("buttons=[CharacterCreateButton { action: Create, enabled: true }, CharacterCreateButton { action: Cancel, enabled: true }]"));
    }

    #[test]
    fn progress_and_error_states_adjust_notices_and_actions() {
        let submitting = character_create_screen(CharacterCreateScreenState::Submitting);
        assert_eq!(submitting.notice, Some("Creating character..."));
        assert!(submitting.classes.iter().all(|entry| !entry.enabled));
        assert_eq!(
            submitting.buttons,
            [
                CharacterCreateButton::new(CharacterCreateAction::Create, false),
                CharacterCreateButton::new(CharacterCreateAction::Cancel, false),
            ]
        );

        let invalid_name = character_create_screen(CharacterCreateScreenState::InvalidName);
        assert_eq!(
            invalid_name.notice,
            Some("Name must be at least 4 characters.")
        );
        assert!(invalid_name.classes.iter().all(|entry| entry.enabled));
        assert_eq!(
            invalid_name.buttons,
            [
                CharacterCreateButton::new(CharacterCreateAction::Create, false),
                CharacterCreateButton::new(CharacterCreateAction::Cancel, true),
            ]
        );

        let error = character_create_screen(CharacterCreateScreenState::Error);
        assert_eq!(error.notice, Some("Character creation is unavailable."));
        assert!(error.snapshot().contains("state=error"));
        assert!(error.snapshot().contains("buttons=[CharacterCreateButton { action: Create, enabled: false }, CharacterCreateButton { action: Cancel, enabled: true }]"));
    }

    #[test]
    fn class_label_includes_stats_and_selection_state() {
        let screen = character_create_screen(CharacterCreateScreenState::Ready);
        let label = class_label(&screen.classes[0]);

        assert!(label.contains("Dark Knight"));
        assert!(label.contains("Front-line fighter."));
        assert!(label.contains("selected"));
        assert!(label.contains("STR"));
        assert!(label.contains("AGI"));
    }
}
