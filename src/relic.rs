//! The contested relics resting in the freezing mire between the flagellants.

use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelicKind {
    CrackedTibia,
    RustedScalpingIron,
}

#[allow(dead_code)]
impl RelicKind {
    pub fn description(&self) -> &'static str {
        match self {
            RelicKind::CrackedTibia => "splintered human tibia (gray with marrow rot)",
            RelicKind::RustedScalpingIron => "notched scalping iron (pitted with dried gore)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            RelicKind::CrackedTibia => "Tibia",
            RelicKind::RustedScalpingIron => "Scalping Iron",
        }
    }
}

#[allow(dead_code)]
pub struct Relic {
    pub id: usize,
    pub kind: RelicKind,
    pub seized_by: Option<usize>,
    pub blood_encrusted_pct: u8,
    pub consecutive_denials: AtomicUsize,
    pub total_feasts: AtomicUsize,
}

#[allow(dead_code)]
impl Relic {
    pub fn new(id: usize) -> Self {
        let kind = if id % 2 == 0 {
            RelicKind::CrackedTibia
        } else {
            RelicKind::RustedScalpingIron
        };

        Self {
            id,
            kind,
            seized_by: None,
            blood_encrusted_pct: 15 + (id as u8 * 12),
            consecutive_denials: AtomicUsize::new(0),
            total_feasts: AtomicUsize::new(0),
        }
    }

    pub fn is_free(&self) -> bool {
        self.seized_by.is_none()
    }

    pub fn claim(&mut self, penitent_id: usize) {
        self.seized_by = Some(penitent_id);
        self.blood_encrusted_pct = self.blood_encrusted_pct.saturating_add(8);
        self.total_feasts.fetch_add(1, Ordering::SeqCst);
    }

    pub fn release(&mut self, penitent_id: usize) {
        if self.seized_by == Some(penitent_id) {
            self.seized_by = None;
        }
    }

    pub fn record_denial(&self) -> usize {
        self.consecutive_denials.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn clear_denials(&self) {
        self.consecutive_denials.store(0, Ordering::SeqCst);
    }
}
