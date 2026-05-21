use std::collections::VecDeque;

use mu_gameplay::skills::{SkillAudioCue, SkillId, SkillPresentation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillAudioEvent {
    pub skill_id: SkillId,
    pub cue: SkillAudioCue,
    pub looped: bool,
}

impl SkillAudioEvent {
    pub fn from_presentation(
        skill_id: SkillId,
        presentation: SkillPresentation,
        looped: bool,
    ) -> Option<Self> {
        if matches!(presentation.audio, SkillAudioCue::None) {
            return None;
        }

        Some(Self {
            skill_id,
            cue: presentation.audio,
            looped,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillAudioQueue {
    events: VecDeque<SkillAudioEvent>,
}

impl SkillAudioQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: SkillAudioEvent) {
        self.events.push_back(event);
    }

    pub fn push_presentation(
        &mut self,
        skill_id: SkillId,
        presentation: SkillPresentation,
        looped: bool,
    ) -> bool {
        let Some(event) = SkillAudioEvent::from_presentation(skill_id, presentation, looped) else {
            return false;
        };

        self.push(event);
        true
    }

    pub fn pop(&mut self) -> Option<SkillAudioEvent> {
        self.events.pop_front()
    }

    pub fn drain(&mut self) -> Vec<SkillAudioEvent> {
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
    use super::{SkillAudioEvent, SkillAudioQueue};
    use mu_gameplay::skills::{SkillAudioCue, SkillEffectCue, SkillPresentation};

    #[test]
    fn queue_stores_audio_events_and_drains_them_in_order() {
        let mut queue = SkillAudioQueue::new();
        let presentation =
            SkillPresentation::new(SkillEffectCue::MagicCast, SkillAudioCue::IceArrow);

        assert!(queue.push_presentation(51, presentation, true));
        assert_eq!(queue.len(), 1);
        assert_eq!(
            queue.pop(),
            Some(SkillAudioEvent {
                skill_id: 51,
                cue: SkillAudioCue::IceArrow,
                looped: true,
            })
        );
        assert!(queue.is_empty());
    }

    #[test]
    fn queue_ignores_presentations_without_audio() {
        let mut queue = SkillAudioQueue::new();
        let presentation = SkillPresentation::new(SkillEffectCue::MagicCast, SkillAudioCue::None);

        assert!(!queue.push_presentation(6, presentation, false));
        assert!(queue.is_empty());
    }
}
