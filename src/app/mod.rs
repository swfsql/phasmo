#![allow(clippy::unnecessary_operation)]

pub mod cursor;
pub mod overlay;

// use cursor::Cursor;
use crate::phasmo;
use druid::{
    im::Vector,
    widget::{Button, Flex, Label},
    Data, Lens, Widget, WidgetExt,
};

#[derive(Debug, Clone, PartialEq, Data, Lens)]
pub struct AppData {
    evidence_status: Vector<EvidenceStatus>,
    selected: Wghost,
}

struct Delegate {}

const SELECT_GHOST: druid::Selector<phasmo::Ghost> = druid::Selector::new("select_ghost");

impl druid::AppDelegate<AppData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppData,
        _env: &druid::Env,
    ) -> bool {
        if let Some(ghost) = cmd.get(SELECT_GHOST) {
            data.selected = Wghost(*ghost);
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Data)]
pub struct Wghost(#[data(same_fn = "PartialEq::eq")] pub phasmo::Ghost);

#[derive(Debug, Clone, PartialEq, Data)]
#[repr(C)]
pub enum EvidenceStatus {
    Unknown,
    Found,
    Forbidden,
}

impl Default for AppData {
    fn default() -> Self {
        let evidence_status: Vector<_> = phasmo::EVIDENCES
            .iter()
            .map(|_e| EvidenceStatus::default())
            .collect();
        #[allow(clippy::deref_addrof)]
        let selected = Wghost(phasmo::GHOSTS[0]);
        Self {
            evidence_status,
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
        let required: Vec<_> = phasmo::EVIDENCES
            .iter()
            .cloned()
            .zip(self.evidence_status.iter())
            .filter_map(|(evd, status)| {
                if let EvidenceStatus::Found = status {
                    Some(evd)
                } else {
                    None
                }
            })
            .collect();
        let forbid: Vec<_> = phasmo::EVIDENCES
            .iter()
            .cloned()
            .zip(self.evidence_status.iter())
            .filter_map(|(evd, status)| {
                if let EvidenceStatus::Forbidden = status {
                    Some(evd)
                } else {
                    None
                }
            })
            .collect();

        let ghosts =
            Ghost::filter_by_required_evidences(phasmo::GHOSTS.iter().cloned(), required.as_ref());
        Ghost::filter_by_forbid_evidences(ghosts.into_iter(), forbid.as_ref())
    }
}

pub fn ui_builder() -> impl Widget<AppData> {
    let mut root = Flex::column();

    // let mut summary_ghosts_flex = Flex::row();
    let mut summary_features_flex = Flex::row();

    // summary
    {
        let mut ghost_icons = Flex::row();

        for g in phasmo::GHOSTS.iter().cloned() {
            let w = Button::new(move |evd_status: &Vector<EvidenceStatus>, _env: &_| {
                let (required, forbid): (Vec<_>, Vec<_>) = phasmo::EVIDENCES
                    .iter()
                    .cloned()
                    .zip(evd_status.iter())
                    .filter(|(_evd, status)| !matches!(status, EvidenceStatus::Unknown))
                    .partition(|(_evd, status)| matches!(status, EvidenceStatus::Found));
                let required: Vec<_> = required.into_iter().map(|(evd, _status)| evd).collect();
                let forbid: Vec<_> = forbid.into_iter().map(|(evd, _status)| evd).collect();
                if g.is_valid(required.as_ref(), forbid.as_ref()) {
                    g.to_string()
                } else {
                    String::new()
                }
            })
            .on_click(
                move |ctx: &mut druid::EventCtx, _e: &mut Vector<EvidenceStatus>, _env: &_| {
                    ctx.submit_command(druid::Command::new(SELECT_GHOST, g), None);
                },
            )
            .expand_width()
            .lens(AppData::evidence_status);
            ghost_icons.add_flex_child(w, 1.0);
        }
        root.add_flex_child(ghost_icons, 1.0);

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

            caution_str + "/ " + &useful_str
        });

        summary_features_flex.add_flex_child(feature_icons, 1.0);
        root.add_flex_child(summary_features_flex.padding((0., 10., 0., 10.)), 1.0);
    };

    // evidences
    {
        // let evidence_status = || {
        //     Label::new(|evidence: &EvidenceState, _env: &_| {
        //         match evidence.status {
        //             // ☐
        //             // ❓ ➖ ❔
        //             EvidenceStatus::Unknown => "➖",
        //             // ☑
        //             // ✔️
        //             EvidenceStatus::Found => "✔️",
        //             // ☒
        //             // ❌
        //             EvidenceStatus::Forbidden => "❌",
        //         }
        //         .to_string()
        //     })
        // };

        let evidence_status = || {
            Label::new(|evidence: &EvidenceStatus, _env: &_| {
                match evidence {
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

        let mut evidence_icons = Flex::row();
        for (i, e) in phasmo::EVIDENCES.iter().cloned().enumerate() {
            use druid::LensExt;
            let l = (AppData::evidence_status).index(i);
            let mut w = Flex::column();
            let button = Button::new(move |_evd: &EvidenceStatus, _env: &_| e.to_string())
                .on_click(
                    move |_ctx: &mut druid::EventCtx, e: &mut EvidenceStatus, _env: &_| {
                        e.next_status();
                    },
                )
                .expand_width()
                .lens(l);
            let l = (AppData::evidence_status).index(i);
            let status = evidence_status().lens(l);
            w.add_flex_child(button, 1.0);
            w.add_flex_child(status, 1.0);
            evidence_icons.add_flex_child(w, 1.0);
        }
        root.add_flex_child(evidence_icons, 1.0);

        /*
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
        */
    }

    // reset button
    {
        let mut reset_flex = Flex::row();
        let reset_button = Button::new("🔄")
            .on_click(|_ctx, evidences: &mut Vector<EvidenceStatus>, _env: _| {
                for e in evidences.iter_mut() {
                    *e = EvidenceStatus::default();
                }
            })
            .padding((0., 0., 0., 0.))
            .lens(AppData::evidence_status);

        reset_flex.add_flex_child(reset_button, 1.0);
        root.add_flex_child(reset_flex, 1.0);
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
        .padding((0., 18., 0., 0.))
        .lens(AppData::selected);

        let mut ghost_flex = Flex::row();
        ghost_flex.add_flex_child(ghost_info, 1.0);

        root.add_flex_child(ghost_flex, 1.0);
    }

    // cursor overlay
    // {
    //     let cursor = Cursor::default();
    //     overlay::Overlay::new(root, cursor)
    // }
    root.padding((20., 10.0, 20., 10.))
}

pub fn run() {
    use druid::{AppLauncher, LocalizedString, WindowDesc};

    let main_window = WindowDesc::new(ui_builder).title(
        LocalizedString::new("app-window-title").with_placeholder("Phasmo Evidence Tracker"),
    );
    // .with_min_size((610.0, 420.0));

    // Set our initial data
    let delegate = Delegate {};
    let data = AppData::default();
    AppLauncher::with_window(main_window)
        .delegate(delegate)
        // changes the default theme
        .configure_env(|env: &mut _, _t: &AppData| {
            use druid::{theme, Color};

            // tomorrow night-based theme
            let dark_bg = &Color::from_rgba32_u32(0x1D1F21FF);
            let dark_bg2 = &Color::from_rgba32_u32(0x29211AFF);
            let light_fg = &Color::from_rgba32_u32(0xC5C8C6FF);

            // solarized theme
            // let base03 = &Color::from_rgba32_u32(0x002b36ff);
            // let base02 = &Color::from_rgba32_u32(0x073642ff);
            // let base01 = &Color::from_rgba32_u32(0x586e75ff);
            // let base00 = &Color::from_rgba32_u32(0x657b83ff);
            // let base0 = &Color::from_rgba32_u32(0x839496ff);
            // let base1 = &Color::from_rgba32_u32(0x93a1a1ff);
            // let base2 = &Color::from_rgba32_u32(0xeee8d5ff);
            // let base3 = &Color::from_rgba32_u32(0xfdf6e3ff);
            // let yellow = &Color::from_rgba32_u32(0xb58900ff);
            // let orange = &Color::from_rgba32_u32(0xcb4b16ff);
            // let red = &Color::from_rgba32_u32(0xdc322fff);
            // let magenta = &Color::from_rgba32_u32(0xd33682ff);
            // let violet = &Color::from_rgba32_u32(0x6c71c4ff);
            // let blue = &Color::from_rgba32_u32(0x268bd2ff);
            // let cyan = &Color::from_rgba32_u32(0x2aa198ff);
            // let green = &Color::from_rgba32_u32(0x859900ff);

            env.set(theme::LABEL_COLOR, light_fg.clone());

            // env.set(theme::PRIMARY_LIGHT, blue.clone());
            // env.set(theme::PRIMARY_DARK, yellow.clone());
            // env.set(theme::FOREGROUND_LIGHT, red.clone());
            // env.set(theme::FOREGROUND_DARK, green.clone());
            // env.set(theme::BACKGROUND_LIGHT, red.clone());
            // env.set(theme::BACKGROUND_DARK, green.clone());
            // env.set(theme::SELECTION_COLOR, red.clone());

            env.set(theme::BUTTON_LIGHT, dark_bg2.clone());
            env.set(theme::BUTTON_DARK, dark_bg.clone());
            env.set(theme::WINDOW_BACKGROUND_COLOR, dark_bg.clone());
            env.set(theme::TEXT_SIZE_NORMAL, 22.0f64);
            env.set(theme::TEXT_SIZE_LARGE, 30.0f64);
        })
        .use_simple_logger()
        .launch(data)
        .expect("launch failed")
}
