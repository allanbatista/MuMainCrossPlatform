use std::collections::VecDeque;

use mu_gameplay::skills::{SkillEffectCue, SkillId, SkillPresentation};

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
    use super::{SkillEffectEvent, SkillEffectQueue};
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
}
