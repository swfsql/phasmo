#![allow(clippy::unnecessary_operation)]

pub mod hlist;

use crate::phasmo;
use druid::{
    im::Vector,
    widget::{Button, Flex, Label},
    Data, Lens, Widget, WidgetExt,
};
use hlist::Hlist;

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct AppData {
    evidences: Vector<EvidenceState>,
    ghosts: Vector<Wghost>,
    selected: Wghost,
}

#[derive(Debug, Clone, PartialEq, Data)]
pub struct Wghost(#[data(same_fn = "PartialEq::eq")] pub phasmo::Ghost);

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
        let ghosts: Vector<_> = phasmo::Ghost::iter_variants()
            .into_iter()
            .map(Wghost)
            .collect();
        #[allow(clippy::deref_addrof)]
        let selected = ghosts[0].clone();
        Self {
            evidences: evidences.into(),
            ghosts,
            selected,
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

        let ghosts = Ghost::filter_by_required_evidences(
            self.ghosts.iter().cloned().map(|g| g.0),
            required.as_ref(),
        );
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
        // let ghost_icons = Flex::row()
        let ghost_icon = || {
            Button::new(|(g, _): &(Wghost, Option<Wghost>), _env: &_| g.0.to_string()).on_click(
                |_ctx, (g, selected): &mut (Wghost, Option<Wghost>), _env| {
                    *selected = Some(g.clone());
                },
            )
        };

        use druid::LensExt;
        let ghost_icons = Hlist::new(move || {
            Flex::column().with_child(ghost_icon())
            // .with_flex_spacer(1.0)
            // .padding(10.0)
            // .background(Color::rgb(0.5, 0.0, 0.5))
            // .fix_height(50.0)
        })
        .lens(druid::lens::Id.map(
            |d: &AppData| {
                d.possible_ghosts()
                    .into_iter()
                    .map(Wghost)
                    .map(|g| (g, None))
                    .collect::<Vector<_>>()
            },
            |d: &mut AppData, x: Vector<(Wghost, Option<Wghost>)>| {
                if let Some(selected) = x.iter().filter_map(|(_, selected)| selected.clone()).next()
                {
                    d.selected = selected;
                };
            },
        ));

        let feature_icons = Label::new(|data: &AppData, _env: &_| {
            use phasmo::Ghost;
            let ghosts = data.possible_ghosts();

            let mut caution: Vec<_> = Ghost::filter_by_caution_features(ghosts.iter().cloned())
                .into_iter()
                .collect();
            caution.sort_unstable();

            let mut useful: Vec<_> = Ghost::filter_by_useful_features(ghosts.iter().cloned())
                .into_iter()
                .collect();
            useful.sort_unstable();

            let caution_str: String = caution.iter().map(|c| c.to_string() + " ").collect();
            let useful_str: String = useful.iter().map(|u| u.to_string() + " ").collect();

            caution_str + "/" + &useful_str
        });

        summary_flex.add_flex_child(ghost_icons, 1.0);
        summary_features_flex.add_flex_child(feature_icons, 1.0);
        root.add_flex_child(summary_flex, 1.0);
        root.add_flex_child(summary_features_flex, 1.0);
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
            .padding((0., 2., 0., 0.))
            .lens(AppData::evidences);

        reset_flex.add_flex_child(reset_button, 1.0);
        root.add_child(reset_flex);
    }

    // general ghost info
    {
        let ghost_info = Label::new(|g: &Wghost, _env: &_| {
            format!(
                "{} {:?}\n\n🕵️ {}\n\n{}",
                g.0,
                g.0,
                g.0.evidences().map(|e| e.to_string()).collect::<String>(),
                g.0.description()
            )
        })
        .with_text_size(18.0)
        .padding((0., 13., 0., 0.))
        .lens(AppData::selected);

        let mut ghost_flex = Flex::row();
        ghost_flex.add_flex_child(ghost_info, 1.0);

        root.add_flex_child(ghost_flex, 1.0);
    }

    root
}

pub fn run() {
    use druid::{AppLauncher, LocalizedString, WindowDesc};

    let main_window = WindowDesc::new(ui_builder).title(
        LocalizedString::new("app-window-title").with_placeholder("Phasmo Evidence Tracker"),
    );
    // .with_min_size((610.0, 420.0));

    // Set our initial data
    let data = AppData::default();
    AppLauncher::with_window(main_window)
        // changes the default theme
        .configure_env(|env: &mut _, _t: &AppData| {
            use druid::{theme, Color};

            // let base03 = &Color::from_rgba32_u32(0x002b36ff);
            let base02 = &Color::from_rgba32_u32(0x073642ff);
            // let base01 = &Color::from_rgba32_u32(0x586e75ff);
            // let base00 = &Color::from_rgba32_u32(0x657b83ff);

            // let base0 = &Color::from_rgba32_u32(0x839496ff);
            let base1 = &Color::from_rgba32_u32(0x93a1a1ff);
            // let base2 = &Color::from_rgba32_u32(0xeee8d5ff);
            // let base3 = &Color::from_rgba32_u32(0xfdf6e3ff);

            // let yellow = &Color::from_rgba32_u32(0xb58900ff);
            // let orange = &Color::from_rgba32_u32(0xcb4b16ff);
            let red = &Color::from_rgba32_u32(0xdc322fff);
            // let magenta = &Color::from_rgba32_u32(0xd33682ff);
            // let violet = &Color::from_rgba32_u32(0x6c71c4ff);
            // let blue = &Color::from_rgba32_u32(0x268bd2ff);
            // let cyan = &Color::from_rgba32_u32(0x2aa198ff);
            // let green = &Color::from_rgba32_u32(0x859900ff);

            env.set(theme::LABEL_COLOR, base02.clone());

            // env.set(theme::PRIMARY_LIGHT, blue.clone());
            // env.set(theme::PRIMARY_DARK, yellow.clone());

            // env.set(theme::FOREGROUND_LIGHT, red.clone());
            // env.set(theme::FOREGROUND_DARK, green.clone());

            // env.set(theme::BACKGROUND_LIGHT, red.clone());
            // env.set(theme::BACKGROUND_DARK, green.clone());

            env.set(theme::SELECTION_COLOR, red.clone());

            env.set(theme::BUTTON_LIGHT, base1.clone());
            env.set(theme::BUTTON_DARK, base1.clone());

            env.set(theme::WINDOW_BACKGROUND_COLOR, base1.clone());

            env.set(theme::TEXT_SIZE_NORMAL, 22.0f64);
            env.set(theme::TEXT_SIZE_LARGE, 30.0f64);
        })
        .use_simple_logger()
        .launch(data)
        .expect("launch failed")
}
