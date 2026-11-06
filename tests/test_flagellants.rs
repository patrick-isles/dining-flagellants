#![allow(dead_code)]
use std::sync::atomic::Ordering;

// Test definitions directly against internal modules
#[path = "../src/penitent.rs"]
mod penitent;
#[path = "../src/relic.rs"]
mod relic;
#[path = "../src/liturgy.rs"]
mod liturgy;

use penitent::{Penitent, PenitentState};
use relic::{Relic, RelicKind};
use liturgy::MOVEMENTS;

#[test]
fn test_ring_topology() {
    for i in 0..5 {
        let p = Penitent::new(i);
        assert_eq!(p.left_relic_id, i);
        assert_eq!(p.right_relic_id, (i + 1) % 5);
    }
}

#[test]
fn test_relic_initialization() {
    let r0 = Relic::new(0);
    let r1 = Relic::new(1);
    assert_eq!(r0.kind, RelicKind::CrackedTibia);
    assert_eq!(r1.kind, RelicKind::RustedScalpingIron);
    assert!(r0.is_free());
    assert!(r1.is_free());
}

#[test]
fn test_relic_claim_and_release() {
    let mut r = Relic::new(0);
    assert!(r.is_free());
    r.claim(2);
    assert!(!r.is_free());
    assert_eq!(r.seized_by, Some(2));
    r.release(2);
    assert!(r.is_free());
}

#[test]
fn test_hunger_and_state_escalation() {
    let p = Penitent::new(0);
    assert_eq!(p.state(), PenitentState::Contemplating);

    p.increase_hunger();
    assert_eq!(p.state(), PenitentState::ClawingRelics);

    for _ in 0..4 {
        p.increase_hunger();
    }
    assert_eq!(p.state(), PenitentState::Starving);

    for _ in 0..4 {
        p.increase_hunger();
    }
    assert_eq!(p.state(), PenitentState::SelfFlagellating);

    p.sate_hunger();
    assert_eq!(p.hunger(), 0);
}

#[test]
fn test_self_flagellation_memory_decay() {
    let p = Penitent::new(0);
    let mask_initial = p.memory_corruption_mask.load(Ordering::SeqCst);
    let scars = p.self_flagellate();
    let mask_after = p.memory_corruption_mask.load(Ordering::SeqCst);

    assert_eq!(scars, 1);
    assert_ne!(mask_initial, mask_after);
}

#[test]
fn test_necrophagic_cadaver_state() {
    let p = Penitent::new(0);
    assert_ne!(p.state(), PenitentState::Cadaver);
    p.is_cadaver.store(true, Ordering::SeqCst);
    assert_eq!(p.state(), PenitentState::Cadaver);
}

#[test]
fn test_liturgical_movements() {
    assert_eq!(MOVEMENTS.len(), 6);
    assert_eq!(MOVEMENTS[0].title, "Obombratio");
    assert_eq!(MOVEMENTS[4].title, "A Choreography of Empty Spaces");
    assert_eq!(MOVEMENTS[5].title, "Apokatastasis Panton");
}
