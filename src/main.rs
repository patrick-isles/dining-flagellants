//! The Dining Flagellants
//! A foul, necrophagic inversion of Dijkstra's Dining Philosophers problem
//! scored to Deathspell Omega's Fas and Cormac McCarthy's Outer Dark.

mod altar;
mod liturgy;
mod penitent;
mod relic;

use std::env;
use colored::*;

use crate::altar::Altar;

fn print_header() {
    let header = r#"
 ╔═══════════════════════════════════════════════════════════════════════════════╗
 ║                                                                               ║
 ║                   †  T H E   D I N I N G   F L A G E L L A N T S  †          ║
 ║                                                                               ║
 ║     A Necrophagic Inversion of Dijkstra's Concurrency Problem (1965)          ║
 ║     In the theological tradition of Deathspell Omega's Fas                    ║
 ║     and Cormac McCarthy's Outer Dark                                          ║
 ║                                                                               ║
 ╚═══════════════════════════════════════════════════════════════════════════════╝
"#;
    println!("{}", header.red().bold());
}

fn print_help() {
    println!("{}", "USAGE:".white().bold());
    println!("  dining-flagellants [MODE]");
    println!();
    println!("{}", "MODES:".white().bold());
    println!("  deadlock    (Default) Execute Dijkstra's nightmare: symmetrical circular wait");
    println!("  starve      Execute terminal thread starvation, self-flagellation & necrophagy");
    println!("  liturgy     Run the complete 6-movement rite from Obombratio to Apokatastasis");
    println!("  altar       Render the circular pentagram altar of bones and irons");
    println!("  help        Display this testament");
    println!();
    println!("{}", "THE FIVE FLAGELLANTS:".dimmed());
    println!("  #0: Culla Holme       (The Blood-Denier)");
    println!("  #1: The Tinker        (Bearer of the sack)");
    println!("  #2: The Grim One      (Captain of the Triune)");
    println!("  #3: The Mute          (Second rider of the Triune)");
    println!("  #4: The Scalp-Scribe  (Witness of the Meridian)");
    println!();
}

fn main() {
    print_header();

    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("deadlock");

    let altar = Altar::new();

    match mode {
        "deadlock" => {
            println!("{}", "Mode: Symmetrical Circular Deadlock (A Choreography of Empty Spaces)".white().bold());
            altar.render_ascii_altar();
            altar.execute_symmetrical_deadlock();
        }
        "starve" | "necrophagy" => {
            println!("{}", "Mode: Ascetic Starvation & Necrophagous Predation (Bread of Bitter Ashes)".white().bold());
            altar.render_ascii_altar();
            altar.execute_necrophagic_starvation();
        }
        "liturgy" | "all" => {
            println!("{}", "Mode: The Complete Liturgy of Irreversible Entropy".white().bold());
            altar.render_ascii_altar();
            altar.execute_necrophagic_starvation();
            println!();
            altar.execute_symmetrical_deadlock();
        }
        "altar" => {
            altar.render_ascii_altar();
        }
        "help" | "-h" | "--help" => {
            print_help();
        }
        other => {
            println!("{} Unknown mode '{}'. Running deadlock by default.", "!".yellow(), other);
            altar.render_ascii_altar();
            altar.execute_symmetrical_deadlock();
        }
    }
}
