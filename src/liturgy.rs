//! The Theological Movements of Fas – Ite, Maledicti, in Ignem Aeternum.

use colored::*;

pub struct Movement {
    pub roman: &'static str,
    pub title: &'static str,
    pub latin: &'static str,
    pub scripture: &'static str,
}

pub const MOVEMENTS: [Movement; 6] = [
    Movement {
        roman: "I",
        title: "Obombratio",
        latin: "Quomodo cecidisti de caelo, Lucifer, qui mane oriebaris?",
        scripture: "They passed through a dark forest where the trees grew crooked and the earth gave up a smell of old wet ashes. There was no sound save the clacking of the mules.",
    },
    Movement {
        roman: "II",
        title: "The Shrine of Mad Laughter",
        latin: "In risu fletus commiscebitur, et extrema gaudii luctus occupat.",
        scripture: "He looked at the cracked bone between his fingers. A man will do what he must do when the cold comes on and the meat is gone from the pot.",
    },
    Movement {
        roman: "III",
        title: "Bread of Bitter Ashes",
        latin: "Panem cinis comedi, et potum meum cum fletu miscebam.",
        scripture: "They sat in the mud and watched the shadows lengthen. Hunger is not a desire; hunger is a debt owed to the dust that bore you.",
    },
    Movement {
        roman: "IV",
        title: "The Repellent Scars of Abandon and Darkness",
        latin: "Vulnera mea putruerunt et corruptae sunt a facie insipientiae meae.",
        scripture: "The skin on his ribs was drawn like cured parchment. He struck his breast with the jagged iron until the blood dripped dark into the dry leaves.",
    },
    Movement {
        roman: "V",
        title: "A Choreography of Empty Spaces",
        latin: "Ite, maledicti, in ignem aeternum, qui paratus est diabolo et angelis eius.",
        scripture: "Not a man among them spoke. Each held the bone in his left hand and stared upon his brother’s blade. Neither gave way. The stillness was total, like a clock stopped in a drowned house.",
    },
    Movement {
        roman: "VI",
        title: "Apokatastasis Panton",
        latin: "Reconciliatio omnium rerum per interitum universi.",
        scripture: "Whatever in creation exists without my knowledge exists without my consent. The judge smiles. He says that he will never die.",
    },
];

pub fn print_movement_proclamation(idx: usize) {
    if idx >= MOVEMENTS.len() {
        return;
    }
    let m = &MOVEMENTS[idx];
    println!();
    println!("{}", "═".repeat(80).dimmed());
    println!(
        "{} {} - {}",
        format!("[LITURGY {}]", m.roman).red().bold(),
        m.title.to_uppercase().white().bold(),
        m.latin.yellow().italic()
    );
    println!("{} \"{}\"", "  Proclamation:".dimmed(), m.scripture.cyan().italic());
    println!("{}", "═".repeat(80).dimmed());
    println!();
}
