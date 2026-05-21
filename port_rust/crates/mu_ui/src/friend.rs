use bevy::app::{App, Plugin};

use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_gameplay::{MailLetterEntry, MailManager, MailMode};

const FRIEND_TITLE: &str = "Friend";
const ROSTER_NOTICE: &str = "Manage friends and chat invites.";
const INBOX_NOTICE: &str = "Review letters and reply.";
const COMPOSE_NOTICE: &str = "Write a new letter.";
const CHAT_ROOMS_NOTICE: &str = "Manage chat rooms.";
const ERROR_NOTICE: &str = "Friend sync failed.";

const ROSTER_ACTIONS: &[FriendAction] = &[
    FriendAction::AddFriend,
    FriendAction::DeleteFriend,
    FriendAction::Talk,
    FriendAction::WriteLetter,
    FriendAction::Close,
];
const INBOX_ACTIONS: &[FriendAction] = &[
    FriendAction::WriteLetter,
    FriendAction::ReadLetter,
    FriendAction::ReplyLetter,
    FriendAction::DeleteLetter,
    FriendAction::Refresh,
    FriendAction::Close,
];
const COMPOSE_ACTIONS: &[FriendAction] = &[
    FriendAction::SendLetter,
    FriendAction::Cancel,
    FriendAction::Close,
];
const CHAT_ROOM_ACTIONS: &[FriendAction] = &[FriendAction::HideAll, FriendAction::Close];
const ERROR_ACTIONS: &[FriendAction] = &[FriendAction::Retry, FriendAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendScreenState {
    Roster,
    Inbox,
    Compose,
    ChatRooms,
    Error,
}

impl FriendScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Roster => "roster",
            Self::Inbox => "inbox",
            Self::Compose => "compose",
            Self::ChatRooms => "chat-rooms",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Roster => Some(ROSTER_NOTICE),
            Self::Inbox => Some(INBOX_NOTICE),
            Self::Compose => Some(COMPOSE_NOTICE),
            Self::ChatRooms => Some(CHAT_ROOMS_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendAction {
    AddFriend,
    DeleteFriend,
    Talk,
    WriteLetter,
    ReadLetter,
    ReplyLetter,
    DeleteLetter,
    Refresh,
    SendLetter,
    Cancel,
    HideAll,
    Retry,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendSort {
    Name,
    Server,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterSort {
    Read,
    Sender,
    Date,
    Subject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendPresence {
    Online,
    Offline,
    Busy,
    Locked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendEntry {
    pub name: String,
    pub server: Option<u8>,
    pub presence: FriendPresence,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetterEntry {
    pub id: u32,
    pub sender: String,
    pub subject: String,
    pub date: String,
    pub time: String,
    pub read: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatRoomEntry {
    pub room_id: u32,
    pub title: String,
    pub member_count: u8,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FriendScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: FriendScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub tab_index: usize,
    pub chat_reject: bool,
    pub new_chat_alert: bool,
    pub new_mail_alert: bool,
    pub mail_mode: Option<MailMode>,
    pub friend_sort: Option<FriendSort>,
    pub letter_sort: Option<LetterSort>,
    pub friend_count: usize,
    pub letter_count: usize,
    pub chat_room_count: usize,
    pub selected_friend: Option<String>,
    pub selected_friend_server: Option<u8>,
    pub selected_letter_id: Option<u32>,
    pub selected_letter_sender: Option<String>,
    pub selected_letter_subject: Option<String>,
    pub draft_recipient: Option<String>,
    pub draft_subject: Option<String>,
    pub draft_body: Option<String>,
    pub selected_chat_room_id: Option<u32>,
    pub selected_chat_room_title: Option<String>,
    pub friends: Vec<FriendEntry>,
    pub letters: Vec<LetterEntry>,
    pub chat_rooms: Vec<ChatRoomEntry>,
    pub actions: &'static [FriendAction],
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FriendUiPlugin;

impl Plugin for FriendUiPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn friend_screen(state: FriendScreenState, mail: &MailManager) -> FriendScreen {
    let (tab_index, friend_sort, letter_sort, friend_count, letter_count, chat_room_count) =
        match state {
            FriendScreenState::Roster => (0, Some(FriendSort::Server), None, 3, 0, 0),
            FriendScreenState::Inbox if mail.letters_loaded() => {
                (1, None, Some(LetterSort::Read), 0, mail.letters().len(), 0)
            }
            FriendScreenState::Inbox => (1, None, Some(LetterSort::Read), 0, 3, 0),
            FriendScreenState::Compose => (1, None, Some(LetterSort::Subject), 0, 0, 0),
            FriendScreenState::ChatRooms => (2, None, None, 0, 0, 2),
            FriendScreenState::Error => (0, None, None, 0, 0, 0),
        };

    let friends = match state {
        FriendScreenState::Roster => build_friend_entries(),
        FriendScreenState::Inbox
        | FriendScreenState::Compose
        | FriendScreenState::ChatRooms
        | FriendScreenState::Error => Vec::new(),
    };
    let letters = match state {
        FriendScreenState::Inbox if mail.letters_loaded() => {
            build_live_letter_entries(mail.letters(), mail.selected_letter_id())
        }
        FriendScreenState::Inbox => build_letter_entries(mail.selected_letter_id()),
        FriendScreenState::Compose
        | FriendScreenState::Roster
        | FriendScreenState::ChatRooms
        | FriendScreenState::Error => Vec::new(),
    };
    let chat_rooms = match state {
        FriendScreenState::ChatRooms => build_chat_room_entries(),
        FriendScreenState::Roster
        | FriendScreenState::Inbox
        | FriendScreenState::Compose
        | FriendScreenState::Error => Vec::new(),
    };
    let selected_friend = match state {
        FriendScreenState::Roster => Some("Astra".to_owned()),
        _ => None,
    };
    let selected_friend_server = match state {
        FriendScreenState::Roster => Some(3),
        _ => None,
    };
    let selected_letter_id = match state {
        FriendScreenState::Inbox => mail.selected_letter_id(),
        _ => None,
    };
    let (selected_letter_sender, selected_letter_subject) = match selected_letter_id {
        Some(id) => selected_letter_details(&letters, id),
        None => (None, None),
    };
    let (draft_recipient, draft_subject, draft_body, mail_mode) = match state {
        FriendScreenState::Compose => (
            Some(mail.draft_recipient().to_owned()),
            Some(mail.draft_subject().to_owned()),
            Some(mail.draft_body().to_owned()),
            Some(mail.mode()),
        ),
        FriendScreenState::Inbox => (None, None, None, Some(mail.mode())),
        FriendScreenState::Roster | FriendScreenState::ChatRooms | FriendScreenState::Error => {
            (None, None, None, None)
        }
    };
    let (selected_chat_room_id, selected_chat_room_title) = match state {
        FriendScreenState::ChatRooms => (Some(17), Some("Blade / Selene".to_owned())),
        _ => (None, None),
    };

    FriendScreen {
        route: UiRoute::Friend,
        layout: UiShellLayout::for_route(UiRoute::Friend),
        widgets: UiShellWidgetSet::for_route(UiRoute::Friend),
        state,
        title: FRIEND_TITLE,
        notice: state.notice(),
        tab_index,
        chat_reject: false,
        new_chat_alert: matches!(state, FriendScreenState::Roster),
        new_mail_alert: mail.new_mail_alert(),
        mail_mode,
        friend_sort,
        letter_sort,
        friend_count,
        letter_count,
        chat_room_count,
        selected_friend,
        selected_friend_server,
        selected_letter_id,
        selected_letter_sender,
        selected_letter_subject,
        draft_recipient,
        draft_subject,
        draft_body,
        selected_chat_room_id,
        selected_chat_room_title,
        friends,
        letters,
        chat_rooms,
        actions: match state {
            FriendScreenState::Roster => ROSTER_ACTIONS,
            FriendScreenState::Inbox => INBOX_ACTIONS,
            FriendScreenState::Compose => COMPOSE_ACTIONS,
            FriendScreenState::ChatRooms => CHAT_ROOM_ACTIONS,
            FriendScreenState::Error => ERROR_ACTIONS,
        },
    }
}

impl FriendScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|tab_index={}|chat_reject={}|new_chat_alert={}|new_mail_alert={}|mail_mode={:?}|friend_sort={:?}|letter_sort={:?}|friend_count={}|letter_count={}|chat_room_count={}|selected_friend={:?}|selected_friend_server={:?}|selected_letter_id={:?}|selected_letter_sender={:?}|selected_letter_subject={:?}|draft_recipient={:?}|draft_subject={:?}|draft_body={:?}|selected_chat_room_id={:?}|selected_chat_room_title={:?}|friends={:?}|letters={:?}|chat_rooms={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.tab_index,
            self.chat_reject,
            self.new_chat_alert,
            self.new_mail_alert,
            self.mail_mode,
            self.friend_sort,
            self.letter_sort,
            self.friend_count,
            self.letter_count,
            self.chat_room_count,
            self.selected_friend,
            self.selected_friend_server,
            self.selected_letter_id,
            self.selected_letter_sender,
            self.selected_letter_subject,
            self.draft_recipient,
            self.draft_subject,
            self.draft_body,
            self.selected_chat_room_id,
            self.selected_chat_room_title,
            self.friends,
            self.letters,
            self.chat_rooms,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

fn build_friend_entries() -> Vec<FriendEntry> {
    vec![
        FriendEntry {
            name: "Astra".to_owned(),
            server: Some(3),
            presence: FriendPresence::Online,
            selected: true,
        },
        FriendEntry {
            name: "Blade".to_owned(),
            server: None,
            presence: FriendPresence::Offline,
            selected: false,
        },
        FriendEntry {
            name: "Selene".to_owned(),
            server: Some(6),
            presence: FriendPresence::Busy,
            selected: false,
        },
    ]
}

fn build_letter_entries(selected_letter_id: Option<u32>) -> Vec<LetterEntry> {
    let mut letters = vec![
        LetterEntry {
            id: 0x0102_0304,
            sender: "Astra".to_owned(),
            subject: "Potion run".to_owned(),
            date: "05/19/2026".to_owned(),
            time: "10:12".to_owned(),
            read: false,
            selected: false,
        },
        LetterEntry {
            id: 0x0102_0305,
            sender: "Selene".to_owned(),
            subject: "Castle prep".to_owned(),
            date: "05/18/2026".to_owned(),
            time: "21:40".to_owned(),
            read: true,
            selected: false,
        },
        LetterEntry {
            id: 0x0102_0306,
            sender: "Marlon".to_owned(),
            subject: "Guild meeting".to_owned(),
            date: "05/17/2026".to_owned(),
            time: "18:05".to_owned(),
            read: true,
            selected: false,
        },
    ];

    if let Some(selected_letter_id) = selected_letter_id {
        if let Some(letter) = letters
            .iter_mut()
            .find(|letter| letter.id == selected_letter_id)
        {
            letter.selected = true;
        }
    }

    letters
}

fn build_live_letter_entries(
    letters: &[MailLetterEntry],
    selected_letter_id: Option<u32>,
) -> Vec<LetterEntry> {
    let mut rows = letters
        .iter()
        .map(|letter| LetterEntry {
            id: letter.id,
            sender: letter.sender.clone(),
            subject: letter.subject.clone(),
            date: letter.date.clone(),
            time: letter.time.clone(),
            read: letter.read,
            selected: false,
        })
        .collect::<Vec<_>>();

    if let Some(selected_letter_id) = selected_letter_id {
        if let Some(letter) = rows
            .iter_mut()
            .find(|letter| letter.id == selected_letter_id)
        {
            letter.selected = true;
        }
    }

    rows
}

fn selected_letter_details(
    letters: &[LetterEntry],
    selected_letter_id: u32,
) -> (Option<String>, Option<String>) {
    let Some(letter) = letters
        .iter()
        .find(|letter| letter.id == selected_letter_id)
    else {
        return (None, None);
    };

    (Some(letter.sender.clone()), Some(letter.subject.clone()))
}

fn build_chat_room_entries() -> Vec<ChatRoomEntry> {
    vec![
        ChatRoomEntry {
            room_id: 17,
            title: "Blade / Selene".to_owned(),
            member_count: 2,
            selected: true,
        },
        ChatRoomEntry {
            room_id: 18,
            title: "Astra / Marlon".to_owned(),
            member_count: 2,
            selected: false,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{friend_screen, FriendScreenState};
    use mu_gameplay::{MailLetterEntry, MailManager};

    #[test]
    fn friend_roster_and_inbox_snapshots() {
        let mut mail = MailManager::new();
        mail.set_new_mail_alert(true);

        assert_eq!(
            friend_screen(FriendScreenState::Roster, &mail).snapshot(),
            "route=friend|group=gameplay|state=roster|title=Friend|notice=Some(\"Manage friends and chat invites.\")|tab_index=0|chat_reject=false|new_chat_alert=true|new_mail_alert=true|mail_mode=None|friend_sort=Some(Server)|letter_sort=None|friend_count=3|letter_count=0|chat_room_count=0|selected_friend=Some(\"Astra\")|selected_friend_server=Some(3)|selected_letter_id=None|selected_letter_sender=None|selected_letter_subject=None|draft_recipient=None|draft_subject=None|draft_body=None|selected_chat_room_id=None|selected_chat_room_title=None|friends=[FriendEntry { name: \"Astra\", server: Some(3), presence: Online, selected: true }, FriendEntry { name: \"Blade\", server: None, presence: Offline, selected: false }, FriendEntry { name: \"Selene\", server: Some(6), presence: Busy, selected: false }]|letters=[]|chat_rooms=[]|actions=[AddFriend, DeleteFriend, Talk, WriteLetter, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );

        mail.select_letter(0x0102_0304);

        assert_eq!(
            friend_screen(FriendScreenState::Inbox, &mail).snapshot(),
            "route=friend|group=gameplay|state=inbox|title=Friend|notice=Some(\"Review letters and reply.\")|tab_index=1|chat_reject=false|new_chat_alert=false|new_mail_alert=true|mail_mode=Some(Reading)|friend_sort=None|letter_sort=Some(Read)|friend_count=0|letter_count=3|chat_room_count=0|selected_friend=None|selected_friend_server=None|selected_letter_id=Some(16909060)|selected_letter_sender=Some(\"Astra\")|selected_letter_subject=Some(\"Potion run\")|draft_recipient=None|draft_subject=None|draft_body=None|selected_chat_room_id=None|selected_chat_room_title=None|friends=[]|letters=[LetterEntry { id: 16909060, sender: \"Astra\", subject: \"Potion run\", date: \"05/19/2026\", time: \"10:12\", read: false, selected: true }, LetterEntry { id: 16909061, sender: \"Selene\", subject: \"Castle prep\", date: \"05/18/2026\", time: \"21:40\", read: true, selected: false }, LetterEntry { id: 16909062, sender: \"Marlon\", subject: \"Guild meeting\", date: \"05/17/2026\", time: \"18:05\", read: true, selected: false }]|chat_rooms=[]|actions=[WriteLetter, ReadLetter, ReplyLetter, DeleteLetter, Refresh, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn friend_inbox_uses_live_letter_rows() {
        let mut mail = MailManager::new();
        mail.upsert_letter(MailLetterEntry {
            id: 0x0102_0304,
            sender: "Astra".to_owned(),
            subject: "Potion run".to_owned(),
            date: "05/19/2026".to_owned(),
            time: "10:12".to_owned(),
            read: false,
        });
        mail.upsert_letter(MailLetterEntry {
            id: 0x0102_0305,
            sender: "Selene".to_owned(),
            subject: "Castle prep".to_owned(),
            date: "05/18/2026".to_owned(),
            time: "21:40".to_owned(),
            read: true,
        });
        mail.select_letter(0x0102_0304);

        let snapshot = friend_screen(FriendScreenState::Inbox, &mail).snapshot();

        assert!(snapshot.contains("letter_count=2"));
        assert!(snapshot.contains("mail_mode=Some(Reading)"));
        assert!(snapshot.contains("selected_letter_id=Some(16909060)"));
        assert!(snapshot.contains(
            "letters=[LetterEntry { id: 16909060, sender: \"Astra\", subject: \"Potion run\", date: \"05/19/2026\", time: \"10:12\", read: false, selected: true }, LetterEntry { id: 16909061, sender: \"Selene\", subject: \"Castle prep\", date: \"05/18/2026\", time: \"21:40\", read: true, selected: false }]"
        ));
    }

    #[test]
    fn friend_compose_chat_rooms_and_error_snapshots() {
        let mut mail = MailManager::new();
        mail.set_new_mail_alert(true);
        mail.set_compose("Blade", "Re: Castle prep", "Meet at Lorencia.");

        assert_eq!(
            friend_screen(FriendScreenState::Compose, &mail).snapshot(),
            "route=friend|group=gameplay|state=compose|title=Friend|notice=Some(\"Write a new letter.\")|tab_index=1|chat_reject=false|new_chat_alert=false|new_mail_alert=true|mail_mode=Some(Compose)|friend_sort=None|letter_sort=Some(Subject)|friend_count=0|letter_count=0|chat_room_count=0|selected_friend=None|selected_friend_server=None|selected_letter_id=None|selected_letter_sender=None|selected_letter_subject=None|draft_recipient=Some(\"Blade\")|draft_subject=Some(\"Re: Castle prep\")|draft_body=Some(\"Meet at Lorencia.\")|selected_chat_room_id=None|selected_chat_room_title=None|friends=[]|letters=[]|chat_rooms=[]|actions=[SendLetter, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            friend_screen(FriendScreenState::ChatRooms, &mail).snapshot(),
            "route=friend|group=gameplay|state=chat-rooms|title=Friend|notice=Some(\"Manage chat rooms.\")|tab_index=2|chat_reject=false|new_chat_alert=false|new_mail_alert=true|mail_mode=None|friend_sort=None|letter_sort=None|friend_count=0|letter_count=0|chat_room_count=2|selected_friend=None|selected_friend_server=None|selected_letter_id=None|selected_letter_sender=None|selected_letter_subject=None|draft_recipient=None|draft_subject=None|draft_body=None|selected_chat_room_id=Some(17)|selected_chat_room_title=Some(\"Blade / Selene\")|friends=[]|letters=[]|chat_rooms=[ChatRoomEntry { room_id: 17, title: \"Blade / Selene\", member_count: 2, selected: true }, ChatRoomEntry { room_id: 18, title: \"Astra / Marlon\", member_count: 2, selected: false }]|actions=[HideAll, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            friend_screen(FriendScreenState::Error, &mail).snapshot(),
            "route=friend|group=gameplay|state=error|title=Friend|notice=Some(\"Friend sync failed.\")|tab_index=0|chat_reject=false|new_chat_alert=false|new_mail_alert=true|mail_mode=None|friend_sort=None|letter_sort=None|friend_count=0|letter_count=0|chat_room_count=0|selected_friend=None|selected_friend_server=None|selected_letter_id=None|selected_letter_sender=None|selected_letter_subject=None|draft_recipient=None|draft_subject=None|draft_body=None|selected_chat_room_id=None|selected_chat_room_title=None|friends=[]|letters=[]|chat_rooms=[]|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
