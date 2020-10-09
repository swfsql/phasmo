use std::collections::HashSet;

pub trait VariantIter: Sized {
    fn iter_variants() -> Vec<Self>;
}

pub trait FilterBy<T> {
    fn is_related(self, t: T) -> bool;
}

// ps. the order is the same when inserting evidences in the journal
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Evidence {
    /// 📡
    EmfLevel5,
    /// 📻
    SpiritBox,
    /// 👣
    Fingerprints,
    /// ✨
    GhostOrb,
    /// 📖
    GhostWriting,
    /// 🥶
    FreezingTemperature,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Feature {
    Caution(CautionFeature),
    Useful(UsefulFeature),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CautionFeature {
    /// ️️⚔️
    Hostile,
    /// 🪁
    Flies,
    /// 🧱
    MoveThroughWalls,
    /// ⏩
    Fast,
    /// 🎯
    SingleTarget,
    /// 😨
    SanityDrain,
    /// 🐌
    Slow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UsefulFeature {
    /// ️✝️
    Crucifix,
    /// 👪
    Grouped,
    /// 🧂
    Salt,
    /// 🚬
    SmudgeSticks,
    /// 🕯️
    Light,
    /// 📸
    Picture,
    /// 🔌
    PowerSource,
    /// 🧹
    CleanRoom,
    /// 🙈
    Hiding,
}

// ps. the order is the same of the journal's ghost index
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ghost {
    /// ## 👻
    ///
    /// 🕵️ 🥶✨📖
    ///
    /// 💪 (none)  
    ///
    /// 😩 🚬 Smudge Sticks stop it's attacks for a long time  
    ///
    /// ---
    Spirit,

    /// ## 💀👻
    ///
    /// 🕵️ 👣🥶📻
    ///
    /// 💪 🪁 can fly  
    /// 💪 🪁 leaves no footsteps  
    /// 📝 🧂 except it leaves a step mark on salt  
    /// 💪 🧱 may move through walls  
    /// 💪 🪑 may change the Ghost Room more frequently  
    ///
    /// 😩 🧂 stops attacking when in contact with salt  
    /// 📝 🧂 but it becomes more agitated  
    ///
    /// ---
    Wraith,

    /// ## 🥛👻
    ///
    /// 🕵️ 📡🥶✨
    ///
    /// 💪 👀 looking at him drops your sanity  
    /// 💪 🧱 can go interact with anyone, even behind walls  
    /// 📝 (ie. random people at random places may detect Emf)  
    ///
    /// 😩 📸 dissapears if it's Shadow Form picture's taken  
    /// 📝 (📸 when hunting, camera's flash won't stop it)  
    /// 📝 🐌 it's not so fast  
    ///
    /// ---
    Phantom,

    /// ## 👻 (ﾉ◕ヮ◕)ﾉ︵ ┻━┻
    ///
    /// 🕵️ 👣✨📻
    ///
    /// 💪 (🤹 can throw/levitate many objects at once)  
    /// 📝 noise from many moved objects can be a consequence  
    ///
    /// 😩 (🧹 becomes almost innefective in an empty room)  
    ///
    /// ---
    Poltergeist,

    /// ## 🌬️
    ///
    /// 🕵️ 📡👣🥶
    ///
    /// 💪 🎯 1 person is targeted at a time  
    /// 💪 🎯 can start the hunt anytime  
    ///
    /// 😩 ✝️ is feared (and it's less agressive nearby one)  
    ///
    /// ---
    Banshee,

    /// ## 🧞
    ///
    /// 🕵️ 📡✨📻
    ///
    /// 💪 ⏩ moves fast if victim is far away  
    /// 📝 ⚔️ and it's terriorial - attacks when threatened  
    ///
    /// 😩 🔌 when off, Jinn cannot use it's "ability"  
    /// 📝 (🔌 counts only as the general power source)  
    ///
    /// ---
    Jinn,

    /// ## 🖤👻
    ///
    /// 🕵️ 🥶✨📻
    ///
    /// 💪 🕶️ attacks more in the dark  
    /// 📝 🔌 often turns lights / power source off  
    ///
    /// 😩 🕯️ attacks less in the light  
    ///
    /// ---
    Mare,

    /// ## 🧟👻
    ///
    /// 🕵️ 📡👣📖
    ///
    /// 💪 ⏩ fastests ghost when hunting  
    /// 💪 ⚔️ attacks regardless of sanity during hunt  
    ///
    /// 😩 🐌 slow otherwise, or when people are hiding  
    ///
    /// ---
    Revenant,

    /// ## 😳👻
    ///
    /// 🕵️ 📡✨📖
    ///
    /// 💪 (⚔️ hunts more often on low sanity)  
    /// 📝 prefers to target loners  
    /// 📝 "being alone" means "being alone in a room"  
    ///
    /// 😩 👪 won't hunt grouped people  
    /// 📝 will hardly interact with grouped people  
    ///
    /// ---
    Shade,

    /// ## 👿
    ///
    /// 🕵️ 🥶📖📻
    ///
    /// 💪 ⚔️ very hostile, attacks often  
    ///
    /// 😩 (🎲  Ouija doesn't lower sanity)  
    /// 📝 ✝️ is recommended to lower hunts  
    ///
    /// ---
    Demon,

    /// ## 🎎👻
    ///
    /// 🕵️ 🥶✨📖
    ///
    /// 💪 😨 fastest sanity drainer  
    ///
    /// 😩 🚬 Smudge Sticks can affect it's room  
    /// 📝 prevents it from wandering for a long time  
    ///
    /// ---
    Yurei,

    /// ## 👹
    ///
    /// 🕵️ 📡📖📻
    ///
    /// 💪 🏃 active when nearby it's prey  
    /// 📝 also moves objects fast  
    ///
    /// 😩 (none)  
    /// 📝 it's very active, and may show himself early  
    /// 📝 likes to wander close to it's room  
    /// 📝 🙈 hiding in closet may be effective  
    ///
    /// ---
    Oni,
}

impl std::fmt::Display for Ghost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Ghost::*;
        let emoji = match *self {
            Banshee => "🌬️",
            Demon => "👿",
            Jinn => "🧞",
            Mare => "🖤",
            Oni => "👹",
            Phantom => "🥛",
            Poltergeist => "🤹",
            Revenant => "🧟",
            Shade => "😳",
            Spirit => "👻",
            Wraith => "💀",
            Yurei => "🎎",
        };
        write!(f, "{}", emoji)
    }
}

impl std::fmt::Display for Evidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Evidence::*;
        let emoji = match self {
            EmfLevel5 => '📡',
            Fingerprints => '👣',
            FreezingTemperature => '🥶',
            GhostOrb => '✨',
            GhostWriting => '📖',
            SpiritBox => '📻',
        };
        write!(f, "{}", emoji)
    }
}

impl std::fmt::Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Feature::*;
        let emoji = match self {
            Caution(c) => c.to_string(),
            Useful(u) => u.to_string(),
        };
        write!(f, "{}", emoji)
    }
}

impl std::fmt::Display for CautionFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CautionFeature::*;
        let emoji = match self {
            SingleTarget => "🎯",
            Hostile => "⚔️",
            Fast => "⏩",
            Slow => "🐌",
            Flies => "🪁",
            MoveThroughWalls => "🧱",
            SanityDrain => "😨",
        };
        write!(f, "{}", emoji)
    }
}

impl std::fmt::Display for UsefulFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use UsefulFeature::*;
        let emoji = match self {
            Crucifix => "✝️",
            PowerSource => "🔌",
            Light => "🕯️",
            Hiding => "🙈",
            Picture => "📸",
            CleanRoom => "🧹",
            Grouped => "👪",
            SmudgeSticks => "🚬",
            Salt => "🧂",
        };
        write!(f, "{}", emoji)
    }
}

impl VariantIter for Evidence {
    fn iter_variants() -> Vec<Self> {
        use Evidence::*;
        vec![
            EmfLevel5,
            SpiritBox,
            Fingerprints,
            GhostOrb,
            GhostWriting,
            FreezingTemperature,
        ]
    }
}

impl VariantIter for Ghost {
    fn iter_variants() -> Vec<Self> {
        use Ghost::*;
        vec![
            Spirit,
            Wraith,
            Phantom,
            Poltergeist,
            Banshee,
            Jinn,
            Mare,
            Revenant,
            Shade,
            Demon,
            Yurei,
            Oni,
        ]
    }
}

impl VariantIter for Feature {
    fn iter_variants() -> Vec<Self> {
        use Feature::*;
        let caution = CautionFeature::iter_variants().into_iter().map(Caution);
        let useful = UsefulFeature::iter_variants().into_iter().map(Useful);
        caution.chain(useful).collect()
    }
}

impl VariantIter for CautionFeature {
    fn iter_variants() -> Vec<Self> {
        use CautionFeature::*;
        vec![
            Hostile,
            Flies,
            MoveThroughWalls,
            Fast,
            SingleTarget,
            SanityDrain,
            Slow,
        ]
    }
}

impl VariantIter for UsefulFeature {
    fn iter_variants() -> Vec<Self> {
        use UsefulFeature::*;
        vec![
            Crucifix,
            Grouped,
            Salt,
            SmudgeSticks,
            Light,
            Picture,
            PowerSource,
            CleanRoom,
            Hiding,
        ]
    }
}

impl Evidence {
    // ✔️
    pub fn required_filter(
        self,
        ghosts: impl Iterator<Item = Ghost>,
    ) -> impl Iterator<Item = Ghost> {
        ghosts.filter(move |g| g.is_related(self))
    }

    // ❌
    pub fn forbidden_filter(
        self,
        ghosts: impl Iterator<Item = Ghost>,
    ) -> impl Iterator<Item = Ghost> {
        ghosts.filter(move |g| !g.is_related(self))
    }
}

impl FilterBy<Evidence> for Ghost {
    fn is_related(self, evidence: Evidence) -> bool {
        use Evidence::*;
        use Ghost::*;
        match (self, evidence) {
            (Banshee, EmfLevel5 | Fingerprints | FreezingTemperature) => true,
            (Demon, FreezingTemperature | GhostWriting | SpiritBox) => true,
            (Jinn, SpiritBox | GhostOrb | EmfLevel5) => true,
            (Mare, SpiritBox | GhostOrb | FreezingTemperature) => true,
            (Oni, EmfLevel5 | SpiritBox | GhostWriting) => true,
            (Phantom, EmfLevel5 | GhostOrb | FreezingTemperature) => true,
            (Poltergeist, SpiritBox | Fingerprints | GhostOrb) => true,
            (Revenant, EmfLevel5 | Fingerprints | GhostWriting) => true,
            (Shade, EmfLevel5 | GhostOrb | GhostWriting) => true,
            (Spirit, GhostOrb | GhostWriting | FreezingTemperature) => true,
            (Wraith, Fingerprints | SpiritBox | FreezingTemperature) => true,
            (Yurei, GhostOrb | GhostWriting | FreezingTemperature) => true,
            _ => false,
        }
    }
}

impl FilterBy<Feature> for Ghost {
    fn is_related(self, feat: Feature) -> bool {
        use CautionFeature::*;
        use Feature::*;
        use Ghost::*;
        use UsefulFeature::*;
        match (self, feat) {
            (Banshee, Caution(SingleTarget) | Useful(Crucifix)) => true,
            (Demon, Caution(Hostile) | Useful(Crucifix)) => true,
            (Jinn, Caution(Hostile) | Caution(Fast) | Useful(PowerSource)) => true,
            (Mare, Useful(Light)) => true,
            (Oni, Useful(Hiding)) => true,
            (Phantom, Caution(Slow) | Useful(Picture)) => true,
            (Poltergeist, Useful(CleanRoom)) => true,
            (Revenant, Caution(Hostile) | Caution(Fast) | Caution(Slow)) => true,
            (Shade, Useful(Grouped)) => true,
            (Spirit, Useful(SmudgeSticks)) => true,
            (Wraith, Caution(Flies) | Caution(MoveThroughWalls) | Useful(Salt)) => true,
            (Yurei, Caution(SanityDrain) | Useful(SmudgeSticks)) => true,
            _ => false,
        }
    }
}

impl Ghost {
    pub fn description(&self) -> &'static str {
        use Ghost::*;
        match *self {
            Banshee => {
                "\
                💪 🎯 1 person is targeted at a time\n\
                💪 🎯 can start the hunt anytime\n\
                \n\
                😩 ✝️ is feared (and it's less agressive nearby one)\
                "
            }

            Demon => {
                "\
                💪 ⚔️ very hostile, attacks often\n\
                \n\
                😩 (🎲  Ouija doesn't lower sanity)\n\
                📝 ✝️ is recommended to lower hunts\n\
                "
            }

            Jinn => {
                "\
                💪 ⏩ moves fast if victim is far away\n\
                📝 ⚔️ and it's terriorial - attacks when threatened\n\
                \n\
                😩 🔌 when off, Jinn cannot use it's 'ability'\n\
                📝 (🔌 counts only as the general power source)\n\
                "
            }

            Mare => {
                "\
                💪 🕶️ attacks more in the dark\n\
                📝 🔌 often turns lights / power source off\n\
                \n\
                😩 🕯️ attacks less in the light\n\
                "
            }

            Oni => {
                "\
                💪 🏃 active when nearby it's prey\n\
                📝 also moves objects fast\n\
                \n\
                😩 (none)\n\
                📝 it's very active, and may show himself early\n\
                📝 likes to wander close to it's room\n\
                📝 🙈 hiding in closet may be effective\n\
                "
            }

            Phantom => {
                "\
                💪 👀 looking at him drops your sanity\n\
                💪 🧱 can go interact with anyone, even behind walls\n\
                📝 (ie. random people at random places may detect Emf)\n\
                \n\
                😩 📸 dissapears if it's Shadow Form picture's taken\n\
                📝 (📸 when hunting, camera's flash won't stop it)\n\
                📝 🐌 it's not so fast\n\
                "
            }

            Poltergeist => {
                "\
                💪 (🤹 can throw/levitate many objects at once)\n\
                📝 noise from many moved objects can be a consequence\n\
                \n\
                😩 (🧹 becomes almost innefective in an empty room)\n\
                "
            }

            Revenant => {
                "\
                💪 ⏩ fastests ghost when hunting\n\
                💪 ⚔️ attacks regardless of sanity during hunt\n\
                \n\
                😩 🐌 slow otherwise, or when people are hiding\n\
                "
            }

            Shade => {
                "\
                💪 (⚔️ hunts more often on low sanity)\n\
                📝 prefers to target loners\n\
                📝 'being alone' means 'being alone in a room'\n\
                \n\
                😩 👪 won't hunt grouped people\n\
                📝 will hardly interact with grouped people\n\
                "
            }

            Spirit => {
                "\
                💪 (none)\n\
                \n\
                😩 🚬 Smudge Sticks stop it's attacks for a long time\n\
                "
            }

            Wraith => {
                "\
                💪 🪁 can fly\n\
                💪 🪁 leaves no footsteps\n\
                📝 🧂 except it leaves a step mark on salt\n\
                💪 🧱 may move through walls\n\
                💪 🪑 may change the Ghost Room more frequently\n\
                \n\
                😩 🧂 stops attacking when in contact with salt\n\
                📝 🧂 but it becomes more agitated\n\
                "
            }

            Yurei => {
                "\
                💪 😨 fastest sanity drainer\n\
                \n\
                😩 🚬 Smudge Sticks can affect it's room\n\
                📝 prevents it from wandering for a long time\n\
                "
            }
        }
    }
    pub fn evidences(self) -> impl Iterator<Item = Evidence> {
        Evidence::iter_variants()
            .into_iter()
            .filter(move |e| self.is_related(*e))
    }
    pub fn features(self) -> impl Iterator<Item = Feature> {
        Feature::iter_variants()
            .into_iter()
            .filter(move |e| self.is_related(*e))
    }
    pub fn caution_features(self) -> impl Iterator<Item = CautionFeature> {
        CautionFeature::iter_variants()
            .into_iter()
            .filter(move |e| self.is_related(Feature::Caution(*e)))
    }
    pub fn useful_features(self) -> impl Iterator<Item = UsefulFeature> {
        UsefulFeature::iter_variants()
            .into_iter()
            .filter(move |e| self.is_related(Feature::Useful(*e)))
    }

    pub fn filter_by_required_evidences(
        ghosts: impl Iterator<Item = Self>,
        required_evidences: &[Evidence],
    ) -> Vec<Ghost> {
        ghosts
            .filter(move |g| required_evidences.iter().all(|re| g.is_related(*re)))
            .collect()
    }

    pub fn filter_by_forbid_evidences(
        ghosts: impl Iterator<Item = Self>,
        forbid_evidences: &[Evidence],
    ) -> Vec<Ghost> {
        ghosts
            .filter(move |g| forbid_evidences.iter().all(move |re| !g.is_related(*re)))
            .collect()
    }

    pub fn filter_by_caution_features(
        ghosts: impl Iterator<Item = Self>,
    ) -> HashSet<CautionFeature> {
        ghosts
            .map(|g| g.caution_features())
            .fold(HashSet::new(), |mut caution_acc, caution| {
                caution.for_each(|c| {
                    caution_acc.insert(c);
                });
                caution_acc
            })
    }

    pub fn filter_by_useful_features(ghosts: impl Iterator<Item = Self>) -> HashSet<UsefulFeature> {
        ghosts
            .map(|g| g.useful_features())
            .fold(HashSet::new(), |mut useful_acc, useful| {
                useful.for_each(|u| {
                    useful_acc.insert(u);
                });
                useful_acc
            })
    }
}
