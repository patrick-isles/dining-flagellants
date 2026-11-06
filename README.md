# The Dining Flagellants †

> **A Necrophagic Inversion of Dijkstra’s Concurrency Problem (1965)**  
> *Scored to Deathspell Omega’s “Fas – Ite, Maledicti, in Ignem Aeternum” and Cormac McCarthy’s “Outer Dark”.*

[![Language: Rust](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![Concurrency: Coffman Deadlock Consecrated](https://img.shields.io/badge/Concurrency-Coffman_Cycle_Affirmed-red.svg)]()
[![Theology: Deathspell Omega](https://img.shields.io/badge/Theology-Fas_--_Ite_Maledicti-black.svg)]()
[![Literature: Cormac McCarthy](https://img.shields.io/badge/Literature-Outer_Dark_%2F_Blood_Meridian-purple.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

```
                       † THE ALTAR OF THE MIRE †
                           [Relic #0: Tibia]                        
                 ◈ [P0: Culla]        │        ◈ [P1: Tinker]
                 (Blood-Denier)       │        (Sack-Bearer)
                      │               │               │
             [Relic #4: Iron]         │      [Relic #1: Iron]
                      │               │               │
                 ◈ [P4: Scribe]       │        ◈ [P2: Grim One]
                 (Meridian Witness)   │        (Triune Captain)
                        \             │             /
                         [Relic #3]───────[Relic #2]
                               ◈ [P3: The Mute]
                               (Blackened Gums)
```

---

## I. The Repudiation of Dijkstra’s Illusion

In 1965, Edsger W. Dijkstra conceived the *Dining Philosophers Problem* as an idyllic Enlightenment fable: polite scholars seated around a table of rice or spaghetti, alternating between quiet contemplation and gentlemanly consumption, regulated by cooperative semaphore handshakes. Dijkstra sought to eradicate thread starvation, eliminate mutual deadlock, and ensure that every actor received its fair allotment of runtime cycles.

**`dining-flagellants` repudiates this bourgeois fantasy.**

The universe is not a cooperative multi-tenant scheduler; the universe is an open wound bleeding into dry scrub oak:
1. **The Shared Resources are Not Cutlery:** Between each pair of penitents lies a cracked human tibia gray with marrow rot (`RelicKind::CrackedTibia`) and a notched scalping iron pitted with dried gore (`RelicKind::RustedScalpingIron`).
2. **Starvation is Not a Bug; Starvation is the Liturgy:** A thread does not complain when starved of CPU cycles. Starvation (`starvation_ticks`) is the ascetic accumulation of holy decrepitude.
3. **Contention Compels Self-Mutilation:** When a thread fails to acquire a contested lock, it does not sleep or backoff: it executes `self_flagellate()`, deliberately corrupting its own local register bitmask (`memory_corruption_mask.rotate_left(3) ^ 0x00FF00AA`) with non-deterministic entropy.
4. **Deadlock is the Sacred Goal:** When all five flagellants simultaneously seize the left bone and demand the right, this circular wait (the 4th Coffman condition) is not broken by hierarchy or Chandy-Misra. **It is consecrated as The Great Stillness.**

---

## II. The Six Movements of *Fas*

The execution follows the theological architecture of Deathspell Omega’s landmark liturgical work:

### 1. *Obombratio* (The Overshadowing)
> *"Quomodo cecidisti de caelo, Lucifer, qui mane oriebaris?"*  
The circle is inscribed in the cold red mud. The five threads are spawned across bare-metal OS cores. The crystal oscillator marks time not as progress, but as an agony of dissipation.

### 2. *The Shrine of Mad Laughter*
> *"In risu fletus commiscebitur, et extrema gaudii luctus occupat."*  
Contention ignites. Five pairs of hands strike into the mire. The sound of splintered bone scraping rusted iron echoes across the bus.

### 3. *Bread of Bitter Ashes*
> *"Panem cinis comedi, et potum meum cum fletu miscebam."*  
The Triune starves Culla Holme. Lock denials mount. Hunger ascends from 1 to 12. The thread burns its own stack frame to keep the instruction pointer moving.

### 4. *The Repellent Scars of Abandon and Darkness*
> *"Vulnera mea putruerunt et corruptae sunt a facie insipientiae meae."*  
The flesh becomes an altar of absolute expenditure without return (*Dépense*). Bitwise rot overtakes the memory mask.

### 5. *A Choreography of Empty Spaces*
> *"Ite, maledicti, in ignem aeternum, qui paratus est diabolo et angelis eius."*  
Symmetrical Deadlock is consecrated:
$$\mathcal{P}_0 \xrightarrow{holds} \mathcal{R}_0 \xrightarrow{awaits} \mathcal{P}_1 \xrightarrow{holds} \mathcal{R}_1 \xrightarrow{awaits} \mathcal{P}_2 \xrightarrow{holds} \mathcal{R}_2 \xrightarrow{awaits} \mathcal{P}_3 \xrightarrow{holds} \mathcal{R}_3 \xrightarrow{awaits} \mathcal{P}_4 \xrightarrow{holds} \mathcal{R}_4 \xrightarrow{awaits} \mathcal{P}_0$$
No mutex yields. No thread backs off. The universe locks in eternal motionless prayer.

### 6. *Apokatastasis Panton* (The Universal Restoration)
> *"Reconciliatio omnium rerum per interitum universi."*  
The Grim One steps forward. When a starved thread crosses the threshold of terminal inanition, the neighboring thread executes `necrophagous_scavenge()`: splitting the victim's ribs, consuming its dirty heap pages, unmapping its virtual memory, and pitching the cadaver into the briars.

---

## III. Usage & Execution

```bash
# 1. Consecrate Dijkstra's Nightmare (Symmetrical Circular Deadlock)
cargo run --release -- deadlock

# 2. Ascetic Thread Starvation & Necrophagous Predation
cargo run --release -- starve

# 3. The Complete Liturgy of Irreversible Entropy (All 6 Movements)
cargo run --release -- liturgy

# 4. View the Circular Altar of Bones and Scalping Irons
cargo run --release -- altar
```

---

## IV. Verification

```bash
cargo test --verbose
```

```
running 7 tests
test test_relic_initialization ... ok
test test_ring_topology ... ok
test test_relic_claim_and_release ... ok
test test_hunger_and_state_escalation ... ok
test test_self_flagellation_memory_decay ... ok
test test_necrophagic_cadaver_state ... ok
test test_liturgical_movements ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 📜 The Five Flagellants

* **P0: Culla Holme** — *The Blood-Denier*, cursed to wander the wasteland with empty hands.
* **P1: The Tinker** — *The Scavenger*, bearing the unbaptized sack through brambles and swamp.
* **P2: The Grim One** — *Captain of the Triune*, wielding the dark bearded axe.
* **P3: The Mute** — *The Second Rider*, smiling with blackened gums and pale eyes.
* **P4: The Scalp-Scribe** — *Witness of the Meridian*, recording atrocity in blood and grease.

---

## ⚖️ License

MIT License. Authored by Patrick Isles.  
*"Whatever in creation exists without my knowledge exists without my consent."*
