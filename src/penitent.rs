//! The five starving flagellants seated in the freezing red clay.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenitentState {
    /// Mutex contemplation / praying into the abyss
    Contemplating,
    /// Seeking to seize the left bone / right iron
    ClawingRelics,
    /// Starving / suffering resource starvation
    Starving,
    /// Consuming bitter ashes and rotting marrow
    FeastingOnAsh,
    /// Thread deliberately mutilating its own memory
    SelfFlagellating,
    /// Locked in symmetrical deadlock (The Great Stillness)
    Deadlocked,
    /// Consumed by a neighboring thread
    Cadaver,
}

impl PenitentState {
    pub fn badge(&self) -> ColoredString {
        match self {
            PenitentState::Contemplating => "CONTEMPLATING".cyan().dimmed(),
            PenitentState::ClawingRelics => "CLAWS_OUT".yellow().bold(),
            PenitentState::Starving => "STARVING".red().bold(),
            PenitentState::FeastingOnAsh => "FEASTING_ON_ASH".green().bold(),
            PenitentState::SelfFlagellating => "FLAGELLATING".magenta().bold(),
            PenitentState::Deadlocked => "DEADLOCKED_STILLNESS".white().on_red().bold(),
            PenitentState::Cadaver => "CADAVER_SCAVENGED".dimmed(),
        }
    }
}

#[allow(dead_code)]
pub struct Penitent {
    pub id: usize,
    pub name: &'static str,
    pub epithet: &'static str,
    pub left_relic_id: usize,
    pub right_relic_id: usize,
    pub hunger_ticks: AtomicUsize,
    pub scars_inflicted: AtomicUsize,
    pub ash_feasts: AtomicUsize,
    pub is_cadaver: AtomicBool,
    pub holds_left: AtomicBool,
    pub holds_right: AtomicBool,
    pub memory_corruption_mask: AtomicUsize,
}

impl Penitent {
    pub fn new(id: usize) -> Self {
        let (name, epithet) = match id {
            0 => ("Culla Holme", "The Blood-Denier, cursed to wander with empty hands"),
            1 => ("The Tinker", "Bearer of the unbaptized child through thorn and mire"),
            2 => ("The Grim One", "Captain of the Triune, axe-wielding scavenger of the dead"),
            3 => ("The Mute", "Second rider of the Triune, smiling with blackened gums"),
            4 => ("The Scalp-Scribe", "Witness of the Glanton meridian, recording atrocity in grease"),
            _ => ("Penitent", "Nameless flagellant in the ditch"),
        };

        // Standard ring topology: left = id, right = (id + 1) % 5
        let left_relic_id = id;
        let right_relic_id = (id + 1) % 5;

        Self {
            id,
            name,
            epithet,
            left_relic_id,
            right_relic_id,
            hunger_ticks: AtomicUsize::new(0),
            scars_inflicted: AtomicUsize::new(0),
            ash_feasts: AtomicUsize::new(0),
            is_cadaver: AtomicBool::new(false),
            holds_left: AtomicBool::new(false),
            holds_right: AtomicBool::new(false),
            memory_corruption_mask: AtomicUsize::new(0xDEADBEEF),
        }
    }

    pub fn hunger(&self) -> usize {
        self.hunger_ticks.load(Ordering::SeqCst)
    }

    pub fn increase_hunger(&self) -> usize {
        self.hunger_ticks.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn sate_hunger(&self) {
        self.hunger_ticks.store(0, Ordering::SeqCst);
        self.ash_feasts.fetch_add(1, Ordering::SeqCst);
    }

    pub fn self_flagellate(&self) -> usize {
        let scars = self.scars_inflicted.fetch_add(1, Ordering::SeqCst) + 1;
        // Bitwise rot in memory mask simulating unaligned entropy
        let prev = self.memory_corruption_mask.load(Ordering::SeqCst);
        self.memory_corruption_mask.store(prev.rotate_left(3) ^ 0x00FF00AA, Ordering::SeqCst);
        scars
    }

    pub fn state(&self) -> PenitentState {
        if self.is_cadaver.load(Ordering::SeqCst) {
            return PenitentState::Cadaver;
        }

        let l = self.holds_left.load(Ordering::SeqCst);
        let r = self.holds_right.load(Ordering::SeqCst);

        if l && r {
            PenitentState::FeastingOnAsh
        } else if l && !r {
            PenitentState::Deadlocked
        } else {
            let h = self.hunger();
            if h >= 8 {
                PenitentState::SelfFlagellating
            } else if h >= 4 {
                PenitentState::Starving
            } else if h >= 1 {
                PenitentState::ClawingRelics
            } else {
                PenitentState::Contemplating
            }
        }
    }
}
