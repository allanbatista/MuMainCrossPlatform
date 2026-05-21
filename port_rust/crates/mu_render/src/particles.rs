use std::collections::VecDeque;

use bevy::prelude::{App, Plugin, Resource};
use mu_gameplay::skills::{SkillId, SkillPresentation};

use crate::effects::{
    particle_cue_for_effect, SkillEffectEvent, SkillEffectQueue, SkillParticleCue,
};

const SNAPSHOT_SAMPLE_LIMIT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkillParticleState {
    #[default]
    Inactive,
    Ready,
}

impl SkillParticleState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillParticleEvent {
    pub skill_id: SkillId,
    pub cue: SkillParticleCue,
    pub target_id: Option<SkillId>,
}

impl SkillParticleEvent {
    pub fn from_presentation(
        skill_id: SkillId,
        presentation: SkillPresentation,
        target_id: Option<SkillId>,
    ) -> Option<Self> {
        let cue = particle_cue_for_effect(presentation.effect)?;

        Some(Self {
            skill_id,
            cue,
            target_id,
        })
    }

    pub fn from_skill_effect(effect: SkillEffectEvent) -> Option<Self> {
        let cue = effect.particle_cue()?;

        Some(Self {
            skill_id: effect.skill_id,
            cue,
            target_id: effect.target_id,
        })
    }

    pub fn snapshot(&self) -> String {
        format!(
            "skill={}|cue={}|target={}",
            self.skill_id,
            self.cue.snapshot_label(),
            self.target_id
                .map(|target_id| target_id.to_string())
                .unwrap_or_else(|| "none".to_string()),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillParticleQueue {
    events: VecDeque<SkillParticleEvent>,
}

impl Resource for SkillParticleQueue {}

#[derive(Debug, Default, Clone, Copy)]
pub struct SkillParticlePlugin;

impl Plugin for SkillParticlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkillParticleQueue>();
    }
}

impl SkillParticleQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> SkillParticleState {
        if self.events.is_empty() {
            SkillParticleState::Inactive
        } else {
            SkillParticleState::Ready
        }
    }

    pub fn push(&mut self, event: SkillParticleEvent) {
        self.events.push_back(event);
    }

    pub fn push_presentation(
        &mut self,
        skill_id: SkillId,
        presentation: SkillPresentation,
        target_id: Option<SkillId>,
    ) -> bool {
        let Some(event) = SkillParticleEvent::from_presentation(skill_id, presentation, target_id)
        else {
            return false;
        };

        self.push(event);
        true
    }

    pub fn push_skill_effect(&mut self, effect: SkillEffectEvent) -> bool {
        let Some(event) = SkillParticleEvent::from_skill_effect(effect) else {
            return false;
        };

        self.push(event);
        true
    }

    pub fn sync_skill_effects(&mut self, effects: &mut SkillEffectQueue) {
        for effect in effects.drain() {
            let _ = self.push_skill_effect(effect);
        }
    }

    pub fn pop(&mut self) -> Option<SkillParticleEvent> {
        self.events.pop_front()
    }

    pub fn drain(&mut self) -> Vec<SkillParticleEvent> {
        self.events.drain(..).collect()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn snapshot(&self) -> String {
        let events: Vec<SkillParticleEvent> = self.events.iter().cloned().collect();
        let events = snapshot_list(&events, SNAPSHOT_SAMPLE_LIMIT, SkillParticleEvent::snapshot);

        format!(
            "state={}|count={}|events={}",
            self.state().as_str(),
            self.len(),
            events,
        )
    }
}

fn snapshot_list<T, F>(entries: &[T], sample_limit: usize, snapshot: F) -> String
where
    F: Fn(&T) -> String,
{
    if entries.is_empty() {
        return "[]".to_string();
    }

    let mut parts: Vec<String> = entries.iter().take(sample_limit).map(snapshot).collect();
    if entries.len() > sample_limit {
        parts.push("...".to_string());
    }

    format!("[{}]", parts.join(", "))
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use super::{SkillParticleEvent, SkillParticlePlugin, SkillParticleQueue, SkillParticleState};
    use crate::effects::SkillEffectQueue;
    use mu_gameplay::skills::{SkillAudioCue, SkillEffectCue, SkillPresentation};

    #[test]
    fn particle_plugin_registers_queue_resource() {
        let mut app = App::new();
        app.add_plugins(SkillParticlePlugin);

        let particles = app.world().resource::<SkillParticleQueue>();
        assert_eq!(particles.state(), SkillParticleState::Inactive);
        assert!(particles.is_empty());
    }

    #[test]
    fn particle_queue_projects_effects_into_visual_cues() {
        let mut effects = SkillEffectQueue::new();
        effects.push_presentation(
            6,
            SkillPresentation::new(SkillEffectCue::Teleport, SkillAudioCue::Teleport),
            Some(99),
        );
        effects.push_presentation(
            51,
            SkillPresentation::new(SkillEffectCue::Projectile, SkillAudioCue::IceArrow),
            None,
        );
        effects.push_presentation(
            223,
            SkillPresentation::new(SkillEffectCue::Summon, SkillAudioCue::SummonExplosion),
            Some(7),
        );
        effects.push_presentation(
            77,
            SkillPresentation::new(SkillEffectCue::LegacyEffect(77), SkillAudioCue::Magic),
            Some(1),
        );
        effects.push_presentation(
            1,
            SkillPresentation::new(SkillEffectCue::None, SkillAudioCue::None),
            None,
        );

        let mut particles = SkillParticleQueue::new();
        particles.sync_skill_effects(&mut effects);

        assert!(effects.is_empty());
        assert_eq!(particles.state(), SkillParticleState::Ready);
        assert_eq!(particles.len(), 4);
        assert_eq!(
            particles.pop(),
            Some(SkillParticleEvent {
                skill_id: 6,
                cue: crate::effects::SkillParticleCue::TeleportBurst,
                target_id: Some(99),
            })
        );
        assert!(particles.snapshot().contains("cue=projectile-trail"));
        assert!(particles.snapshot().contains("cue=summon-cloud"));
        assert!(particles.snapshot().contains("cue=legacy-effect-77"));
    }

    #[test]
    fn particle_queue_ignores_presentations_without_a_visual_cue() {
        let mut queue = SkillParticleQueue::new();
        let presentation = SkillPresentation::new(SkillEffectCue::None, SkillAudioCue::Magic);

        assert!(!queue.push_presentation(7, presentation, None));
        assert!(queue.is_empty());
    }
}
