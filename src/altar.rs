//! The Altar of the Mire: Manages the 5 Penitents and 5 Relics in concurrent contention.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use colored::*;

use crate::liturgy::print_movement_proclamation;
use crate::penitent::{Penitent, PenitentState};
use crate::relic::Relic;

#[allow(dead_code)]
pub struct Altar {
    pub penitents: Vec<Arc<Penitent>>,
    pub relics: Vec<Arc<Mutex<Relic>>>,
    pub running: Arc<AtomicBool>,
    pub deadlock_detected: Arc<AtomicBool>,
    pub total_cycles: Arc<AtomicUsize>,
}

impl Altar {
    pub fn new() -> Self {
        let mut penitents = Vec::new();
        let mut relics = Vec::new();

        for i in 0..5 {
            relics.push(Arc::new(Mutex::new(Relic::new(i))));
            penitents.push(Arc::new(Penitent::new(i)));
        }

        Self {
            penitents,
            relics,
            running: Arc::new(AtomicBool::new(true)),
            deadlock_detected: Arc::new(AtomicBool::new(false)),
            total_cycles: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Render terminal ASCII circular altar
    pub fn render_ascii_altar(&self) {
        println!("{}", "            † THE CIRCLE OF ASHES AND HUMAN MARROW †".red().bold());
        println!("{}", "                           [Relic #0: Tibia]                        ".dimmed());
        println!("                 {}          {}          {}", 
            self.penitent_glyph(0), 
            self.relic_status(0), 
            self.penitent_glyph(1)
        );
        println!("                 (Culla Holme)                    (The Tinker)");
        println!("                      │                                 │");
        println!("             [Relic #4: Iron]                  [Relic #1: Iron]");
        println!("             {}                                {}", 
            self.relic_status(4), 
            self.relic_status(1)
        );
        println!("                      │                                 │");
        println!("                 {}                                {}", 
            self.penitent_glyph(4), 
            self.penitent_glyph(2)
        );
        println!("             (The Scribe)                          (The Grim One)");
        println!("                        \\                     /");
        println!("                         [Relic #3]  [Relic #2]");
        println!("                               {}", self.penitent_glyph(3));
        println!("                              (The Mute)");
        println!();
    }

    fn penitent_glyph(&self, id: usize) -> String {
        let p = &self.penitents[id];
        let state = p.state();
        let badge = match state {
            PenitentState::FeastingOnAsh => "◈ [FEASTING]".green().bold(),
            PenitentState::Deadlocked => "✖ [DEADLOCKED]".red().bold(),
            PenitentState::Starving => "▲ [STARVING]".yellow().bold(),
            PenitentState::SelfFlagellating => "⚡ [SCARRING]".magenta().bold(),
            PenitentState::Cadaver => "☠ [CADAVER]".dimmed(),
            _ => "○ [WAITING]".cyan().dimmed(),
        };
        format!("{badge}")
    }

    fn relic_status(&self, id: usize) -> String {
        let r = self.relics[id].lock().unwrap();
        if let Some(owner) = r.seized_by {
            format!("{}: P#{}", r.kind.short_name().red().bold(), owner)
        } else {
            format!("{}: FREE", r.kind.short_name().green())
        }
    }

    /// Run the sacred Deadlock rite (Dijkstra's Nightmare)
    pub fn execute_symmetrical_deadlock(&self) {
        print_movement_proclamation(0); // Obombratio
        thread::sleep(Duration::from_millis(800));

        print_movement_proclamation(1); // The Shrine of Mad Laughter
        println!("{}", "All 5 flagellants lunge simultaneously into the freezing mud...".yellow().italic());
        println!("{}", "Each clutches the relic to their left with bloody fingers.".yellow().italic());
        println!();

        let mut handles = Vec::new();

        // Barrier to release all 5 threads at the exact same instant
        let barrier = Arc::new(std::sync::Barrier::new(5));

        for id in 0..5 {
            let penitent = Arc::clone(&self.penitents[id]);
            let left_relic = Arc::clone(&self.relics[penitent.left_relic_id]);
            let right_relic = Arc::clone(&self.relics[penitent.right_relic_id]);
            let deadlock_flag = Arc::clone(&self.deadlock_detected);
            let b = Arc::clone(&barrier);

            let handle = thread::spawn(move || {
                b.wait();

                // Step 1: Seize left relic
                let mut left_guard = left_relic.lock().unwrap();
                left_guard.claim(penitent.id);
                penitent.holds_left.store(true, Ordering::SeqCst);
                println!(
                    "  {} {} seized left {} #{}...",
                    "†".red(),
                    penitent.name.white().bold(),
                    left_guard.kind.short_name().yellow(),
                    left_guard.id
                );

                // Small delay to ensure all 5 threads acquire left before anyone gets right
                thread::sleep(Duration::from_millis(50));

                println!(
                    "  {} {} reaches for right {} #{} (held by brother)...",
                    "►".cyan(),
                    penitent.name.white().bold(),
                    if penitent.right_relic_id % 2 == 0 { "Tibia" } else { "Iron" },
                    penitent.right_relic_id
                );

                // Attempt to acquire right relic with non-blocking try_lock to demonstrate deadlock mathematically
                let start = Instant::now();
                loop {
                    if let Ok(mut right_guard) = right_relic.try_lock() {
                        if right_guard.is_free() {
                            right_guard.claim(penitent.id);
                            penitent.holds_right.store(true, Ordering::SeqCst);
                            penitent.sate_hunger();
                            println!("  {} {} feasted on ashes!", "★".green(), penitent.name);
                            break;
                        }
                    }

                    // Self-flagellate on lock contention
                    penitent.self_flagellate();
                    penitent.increase_hunger();

                    if start.elapsed() > Duration::from_millis(400) {
                        deadlock_flag.store(true, Ordering::SeqCst);
                        break;
                    }
                    thread::sleep(Duration::from_millis(40));
                }
            });
            handles.push(handle);
        }

        for h in handles {
            let _ = h.join();
        }

        thread::sleep(Duration::from_millis(500));
        print_movement_proclamation(4); // A Choreography of Empty Spaces

        println!("{}", "╔═══════════════════════════════════════════════════════════════════════════════╗".red().bold());
        println!("{}", "║                 CIRCULAR WAIT COFFMAN CONDITION CONSECRATED                   ║".white().on_red().bold());
        println!("{}", "║                                                                               ║".red().bold());
        println!("{}", "║  P[0] (Culla)    holds Relic#0, awaits Relic#1 [HELD BY The Tinker]           ║".white().bold());
        println!("{}", "║  P[1] (Tinker)   holds Relic#1, awaits Relic#2 [HELD BY The Grim One]         ║".white().bold());
        println!("{}", "║  P[2] (Grim One) holds Relic#2, awaits Relic#3 [HELD BY The Mute]             ║".white().bold());
        println!("{}", "║  P[3] (The Mute) holds Relic#3, awaits Relic#4 [HELD BY The Scalp-Scribe]     ║".white().bold());
        println!("{}", "║  P[4] (Scribe)   holds Relic#4, awaits Relic#0 [HELD BY Culla Holme]          ║".white().bold());
        println!("{}", "║                                                                               ║".red().bold());
        println!("{}", "║  CYCLE CONFIRMED: P0 → R1 → P1 → R2 → P2 → R3 → P3 → R4 → P4 → R0 → P0        ║".yellow().bold());
        println!("{}", "║  No thread yields. No mutex releases. The universe halts in eternal prayer.   ║".cyan().italic());
        println!("{}", "╚═══════════════════════════════════════════════════════════════════════════════╝".red().bold());
        println!();
    }

    /// Run the Starvation & Necrophagous Cannibalism rite
    pub fn execute_necrophagic_starvation(&self) {
        print_movement_proclamation(2); // Bread of Bitter Ashes
        println!("{}", "The Triune starves Culla Holme. His left relic is denied; his hunger ascends.".yellow().italic());
        println!();

        let victim = Arc::clone(&self.penitents[0]); // Culla
        let predator = Arc::clone(&self.penitents[2]); // The Grim One

        for tick in 1..=12 {
            let hunger = victim.increase_hunger();
            let scars = victim.self_flagellate();
            let mask = victim.memory_corruption_mask.load(Ordering::SeqCst);

            println!(
                "  [Tick #{:02}] {} | Hunger: {:02}/12 | Scars: {:02} | Rotten Mask: {:#010X} | State: {}",
                tick,
                victim.name.red().bold(),
                hunger,
                scars,
                mask,
                victim.state().badge()
            );
            thread::sleep(Duration::from_millis(120));
        }

        print_movement_proclamation(3); // The Repellent Scars
        println!();
        println!(
            "{} {} reaches terminal inanition. {} steps forward.",
            "☠".red().bold(),
            victim.name.white().bold(),
            predator.name.red().bold()
        );

        // Necrophagy: mark victim as cadaver and inherit context
        victim.is_cadaver.store(true, Ordering::SeqCst);
        predator.ash_feasts.fetch_add(1, Ordering::SeqCst);

        println!(
            "  {} {} splits Culla's ribcage with the rusted iron and consumes his stack frame.",
            "†".red().bold(),
            predator.name.white().bold()
        );
        println!(
            "  {} Culla's memory footprint unmapped. Cadaver tossed into the scrub oak.",
            "►".dimmed()
        );
        println!();

        print_movement_proclamation(5); // Apokatastasis Panton
    }
}
