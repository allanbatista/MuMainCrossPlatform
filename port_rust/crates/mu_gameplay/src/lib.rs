pub mod buffs;
pub mod characters;
pub mod classes;
pub mod combat;
pub mod duel;
pub mod equipment;
pub mod events;
pub mod experience;
pub mod game_shop;
pub mod gens;
pub mod inventory;
pub mod items;
pub mod mail;
pub mod master_level;
pub mod mounts;
pub mod mu_helper;
pub mod mu_helper_runtime;
pub mod npc;
pub mod party;
pub mod pets;
pub mod player_shop;
pub mod quests;
pub mod skills;
pub mod stats;
pub mod summons;
pub mod trade;
pub mod vault;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use buffs::{
    split_duration, split_minutes_duration, BuffCatalog, BuffClass, BuffDefinition, BuffPlugin,
    BuffRegistry, BuffState, BuffTimeType, BuffTimerRegistry, BuffValueLoadType, DurationParts,
};
pub use characters::{next_experience_for_level, CharacterSheet};
pub use classes::{
    CharacterClass, CharacterSkinIndex, ServerClassType, MASTER_EXPERIENCE_UNLOCK_LEVEL,
};
pub use combat::{
    curse_skill_damage, magic_skill_damage, skill_damage, CombatBonuses, CombatProfile,
    DamageRange, SkillDamageProfile,
};
pub use duel::{
    DuelChannelInfo, DuelCharacterSnapshot, DuelManager, DuelPlayerInfo, DuelPlugin,
    MAX_DUEL_CHANNELS, MAX_DUEL_PLAYERS, MAX_USERNAME_SIZE,
};
pub use equipment::{EquipmentError, EquipmentManager, EquipmentPlugin, MAX_EQUIPMENT_SLOTS};
pub use events::{EventKind, EventManager, EventMode, EventPlugin};
pub use experience::{
    next_experience_for_level as experience_next_experience_for_level,
    next_master_level_experience as experience_next_master_level_experience,
    previous_experience_for_level, previous_master_level_experience, ExperienceBand,
    MASTER_LEVEL_EXPERIENCE_OFFSET, MASTER_LEVEL_EXPONENT_BONUS, MASTER_LEVEL_EXPONENT_SCALE,
    MASTER_LEVEL_OVERFLOW_SCALE, NORMAL_EXPERIENCE_LEVEL_BONUS, NORMAL_EXPERIENCE_SCALE,
    OVERLEVEL_EXPERIENCE_SCALE, OVERLEVEL_EXPERIENCE_THRESHOLD,
};
pub use game_shop::{
    GameShopCatalogState, GameShopExecutionState, GameShopManager, GameShopMode, GameShopPlugin,
    GameShopStorageState, GameShopVersion, GameShopWalletSummary,
};
pub use gens::{GensManager, GensMode, GensPlugin, GensType};
pub use inventory::{
    InventoryError, InventoryManager, InventoryPlugin, InventorySlot,
    INVENTORY_EXTENSION_PAGE_COUNT, INVENTORY_EXTENSION_PAGE_ROWS, INVENTORY_MAIN_PAGE_COLUMNS,
    INVENTORY_MAIN_PAGE_ROWS, INVENTORY_TOTAL_PAGE_COUNT,
};
pub use items::{
    EquipmentSlot, Item, ItemError, ItemOptionFlags, ItemPacketData, ItemPacketError,
    ItemRequirements, ItemSize, MAX_ITEM_INDEX, MAX_ITEM_SOCKETS, SOCKET_EMPTY,
};
pub use mail::{
    MailManager, MailMode, MailPlugin, MAX_MAIL_BODY_LENGTH, MAX_MAIL_RECIPIENT_LENGTH,
    MAX_MAIL_SUBJECT_LENGTH,
};
pub use master_level::{next_master_level_experience, MasterLevelState};
pub use mounts::{mount_camera_offset, MountKind, MountManager, MountPlugin};
pub use mu_helper::{
    MuHelperConfig, MuHelperConfigError, MuHelperDarkRavenMode, MuHelperMobCount,
    MuHelperMobPresence, MuHelperSkillCondition, MuHelperSkillSlot, MU_HELPER_MAX_EXTRA_ITEMS,
    MU_HELPER_MAX_EXTRA_ITEM_LENGTH, MU_HELPER_MAX_HUNTING_RANGE, MU_HELPER_MAX_OBTAINING_RANGE,
    MU_HELPER_MAX_SECONDS_AWAY, MU_HELPER_MAX_THRESHOLD, MU_HELPER_PACKET_PAYLOAD_SIZE,
    MU_HELPER_THRESHOLD_STEP,
};
pub use mu_helper_runtime::{MuHelperExecutionState, MuHelperRuntime, MuHelperRuntimePlugin};
pub use npc::{NpcDialogueState, NpcManager, NpcPlugin, NpcShopMode, NpcShopState};
pub use party::{
    PartyCharacterSnapshot, PartyManager, PartyMemberInfo, PartyPlugin, MAX_PARTY_ACTIVE_MEMBERS,
    MAX_PARTY_MEMBERS, PARTY_INDEX_HERO, PARTY_INDEX_NOT_FOUND, PARTY_INDEX_UNSEARCHED,
};
pub use pets::{
    calculate_pet_info, dark_horse_defence_bonus, dark_horse_level_requirement,
    dark_spirit_charisma_requirement, pet_item_value, PetAttackState, PetCommandMode,
    PetCommandState, PetInfo, PetKind, PetManager, PetPlugin,
};
pub use player_shop::{
    PlayerShopManager, PlayerShopMode, PlayerShopPlugin, MAX_PLAYER_SHOP_TITLE_LENGTH,
};
pub use quests::{QuestDialogueState, QuestManager, QuestMode, QuestPlugin, QuestRewardState};
pub use skills::{
    SkillAudioCue, SkillCatalog, SkillDefinition, SkillDisplayInfo, SkillEffectCue, SkillId,
    SkillManager, SkillPlugin, SkillPresentation, SkillRequirement, SkillRequirementsCache,
    SkillSlotState, SkillStatsSnapshot, AT_SKILL_CHAIN_DRIVE, AT_SKILL_CHAIN_DRIVE_STR,
    AT_SKILL_DRAGON_KICK, AT_SKILL_DRAGON_ROAR, AT_SKILL_DRAGON_ROAR_STR, AT_SKILL_ICE_ARROW,
    AT_SKILL_ICE_ARROW_STR, AT_SKILL_PENETRATION, AT_SKILL_PENETRATION_STR,
    AT_SKILL_SUMMON_EXPLOSION, AT_SKILL_SUMMON_POLLUTION, AT_SKILL_SUMMON_REQUIEM,
    AT_SKILL_TELEPORT, AT_SKILL_TELEPORT_ALLY, AT_SKILL_TRIPLE_SHOT, AT_SKILL_TRIPLE_SHOT_MASTERY,
    AT_SKILL_TRIPLE_SHOT_STR, AT_SKILL_UNDEFINED, MAX_SKILLS,
};
pub use stats::{base_class_attributes, ClassAttributes, BASE_CLASS_COUNT, CLASS_ATTRIBUTES};
pub use summons::{
    summon_weapon_level_tier, PlayerSummonPose, SummonCastState, SummonKind, SummonManager,
    SummonPlugin,
};
pub use trade::{TradeManager, TradeMode, TradePartnerInfo, TradePlugin, MAX_TRADE_WAIT_TICKS};
pub use vault::{
    VaultError, VaultManager, VaultMoneyDirection, VaultPendingTransfer, VaultPlugin, VaultSlot,
    VaultSyncOutcome, VAULT_PAGE_COUNT, VAULT_PAGE_ROWS, VAULT_PAGE_SLOTS,
};
