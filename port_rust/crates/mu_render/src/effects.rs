use std::collections::VecDeque;

use mu_gameplay::skills::{SkillEffectCue, SkillId, SkillPresentation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillParticleCue {
    LegacyEffect(u16),
    TeleportBurst,
    ProjectileTrail,
    SummonCloud,
    MagicCastGlow,
}

impl SkillParticleCue {
    pub fn snapshot_label(self) -> String {
        match self {
            Self::LegacyEffect(effect_id) => format!("legacy-effect-{effect_id}"),
            Self::TeleportBurst => "teleport-burst".to_string(),
            Self::ProjectileTrail => "projectile-trail".to_string(),
            Self::SummonCloud => "summon-cloud".to_string(),
            Self::MagicCastGlow => "magic-cast-glow".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillEffectEvent {
    pub skill_id: SkillId,
    pub effect: SkillEffectCue,
    pub target_id: Option<SkillId>,
}

impl SkillEffectEvent {
    pub fn from_presentation(
        skill_id: SkillId,
        presentation: SkillPresentation,
        target_id: Option<SkillId>,
    ) -> Option<Self> {
        if matches!(presentation.effect, SkillEffectCue::None) {
            return None;
        }

        Some(Self {
            skill_id,
            effect: presentation.effect,
            target_id,
        })
    }

    pub fn particle_cue(&self) -> Option<SkillParticleCue> {
        particle_cue_for_effect(self.effect)
    }
}

pub fn particle_cue_for_effect(effect: SkillEffectCue) -> Option<SkillParticleCue> {
    match effect {
        SkillEffectCue::None => None,
        SkillEffectCue::LegacyEffect(effect_id) => Some(SkillParticleCue::LegacyEffect(effect_id)),
        SkillEffectCue::Teleport => Some(SkillParticleCue::TeleportBurst),
        SkillEffectCue::Projectile => Some(SkillParticleCue::ProjectileTrail),
        SkillEffectCue::Summon => Some(SkillParticleCue::SummonCloud),
        SkillEffectCue::MagicCast => Some(SkillParticleCue::MagicCastGlow),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillEffectQueue {
    events: VecDeque<SkillEffectEvent>,
}

impl SkillEffectQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: SkillEffectEvent) {
        self.events.push_back(event);
    }

    pub fn push_presentation(
        &mut self,
        skill_id: SkillId,
        presentation: SkillPresentation,
        target_id: Option<SkillId>,
    ) -> bool {
        let Some(event) = SkillEffectEvent::from_presentation(skill_id, presentation, target_id)
        else {
            return false;
        };

        self.push(event);
        true
    }

    pub fn pop(&mut self) -> Option<SkillEffectEvent> {
        self.events.pop_front()
    }

    pub fn drain(&mut self) -> Vec<SkillEffectEvent> {
        self.events.drain(..).collect()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{particle_cue_for_effect, SkillEffectEvent, SkillEffectQueue, SkillParticleCue};
    use mu_gameplay::skills::{SkillAudioCue, SkillEffectCue, SkillId, SkillPresentation};

    #[test]
    fn queue_stores_effects_and_drains_them_in_order() {
        let mut queue = SkillEffectQueue::new();
        let presentation =
            SkillPresentation::new(SkillEffectCue::LegacyEffect(77), SkillAudioCue::Magic);

        assert!(queue.push_presentation(42, presentation, Some(99)));
        assert_eq!(queue.len(), 1);
        assert_eq!(
            queue.pop(),
            Some(SkillEffectEvent {
                skill_id: 42,
                effect: SkillEffectCue::LegacyEffect(77),
                target_id: Some(99),
            })
        );
        assert!(queue.is_empty());
    }

    #[test]
    fn queue_ignores_presentations_without_a_visual_cue() {
        let mut queue = SkillEffectQueue::new();
        let presentation = SkillPresentation::new(SkillEffectCue::None, SkillAudioCue::Magic);

        assert!(!queue.push_presentation(7 as SkillId, presentation, None));
        assert!(queue.is_empty());
    }

    #[test]
    fn effect_cues_map_to_representative_particle_cues() {
        assert_eq!(
            particle_cue_for_effect(SkillEffectCue::Teleport),
            Some(SkillParticleCue::TeleportBurst)
        );
        assert_eq!(
            particle_cue_for_effect(SkillEffectCue::Projectile),
            Some(SkillParticleCue::ProjectileTrail)
        );
        assert_eq!(
            particle_cue_for_effect(SkillEffectCue::Summon),
            Some(SkillParticleCue::SummonCloud)
        );
        assert_eq!(
            particle_cue_for_effect(SkillEffectCue::MagicCast),
            Some(SkillParticleCue::MagicCastGlow)
        );
        assert_eq!(particle_cue_for_effect(SkillEffectCue::None), None);
    }
}
