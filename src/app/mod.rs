#![allow(clippy::unnecessary_operation)]

pub mod hlist;

use crate::phasmo;
use druid::{
    im::Vector,
    widget::{Button, Either, Flex, Label, List, Scroll},
    Data, Lens, UnitPoint, Widget, WidgetExt,
};
use hlist::Hlist;

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct AppData {
    evidences: Vector<EvidenceState>,
    ghosts: Vector<GhostState>,
}

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct EvidenceState {
    status: EvidenceStatus,
    #[data(same_fn = "PartialEq::eq")]
    kind: phasmo::Evidence,
}

#[derive(Debug, Clone, PartialEq, Data)]
#[repr(C)]
pub enum EvidenceStatus {
    Unknown,
    Found,
    Forbidden,
}

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct GhostState {
    is_possible: bool,
    #[data(same_fn = "PartialEq::eq")]
    kind: phasmo::Ghost,
}

impl Default for AppData {
    fn default() -> Self {
        use phasmo::VariantIter;
        let evidences: Vec<_> = phasmo::Evidence::iter_variants()
            .into_iter()
            .map(|e| EvidenceState {
                status: EvidenceStatus::default(),
                kind: e,
            })
            .collect();
        let ghosts: Vec<_> = phasmo::Ghost::iter_variants()
            .into_iter()
            .map(|g| GhostState {
                is_possible: true,
                kind: g,
            })
            .collect();
        Self {
            evidences: evidences.into(),
            ghosts: ghosts.into(),
        }
    }
}

impl Default for EvidenceStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl EvidenceStatus {
    pub fn next_status(&mut self) {
        use EvidenceStatus::*;
        *self = match self {
            Unknown => Found,
            Found => Forbidden,
            Forbidden => Unknown,
        };
    }
}

impl AppData {
    fn possible_ghosts(&self) -> Vec<phasmo::Ghost> {
        use phasmo::Ghost;
        let required: Vec<_> = self
            .evidences
            .iter()
            .filter_map(|e| {
                if let EvidenceStatus::Found = e.status {
                    Some(e.kind)
                } else {
                    None
                }
            })
            .collect();
        let forbid: Vec<_> = self
            .evidences
            .iter()
            .filter_map(|e| {
                if let EvidenceStatus::Forbidden = e.status {
                    Some(e.kind)
                } else {
                    None
                }
            })
            .collect();

        let ghosts = self.ghosts.clone().into_iter().map(|g| g.kind);

        let ghosts = Ghost::filter_by_required_evidences(ghosts, required.as_ref());
        Ghost::filter_by_forbid_evidences(ghosts.into_iter(), forbid.as_ref())
    }
}

pub fn ui_builder() -> impl Widget<AppData> {
    let mut root = Flex::column();

    let mut summary_flex = Flex::row();
    // let mut summary_ghosts_flex = Flex::row();
    let mut summary_features_flex = Flex::row();

    // summary
    {
        // possible ghosts
        {
            let ghost_icons = Label::new(|data: &AppData, _env: &_| {
                let ghosts = data.possible_ghosts();

                let ghosts_str: String = ghosts.iter().map(|g| g.to_string() + " ").collect();

                ghosts_str
            });

            let feature_icons = Label::new(|data: &AppData, _env: &_| {
                use phasmo::Ghost;
                let ghosts = data.possible_ghosts();

                let caution = Ghost::filter_by_caution_features(ghosts.iter().cloned());

                // caution.iter().for_each(|c| print!("{}", c));

                let useful = Ghost::filter_by_useful_features(ghosts.iter().cloned());

                // useful.iter().for_each(|u| print!("{}", u))

                let caution_str: String = caution.iter().map(|c| c.to_string() + " ").collect();
                let useful_str: String = useful.iter().map(|u| u.to_string() + " ").collect();

                caution_str + "/" + &useful_str
            });

            // summary_ghosts_flex.add_flex_child(ghost_icons, 1.0);
            summary_flex.add_flex_child(ghost_icons, 1.0);
            summary_features_flex.add_flex_child(feature_icons, 1.0);
            root.add_flex_child(summary_flex, 1.0);
            root.add_flex_child(summary_features_flex, 1.0);
            // root.add_child(ghost_icons);
        }
    };

    // evidences
    {
        let evidence_status = || {
            Label::new(|evidence: &EvidenceState, _env: &_| {
                match evidence.status {
                    // ☐
                    // ❓ ➖ ❔
                    EvidenceStatus::Unknown => "➖",
                    // ☑
                    // ✔️
                    EvidenceStatus::Found => "✔️",
                    // ☒
                    // ❌
                    EvidenceStatus::Forbidden => "❌",
                }
                .to_string()
            })
        };

        let evidence_button = || {
            Button::new(|evidence: &EvidenceState, _env: &_| evidence.kind.to_string()).on_click(
                |_ctx, e: &mut EvidenceState, _env| {
                    e.status.next_status();
                },
            )
        };

        let evidences = Hlist::new(move || {
            Flex::column()
                .with_child(evidence_button())
                .with_child(evidence_status())
            // .with_flex_spacer(1.0)
            // .padding(10.0)
            // .background(Color::rgb(0.5, 0.0, 0.5))
            // .fix_height(50.0)
        })
        .lens(AppData::evidences);

        let mut evidences_flex_list = Flex::row();
        evidences_flex_list.add_flex_child(evidences, 1.0);

        root.add_flex_child(evidences_flex_list, 1.0);
    }

    // reset button
    {
        let mut reset_flex = Flex::row();
        let reset_button = Button::new("🔄")
            .on_click(|_ctx, evidences: &mut Vector<EvidenceState>, _env: _| {
                for e in evidences.iter_mut() {
                    e.status = EvidenceStatus::default();
                }
            })
            .lens(AppData::evidences);

        // let mut reset_flex = Flex::row();
        // reset_flex.add_flex_child(reset_button, 1.0);
        // root.add_flex_child(reset_flex, 1.0);

        reset_flex.add_flex_child(reset_button, 1.0);
        root.add_child(reset_flex);
    }

    root
}
