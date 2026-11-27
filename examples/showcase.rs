//! Interactive ModernRust Showcase — test themes, GPU vector icons, animated menus,
//! prompt boxes, semantic toasts, switches, segmented pills, and Fluent side panels live.

use eframe::egui::{self, vec2, CornerRadius, Frame, Margin, RichText, Stroke};
use modernrust::{
    chip, edge_collapse_tab, fluent_switch, icons, menu_button, modern_button, progress_bar,
    section_card, segmented_pill, side_panel_header, ButtonVariant, Icon, MenuEntry, PromptBox,
    PromptOutcome, Role, ThemePreset, ToastStack,
};

struct ShowcaseApp {
    preset: ThemePreset,
    pdf_mode: usize,
    dark_switch: bool,
    diff_first_page: bool,
    auto_save: bool,
    panel_collapsed: bool,
    show_grid: bool,
    icon_size: f32,
    toasts: ToastStack,
    selected_chips: Vec<&'static str>,
    progress: f32,
}

impl Default for ShowcaseApp {
    fn default() -> Self {
        let preset = match std::env::var("MODERNRUST_THEME").unwrap_or_default().to_ascii_lowercase().as_str() {
            "fluent_dark" | "fluentdark" | "dark" => ThemePreset::FluentDark,
            "cosmo" => ThemePreset::Cosmo,
            "darkly" => ThemePreset::Darkly,
            "cyborg" => ThemePreset::Cyborg,
            "minty" => ThemePreset::Minty,
            "dracula" => ThemePreset::Dracula,
            "obsidian_paper" | "obsidianpaper" | "obsidian" => ThemePreset::ObsidianPaper,
            _ => ThemePreset::FluentLight,
        };
        let dark_switch = preset.palette().dark;
        Self {
            preset,
            pdf_mode: 0,
            dark_switch,
            diff_first_page: true,
            auto_save: true,
            panel_collapsed: false,
            show_grid: true,
            icon_size: 32.0,
            toasts: ToastStack::default(),
            selected_chips: vec!["Page X of Y", "Strictly Confidential"],
            progress: 0.68,
        }
    }
}

impl eframe::App for ShowcaseApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let now = ctx.input(|i| i.time);
        let p = self.preset.palette();
        p.apply_to_egui(&ctx);

        // Top Fluent Menubar with animated dropdowns & gliding [ Read | Edit ] pill
        egui::Panel::top("mr-showcase-top")
            .frame(Frame::new().fill(p.surface).inner_margin(Margin::symmetric(14, 8)).stroke(Stroke::new(1.0, p.divider)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("ModernRust").strong().size(15.0).color(p.primary));
                    ui.add_space(10.0);

                    let file_entries = [
                        MenuEntry::Section("Document".into()),
                        MenuEntry::command("save", "Save Document").with_icon(Icon::Save).with_shortcut("Ctrl+S"),
                        MenuEntry::command("pdf", "Export PDF").with_icon(Icon::Pdf).with_shortcut("Ctrl+Shift+P"),
                        MenuEntry::command("print", "Print…").with_icon(Icon::Print).with_shortcut("Ctrl+P"),
                        MenuEntry::Separator,
                        MenuEntry::command("delete", "Delete Draft").with_icon(Icon::Trash).with_role(Role::Danger),
                    ];
                    if let Some(cmd) = menu_button(ui, &p, "File", 230.0, &file_entries) {
                        match cmd {
                            "save" => self.toasts.push("Saved", "Annual_Report.wpx updated.", Role::Success, now),
                            "pdf" => self.toasts.push("Exported PDF", "Saved searchable vector PDF.", Role::Info, now),
                            "print" => self.toasts.push("Print", "Sent to default printer.", Role::Primary, now),
                            "delete" => self.toasts.push("Draft Deleted", "Moved to recycle bin.", Role::Danger, now),
                            _ => {}
                        }
                    }

                    let insert_entries = [
                        MenuEntry::command("table", "Insert Table").with_icon(Icon::Table).with_shortcut("Ctrl+T"),
                        MenuEntry::command("hf", "Headers & Footers").with_icon(Icon::HeaderFooter),
                        MenuEntry::command("comment", "Add Comment").with_icon(Icon::Comment).with_shortcut("Ctrl+Alt+M"),
                        MenuEntry::Separator,
                        MenuEntry::check("grid", "Show Icon Grid", self.show_grid),
                    ];
                    if let Some(cmd) = menu_button(ui, &p, "Insert", 230.0, &insert_entries) {
                        if cmd == "grid" {
                            self.show_grid = !self.show_grid;
                        } else {
                            self.toasts.push("Insert Action", format!("Triggered `{cmd}`"), Role::Primary, now);
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_id_salt("mr-theme-picker")
                            .selected_text(self.preset.label())
                            .width(135.0)
                            .show_ui(ui, |ui| {
                                for &pr in ThemePreset::ALL {
                                    if ui.selectable_value(&mut self.preset, pr, pr.label()).clicked() {
                                        self.dark_switch = pr.palette().dark;
                                    }
                                }
                            });
                        ui.add_space(6.0);
                        if segmented_pill(ui, &p, "mode", &["Read", "Edit"], &mut self.pdf_mode) {
                            let m = if self.pdf_mode == 0 { "Read" } else { "Edit" };
                            self.toasts.push(format!("{m} Mode"), "Switched document mode smoothly.", Role::Info, now);
                        }
                    });
                });
            });

        // Right-hand collapsible ModernRust Side Panel
        if !self.panel_collapsed {
            egui::Panel::right("mr-showcase-side")
                .resizable(true)
                .default_size(275.0)
                .min_size(240.0)
                .frame(Frame::new().fill(p.surface).inner_margin(Margin::same(14)).stroke(Stroke::new(1.0, p.divider)))
                .show(ui, |ui| {
                    if side_panel_header(ui, &p, "Headers & Footers", Some("Live")) {
                        self.panel_collapsed = true;
                    }
                    ui.spacing_mut().item_spacing.y = 10.0;
                    if fluent_switch(ui, &p, "Different first page", &mut self.diff_first_page).clicked() {}
                    if fluent_switch(ui, &p, "Auto-save changes", &mut self.auto_save).clicked() {}
                    if fluent_switch(ui, &p, "Dark mode", &mut self.dark_switch).clicked() {
                        self.preset = if self.dark_switch { ThemePreset::FluentDark } else { ThemePreset::FluentLight };
                    }
                    ui.separator();
                    ui.label(RichText::new("Quick Insert Chips").strong().size(12.0).color(p.fg_muted));
                    ui.horizontal_wrapped(|ui| {
                        for tag in ["Page #", "Page X of Y", "Strictly Confidential", "Draft", "Date", "Title"] {
                            let is_sel = self.selected_chips.contains(&tag);
                            if chip(ui, &p, tag, is_sel).clicked() {
                                if let Some(pos) = self.selected_chips.iter().position(|&t| t == tag) {
                                    self.selected_chips.remove(pos);
                                } else {
                                    self.selected_chips.push(tag);
                                }
                            }
                        }
                    });
                    ui.separator();
                    ui.label(RichText::new("Export Progress").strong().size(12.0).color(p.fg_muted));
                    progress_bar(ui, &p, self.progress, Role::Primary, true);
                    ui.add(egui::Slider::new(&mut self.progress, 0.0..=1.0).show_value(false));
                });
        }
        egui::Panel::right("mr-showcase-edge")
            .resizable(false)
            .default_size(18.0)
            .min_size(18.0)
            .max_size(18.0)
            .frame(Frame::new().fill(p.bg).inner_margin(Margin::ZERO))
            .show(ui, |ui| {
                edge_collapse_tab(ui, &p, &mut self.panel_collapsed, "Inspector");
            });

        // Main Showcase Canvas
        egui::CentralPanel::default()
            .frame(Frame::new().fill(p.bg).inner_margin(Margin::same(18)))
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 14.0;

                    // 1. Prompt Boxes & Semantic Toasts
                    section_card(ui, &p, Some("Prompt Box & Semantic Toasts"), |ui| {
                        let outcome = PromptBox::new(
                            "Save changes before closing?",
                            "Your edits to Annual_Report.pdf haven't been baked to disk yet.",
                        )
                        .role(Role::Warning)
                        .confirm_label("Save PDF")
                        .secondary_label("Don't Save")
                        .cancel_label("Cancel")
                        .show_inline(ui, &p);

                        match outcome {
                            PromptOutcome::Confirm => self.toasts.push("Saved PDF", "Annual_Report.pdf exported.", Role::Success, now),
                            PromptOutcome::Secondary => self.toasts.push("Discarded", "Closed without saving.", Role::Warning, now),
                            PromptOutcome::Cancel => self.toasts.push("Cancelled", "Returned to document.", Role::Info, now),
                            PromptOutcome::Pending => {}
                        }

                        ui.add_space(10.0);
                        ui.horizontal_wrapped(|ui| {
                            if modern_button(ui, &p, "Success Toast", Role::Success, ButtonVariant::Filled, Some(Icon::Check)).clicked() {
                                self.toasts.push("Saved!", "All changes synced cleanly.", Role::Success, now);
                            }
                            if modern_button(ui, &p, "Info Toast", Role::Info, ButtonVariant::Pill, Some(Icon::Info)).clicked() {
                                self.toasts.push("Up to Date", "Syllabary v0.2.2 is the latest version.", Role::Info, now);
                            }
                            if modern_button(ui, &p, "Warning Toast", Role::Warning, ButtonVariant::Outline, Some(Icon::Warning)).clicked() {
                                self.toasts.push("Unsaved Changes", "Press Ctrl+S to save your PDF.", Role::Warning, now);
                            }
                            if modern_button(ui, &p, "Danger Toast", Role::Danger, ButtonVariant::Filled, Some(Icon::Trash)).clicked() {
                                self.toasts.push("Block Removed", "Press Ctrl+Z to undo.", Role::Danger, now);
                            }
                        });
                    });

                    // 2. GPU Procedural Vector Icon Suite
                    if self.show_grid {
                        let grid_title = format!("Persistent SVG Icons ({} Built-In)", Icon::ALL.len());
                        section_card(ui, &p, Some(&grid_title), |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Preview size:").small().color(p.fg_muted));
                                for sz in [18.0_f32, 24.0, 32.0, 40.0] {
                                    if chip(ui, &p, &format!("{sz:.0} px"), (self.icon_size - sz).abs() < 0.5).clicked() {
                                        self.icon_size = sz;
                                    }
                                }
                            });
                            ui.add_space(10.0);
                            egui::Grid::new("mr-icon-grid").spacing(vec2(12.0, 12.0)).show(ui, |ui| {
                                for (i, &ic) in Icon::ALL.iter().enumerate() {
                                    Frame::new()
                                        .fill(p.surface_alt)
                                        .stroke(Stroke::new(1.0, p.divider))
                                        .corner_radius(CornerRadius::same(p.radius_md))
                                        .inner_margin(Margin::symmetric(12, 12))
                                        .show(ui, |ui| {
                                            ui.set_min_width(86.0);
                                            ui.vertical_centered(|ui| {
                                                icons::widget(ui, ic, self.icon_size, p.fg, p.primary);
                                                ui.add_space(6.0);
                                                ui.label(RichText::new(ic.name()).size(11.0).color(p.fg_muted));
                                            });
                                        });
                                    if (i + 1) % 7 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });
                        });
                    }
                });
            });

        self.toasts.show(&ctx, &p);
    }
}

fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ModernRust — Interactive UI Showcase")
            .with_inner_size([1120.0, 740.0]),
        ..Default::default()
    };
    eframe::run_native("ModernRust Showcase", opts, Box::new(|_cc| Ok(Box::new(ShowcaseApp::default()))))
}
