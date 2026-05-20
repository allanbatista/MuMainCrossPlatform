use bevy::prelude::{App, Plugin, Resource};

pub const MAX_GUILD_NAME_LENGTH: usize = 8;
pub const MAX_GUILD_MARKS: usize = 2000;
pub const GUILD_MARK_SIZE: usize = 64;
pub const GUILD_MARK_BYTES: usize = GUILD_MARK_SIZE / 2;
pub const GUILD_MARK_NIBBLE_SHIFT: u8 = 4;
pub const GUILD_MARK_NIBBLE_MASK: u8 = 0x0F;
pub const INVALID_GUILD_MARK_INDEX: i32 = -1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildMarkEntry {
    pub key: i32,
    pub union_name: String,
    pub guild_name: String,
    pub mark: [u8; GUILD_MARK_SIZE],
}

impl Default for GuildMarkEntry {
    fn default() -> Self {
        Self {
            key: INVALID_GUILD_MARK_INDEX,
            union_name: String::new(),
            guild_name: String::new(),
            mark: [0; GUILD_MARK_SIZE],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildCache {
    marks: Vec<GuildMarkEntry>,
    current_index: usize,
}

impl Default for GuildCache {
    fn default() -> Self {
        Self {
            marks: default_guild_marks(),
            current_index: 0,
        }
    }
}

impl Resource for GuildCache {}

#[derive(Debug, Default, Clone, Copy)]
pub struct GuildCachePlugin;

impl Plugin for GuildCachePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuildCache>();
    }
}

impl GuildCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn marks(&self) -> &[GuildMarkEntry] {
        &self.marks
    }

    pub fn mark(&self, index: usize) -> Option<&GuildMarkEntry> {
        self.marks.get(index)
    }

    pub fn is_exist_guild_mark(&self, guild_key: i32) -> bool {
        self.get_guild_mark_index(guild_key) != INVALID_GUILD_MARK_INDEX
    }

    pub fn get_guild_mark_index(&self, guild_key: i32) -> i32 {
        self.marks
            .iter()
            .take(self.current_index.min(self.marks.len()))
            .position(|mark| mark.key == guild_key)
            .map(|index| index as i32)
            .unwrap_or(INVALID_GUILD_MARK_INDEX)
    }

    pub fn make_guild_mark_index(&mut self, guild_key: i32) -> i32 {
        if self.current_index >= self.marks.len() {
            return INVALID_GUILD_MARK_INDEX;
        }

        let index = self.current_index;
        self.marks[index].key = guild_key;
        self.current_index += 1;
        index as i32
    }

    pub fn set_guild_mark(
        &mut self,
        guild_key: i32,
        union_name: &str,
        guild_name: &str,
        mark: &[u8],
    ) -> i32 {
        let index = self.get_guild_mark_index(guild_key);
        if index == INVALID_GUILD_MARK_INDEX {
            return INVALID_GUILD_MARK_INDEX;
        }

        let entry = &mut self.marks[index as usize];
        entry.union_name = normalize_guild_name(union_name);
        entry.guild_name = normalize_guild_name(guild_name);
        entry.mark = unpack_guild_mark(mark);
        index
    }
}

fn default_guild_marks() -> Vec<GuildMarkEntry> {
    std::iter::repeat_with(GuildMarkEntry::default)
        .take(MAX_GUILD_MARKS)
        .collect()
}

fn normalize_guild_name(value: &str) -> String {
    value.chars().take(MAX_GUILD_NAME_LENGTH).collect()
}

fn unpack_guild_mark(mark: &[u8]) -> [u8; GUILD_MARK_SIZE] {
    let mut unpacked = [0; GUILD_MARK_SIZE];

    for (index, slot) in unpacked.iter_mut().enumerate() {
        let byte = mark.get(index / 2).copied().unwrap_or(0);
        *slot = if index % 2 == 0 {
            (byte >> GUILD_MARK_NIBBLE_SHIFT) & GUILD_MARK_NIBBLE_MASK
        } else {
            byte & GUILD_MARK_NIBBLE_MASK
        };
    }

    unpacked
}

#[cfg(test)]
mod tests {
    use super::{
        GuildCache, GuildCachePlugin, INVALID_GUILD_MARK_INDEX, MAX_GUILD_MARKS,
        MAX_GUILD_NAME_LENGTH,
    };
    use bevy::prelude::App;

    fn packed_mark_bytes() -> [u8; super::GUILD_MARK_BYTES] {
        std::array::from_fn(|index| ((index as u8) << 4) | (index as u8 & 0x0F))
    }

    #[test]
    fn plugin_registers_the_guild_cache_resource() {
        let mut app = App::new();
        app.add_plugins(GuildCachePlugin);

        let cache = app.world().resource::<GuildCache>();
        assert_eq!(cache.current_index(), 0);
        assert_eq!(cache.marks().len(), MAX_GUILD_MARKS);
        assert!(cache
            .marks()
            .iter()
            .all(|mark| mark.key == INVALID_GUILD_MARK_INDEX));
    }

    #[test]
    fn guild_mark_cache_allocates_updates_and_decodes_marks() {
        let mut cache = GuildCache::new();
        assert_eq!(cache.make_guild_mark_index(42), 0);
        assert!(cache.is_exist_guild_mark(42));
        assert_eq!(cache.get_guild_mark_index(42), 0);

        let returned =
            cache.set_guild_mark(42, "Alliance Hub", "Night Watchers", &packed_mark_bytes());

        assert_eq!(returned, 0);

        let entry = cache.mark(0).expect("guild mark entry");
        assert_eq!(entry.key, 42);
        assert_eq!(entry.union_name.chars().count(), MAX_GUILD_NAME_LENGTH);
        assert_eq!(entry.union_name, "Alliance");
        assert_eq!(entry.guild_name.chars().count(), MAX_GUILD_NAME_LENGTH);
        assert_eq!(entry.guild_name, "Night Wa");
        assert_eq!(&entry.mark[..6], &[0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn guild_mark_cache_rejects_unknown_keys_and_full_capacity() {
        let mut cache = GuildCache::new();

        assert_eq!(
            cache.set_guild_mark(99, "Union", "Guild", &packed_mark_bytes()),
            INVALID_GUILD_MARK_INDEX
        );

        for guild_key in 0..MAX_GUILD_MARKS as i32 {
            assert_eq!(cache.make_guild_mark_index(guild_key), guild_key);
        }

        assert_eq!(cache.make_guild_mark_index(9_999), INVALID_GUILD_MARK_INDEX);
    }
}
