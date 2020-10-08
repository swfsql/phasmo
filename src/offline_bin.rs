use phasmo_rs::phasmo::{Evidence, Ghost, VariantIter};
use std::collections::HashSet;

fn main() {
    println!("Demonstration on how to use the functions:");
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

    let all_ghosts = Ghost::iter_variants();

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

    phasmo_rs::run()
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
    for ghost in Ghost::iter_variants() {
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
    for ghost in Ghost::iter_variants() {
        let feats: String = ghost.features().map(|e| e.to_string()).collect();
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
    let all_ghosts = Ghost::iter_variants();

    print!("Caution: ");
    let caution = Ghost::filter_by_caution_features(all_ghosts.iter().cloned());
    caution.iter().for_each(|c| print!("{}", c));

    println!();

    print!("Useful: ");
    let useful = Ghost::filter_by_useful_features(all_ghosts.into_iter());
    useful.iter().for_each(|u| print!("{}", u));
}
