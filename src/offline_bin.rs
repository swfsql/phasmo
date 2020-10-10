use phasmo_rs::phasmo::{self, Evidence, Ghost};
use std::collections::HashSet;

fn main() {
    phasmo_rs::app::run()
}

/// Demonstration on how to use some functions.
pub fn print_info() {
    println!("Demonstration on how to use some functions:");
    println!();

    println!("Ghosts and evidences");
    print_ghost_evidences();
    println!();
    println!("Ghosts and features");
    print_ghost_features();
    println!();
    println!("all feature sets");
    print_all_features();

    println!();
    println!();

    let all_ghosts = phasmo::GHOSTS;

    println!("Requires Emf5:");
    // 🧞👹🥛😳🌬️🧟
    let requries = Evidence::EmfLevel5;
    let ghosts: HashSet<Ghost> = requries
        .required_filter(all_ghosts.iter().cloned())
        .collect();
    for ghost in ghosts.iter() {
        print!("{}", ghost);
    }
    println!();

    println!("Requires Emf5, but does not requires fingerprints:");
    // 🧞👹🥛😳
    let requires = vec![Evidence::EmfLevel5];
    let forbids = vec![Evidence::Fingerprints];
    //
    let ghosts = Ghost::filter_by_required_evidences(all_ghosts.iter().cloned(), requires.as_ref());
    let ghosts = Ghost::filter_by_forbid_evidences(ghosts.into_iter(), forbids.as_ref());
    ghosts.iter().for_each(|ghost| {
        print!("{}", ghost);
    });
}

/// Prints:
///
/// Banshee: 📡👣🥶
/// Demon: 🥶📖📻
/// Jinn: 📡✨📻
/// Mare: 🥶✨📻
/// Oni: 📡📖📻
/// Phantom: 📡🥶✨
/// Poltergeist: 👣✨📻
/// Revenant: 📡👣📖
/// Shade: 📡✨📖
/// Spirit: 🥶✨📖
/// Wraith: 👣🥶📻
/// Yurei: 🥶✨📖
///
/// ie. for each ghost, it's evidences.
pub fn print_ghost_evidences() {
    for ghost in phasmo::GHOSTS.iter() {
        let evidences: String = ghost.evidences().map(|e| e.to_string()).collect();
        println!("{:?}: {}", ghost, evidences);
    }
}

/// Prints:
///
/// Banshee: 🎯✝️
/// Demon: ⚔️✝️
/// Jinn: ⚔️⏩🔌
/// Mare: 🕯️
/// Oni: 🙈
/// Phantom: 🐌📸
/// Poltergeist: 🧹
/// Revenant: ⚔️⏩🐌
/// Shade: 👪
/// Spirit: 🚬
/// Wraith: 🪁🧱🧂
/// Yurei: 😨🚬
///
/// ie. for each ghost, it's caution features
/// followed by it's useful features.
pub fn print_ghost_features() {
    use phasmo::Feature::*;
    for ghost in phasmo::GHOSTS.iter() {
        let feats: String = ghost
            .caution_features()
            .map(Caution)
            .chain(ghost.useful_features().map(Useful))
            .map(|e| e.to_string())
            .collect();
        println!("{:?}: {}", ghost, feats);
    }
}

/// Prints:
/// Caution: 🐌🧱⚔️🎯😨⏩🪁
/// Useful: 👪🧂📸🚬🧹🔌✝️🙈🕯️
///
/// ie. a union set of caution features applicable for each ghost,
/// and a union set of useful feature applicable for each ghost.
pub fn print_all_features() {
    let all_ghosts = phasmo::GHOSTS;

    print!("Caution: ");
    let caution = Ghost::filter_by_caution_features(all_ghosts.iter().cloned());
    caution.iter().for_each(|c| print!("{}", c));

    println!();

    print!("Useful: ");
    let useful = Ghost::filter_by_useful_features(all_ghosts.iter().cloned());
    useful.iter().for_each(|u| print!("{}", u));
}
