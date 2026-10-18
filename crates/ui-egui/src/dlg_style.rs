//! The dialog design language, from the Figma file "DesignCraft UI": a 12 pt rounded frame with a
//! 30 pt title bar, 25 pt controls with an 8 pt radius, a green accent, and 12 pt Source Sans text.
//!
//! Dialogs opt in one at a time ([`is_styled`]); the others keep the classic look until they are
//! converted. Colours come from [`crate::theme::DialogTokens`]; nothing here hard-codes one.

use egui::{Align2, Color32, CornerRadius, FontId, Margin, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui, pos2, vec2};

use crate::theme::{DialogTokens, Tokens, bold};

/// Height of fields, buttons, dropdowns, toggle buttons and checkboxes.
pub const CONTROL_H: f32 = 25.0;
/// Corner radius of fields, buttons, dropdowns and toggle buttons.
pub const RADIUS: u8 = 8;
/// Corner radius of the dialog frame.
pub const FRAME_RADIUS: u8 = 12;
/// Height of the title bar.
pub const TITLE_H: f32 = 30.0;
/// Width of the stepper pill beside a number field, and the gap between them.
pub const STEPPER_W: f32 = 20.0;
pub const STEPPER_GAP: f32 = 5.0;
/// Width of the switch track and its thumb size.
pub const SWITCH_W: f32 = 50.0;
pub const SWITCH_H: f32 = 24.0;
const THUMB: f32 = 18.0;
/// Space between a switch or checkbox and its caption.
const CAPTION_GAP: f32 = 10.0;
/// Text inside buttons and toggle buttons: 12 pt, with this much room each side.
pub const TEXT_SIZE: f32 = 12.0;
const BUTTON_PAD: f32 = 12.0;
/// Width the label column of a form needs (Figma: 19 pt margin, labels, then the controls at 156).
pub const LABEL_COL_W: f32 = 122.0;
/// Space between neighbouring controls in a row.
pub const GAP: f32 = 15.0;
/// Space between rows of 25 pt controls.
pub const ROW_GAP: f32 = 14.0;
/// Space around the content of a dialog.
pub const MARGIN: Margin = Margin { left: 19, right: 25, top: 27, bottom: 0 };
pub const FOOTER_MARGIN: Margin = Margin { left: 19, right: 25, top: 0, bottom: 25 };

/// Whether the dialog `id` has been converted to the new design language.
pub fn is_styled(id: &str) -> bool {
    matches!(id, "justification")
}

/// Whether a converted dialog is being drawn. The shared helpers (`check`, `combo`, number
/// fields) read it to draw themselves in this design language.
pub fn active(ctx: &egui::Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(egui::Id::new("dlg_style_active")).unwrap_or(false))
}

/// Marks a converted dialog as being drawn (or done).
pub fn set_active(ctx: &egui::Context, on: bool) {
    ctx.data_mut(|d| d.insert_temp(egui::Id::new("dlg_style_active"), on));
}

/// Gives the egui widgets inside a converted dialog the new look, so a dialog needs no per-widget
/// work: buttons, combo boxes, text fields, sliders and drag values get the 25 pt height, 8 pt
/// corners, field fill and green focus outline; labels are muted 12 pt text.
pub fn apply_scope(ui: &mut Ui) {
    let d = tokens(ui.ctx());
    let style = ui.style_mut();
    for (text_style, size) in [
        (egui::TextStyle::Small, 11.0),
        (egui::TextStyle::Body, TEXT_SIZE),
        (egui::TextStyle::Button, TEXT_SIZE),
        (egui::TextStyle::Monospace, TEXT_SIZE),
    ] {
        if let Some(font) = style.text_styles.get_mut(&text_style) {
            font.size = size;
        }
    }
    style.spacing.interact_size.y = CONTROL_H;
    style.spacing.button_padding = vec2(BUTTON_PAD, 4.0);
    style.spacing.combo_height = 320.0;
    let v = &mut style.visuals;
    v.override_text_color = None;
    v.extreme_bg_color = d.field;
    v.faint_bg_color = d.field_hover;
    v.window_fill = d.frame;
    v.selection.bg_fill = d.accent_pressed;
    v.selection.stroke = Stroke::new(1.0, d.accent);
    v.menu_corner_radius = radius();
    let none = Stroke::NONE;
    let widgets = &mut v.widgets;
    widgets.noninteractive.fg_stroke = Stroke::new(1.0, d.label);
    widgets.noninteractive.bg_stroke = Stroke::new(1.0, d.field);
    for (w, fill, stroke, fg) in [
        (&mut widgets.inactive, d.field, none, d.text),
        (&mut widgets.hovered, d.field_hover, none, d.text),
        (&mut widgets.active, d.field_hover, Stroke::new(1.0, d.accent), d.text),
        (&mut widgets.open, d.field, Stroke::new(1.0, d.accent), d.text),
    ] {
        w.bg_fill = fill;
        w.weak_bg_fill = fill;
        w.bg_stroke = stroke;
        w.fg_stroke = Stroke::new(1.0, fg);
        w.corner_radius = radius();
        w.expansion = 0.0;
    }
    widgets.noninteractive.corner_radius = radius();
}

fn tokens(ctx: &egui::Context) -> DialogTokens {
    Tokens::get(ctx).dialog
}

/// Regular text.
pub fn text_font() -> FontId {
    FontId::proportional(TEXT_SIZE)
}

/// Bold text (button labels, field values).
pub fn strong_font() -> FontId {
    bold(TEXT_SIZE)
}

fn radius() -> CornerRadius {
    CornerRadius::same(RADIUS)
}

/// The window frame of a converted dialog: 12 pt corners, a slight drop shadow, no padding (the
/// title bar runs edge to edge).
pub fn frame(ctx: &egui::Context) -> egui::Frame {
    let d = tokens(ctx);
    egui::Frame::new().fill(d.frame).corner_radius(CornerRadius::same(FRAME_RADIUS)).inner_margin(Margin::ZERO).shadow(egui::epaint::Shadow {
        offset: [0, 6],
        blur: 20,
        spread: 0,
        color: d.shadow,
    })
}

/// The title bar: the title on the left (right in a right-to-left language), a close mark opposite,
/// and a drag area over the whole bar. Returns true when the close mark was clicked.
pub fn title_bar(ui: &mut Ui, title: &str, width: f32, rtl: bool, instance: egui::Id) -> bool {
    let d = tokens(ui.ctx());
    let (bar, _) = ui.allocate_exact_size(vec2(width, TITLE_H), Sense::hover());
    let top = CornerRadius { nw: FRAME_RADIUS, ne: FRAME_RADIUS, sw: 0, se: 0 };
    ui.painter().rect_filled(bar, top, d.title_bar);
    let (anchor, x) = if rtl { (Align2::RIGHT_CENTER, bar.max.x - 19.0) } else { (Align2::LEFT_CENTER, bar.min.x + 19.0) };
    crate::rtl::paint(ui.painter(), pos2(x, bar.center().y), anchor, title, bold(15.0), d.title_text);
    // The drag area first, so the close mark above it keeps its own clicks.
    crate::modal_drag::title_drag(ui, instance, bar);
    let cx = if rtl { bar.min.x + 20.0 } else { bar.max.x - 20.0 };
    let hit = Rect::from_center_size(pos2(cx, bar.center().y), vec2(24.0, 24.0));
    let resp = ui.interact(hit, instance.with("close"), Sense::click());
    let color = if resp.hovered() || resp.has_focus() { d.destructive } else { d.label };
    let c = hit.center();
    let stroke = Stroke::new(1.5, color);
    ui.painter().line_segment([c + vec2(-4.5, -4.5), c + vec2(4.5, 4.5)], stroke);
    ui.painter().line_segment([c + vec2(-4.5, 4.5), c + vec2(4.5, -4.5)], stroke);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Close"));
    resp.clicked()
}

/// A row label (12 pt, muted), vertically centred in a 25 pt row and `LABEL_COL_W` wide.
pub fn row_label(ui: &mut Ui, text: &str) {
    let d = tokens(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(LABEL_COL_W, CONTROL_H), Sense::hover());
    let rtl = ui.layout().prefer_right_to_left();
    let (anchor, x) = if rtl { (Align2::RIGHT_CENTER, r.max.x) } else { (Align2::LEFT_CENTER, r.min.x) };
    crate::rtl::paint(ui.painter(), pos2(x, r.center().y), anchor, text, text_font(), d.label);
}

/// A column caption above a group of fields (12 pt, muted).
pub fn caption(ui: &mut Ui, text: &str, width: f32) {
    let d = tokens(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(width, 17.0), Sense::hover());
    crate::rtl::paint(ui.painter(), r.left_center(), Align2::LEFT_CENTER, text, text_font(), d.label);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonKind {
    /// The light button; its text and outline turn red on hover (Cancel, Close).
    Cancel,
    /// The green button (OK).
    Accent,
}

/// A 25 pt button, at least `min_width` wide and wide enough for its label.
pub fn button(ui: &mut Ui, label: &str, kind: ButtonKind, min_width: f32) -> Response {
    let d = tokens(ui.ctx());
    let label = label.trim();
    let enabled = ui.is_enabled();
    let galley = crate::rtl::plain(ui.ctx(), label, strong_font(), d.text);
    let width = (galley.size().x + 2.0 * BUTTON_PAD).max(min_width);
    let (r, resp) = ui.allocate_exact_size(vec2(width, CONTROL_H), if enabled { Sense::click() } else { Sense::hover() });
    let hovered = enabled && resp.hovered();
    let pressed = enabled && resp.is_pointer_button_down_on();
    let (fill, mut text, mut border) = match kind {
        ButtonKind::Cancel => (if pressed { d.light_button_pressed } else { d.light_button }, d.on_light, None),
        ButtonKind::Accent => (
            if pressed {
                d.accent_pressed
            } else if hovered {
                d.accent_hover
            } else {
                d.accent
            },
            d.on_accent,
            None,
        ),
    };
    if kind == ButtonKind::Cancel && hovered {
        text = d.destructive;
        border = Some(d.destructive);
    }
    if enabled && resp.has_focus() {
        border = Some(if kind == ButtonKind::Accent { d.accent_hover } else { d.accent });
    }
    let (fill, text) = if enabled { (fill, text) } else { (d.field, d.text_disabled) };
    ui.painter().rect_filled(r, radius(), fill);
    if let Some(c) = border {
        ui.painter().rect_stroke(r, radius(), Stroke::new(1.0, c), StrokeKind::Inside);
    }
    crate::rtl::paint(ui.painter(), r.center(), Align2::CENTER_CENTER, label, strong_font(), text);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));
    resp
}

/// One option of a group of exclusive options (Paragraph Composer | Single-Line Composer): the
/// chosen one has a green outline.
pub fn toggle_button(ui: &mut Ui, label: &str, selected: bool) -> Response {
    let d = tokens(ui.ctx());
    let enabled = ui.is_enabled();
    let galley = crate::rtl::plain(ui.ctx(), label, text_font(), d.text);
    let (r, resp) =
        ui.allocate_exact_size(vec2(galley.size().x + 2.0 * BUTTON_PAD, CONTROL_H), if enabled { Sense::click() } else { Sense::hover() });
    let fill = if enabled && resp.hovered() && !selected { d.field_hover } else { d.field };
    ui.painter().rect_filled(r, radius(), fill);
    if selected || (enabled && resp.has_focus()) {
        ui.painter().rect_stroke(r, radius(), Stroke::new(1.0, d.accent), StrokeKind::Inside);
    }
    crate::rtl::paint(ui.painter(), r.center(), Align2::CENTER_CENTER, label, text_font(), if enabled { d.text } else { d.text_disabled });
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, enabled, selected, label));
    resp
}

/// A switch with its caption on the right. `mixed` shows the thumb in the middle (the selected
/// objects differ); a click then turns the switch on or off like any other.
pub fn switch(ui: &mut Ui, on: &mut bool, mixed: bool, caption: &str) -> Response {
    let d = tokens(ui.ctx());
    let enabled = ui.is_enabled();
    let galley = crate::rtl::plain(ui.ctx(), caption, text_font(), d.label);
    let extra = if caption.is_empty() { 0.0 } else { CAPTION_GAP + galley.size().x };
    let (r, mut resp) = ui.allocate_exact_size(vec2(SWITCH_W + extra, SWITCH_H), if enabled { Sense::click() } else { Sense::hover() });
    if resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }
    let track = Rect::from_min_size(r.min, vec2(SWITCH_W, SWITCH_H));
    let t = ui.ctx().animate_bool(resp.id, *on);
    let travel = SWITCH_W - 2.0 * 3.0 - THUMB;
    let x = if mixed { 3.0 + travel / 2.0 } else { 3.0 + travel * t };
    let (thumb_color, track_fill) = if !enabled {
        (d.text_disabled, d.field)
    } else if *on && !mixed {
        (d.accent, d.field)
    } else {
        (d.light_button, d.field)
    };
    ui.painter().rect_filled(track, CornerRadius::same(12), track_fill);
    if enabled && resp.has_focus() {
        ui.painter().rect_stroke(track, CornerRadius::same(12), Stroke::new(1.0, d.accent), StrokeKind::Inside);
    }
    ui.painter().circle_filled(pos2(track.min.x + x + THUMB / 2.0, track.center().y), THUMB / 2.0, thumb_color);
    if !caption.is_empty() {
        crate::rtl::paint(ui.painter(), pos2(track.max.x + CAPTION_GAP, r.center().y), Align2::LEFT_CENTER, caption, text_font(), d.label);
    }
    let on_now = *on && !mixed;
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, enabled, on_now, caption));
    resp
}

/// A checkbox: a 25 pt box with a green square when on, a light bar when mixed, then the caption.
pub fn checkbox(ui: &mut Ui, on: &mut bool, mixed: bool, caption: &str) -> Response {
    let d = tokens(ui.ctx());
    let enabled = ui.is_enabled();
    let galley = crate::rtl::plain(ui.ctx(), caption, text_font(), d.label);
    let extra = if caption.is_empty() { 0.0 } else { 9.0 + galley.size().x };
    let (r, mut resp) = ui.allocate_exact_size(vec2(CONTROL_H + extra, CONTROL_H), if enabled { Sense::click() } else { Sense::hover() });
    if resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }
    let b = Rect::from_min_size(r.min, vec2(CONTROL_H, CONTROL_H));
    let fill = if enabled && resp.hovered() { d.field_hover } else { d.field };
    ui.painter().rect_filled(b, CornerRadius::same(6), fill);
    if enabled && resp.has_focus() {
        ui.painter().rect_stroke(b, CornerRadius::same(6), Stroke::new(1.0, d.accent), StrokeKind::Inside);
    }
    if mixed {
        ui.painter().rect_filled(
            Rect::from_center_size(b.center(), vec2(15.0, 7.0)),
            CornerRadius::same(4),
            if enabled { d.light_button } else { d.text_disabled },
        );
    } else if *on {
        ui.painter().rect_filled(
            Rect::from_center_size(b.center(), vec2(15.0, 15.0)),
            CornerRadius::same(4),
            if enabled { d.accent } else { d.text_disabled },
        );
    }
    if !caption.is_empty() {
        crate::rtl::paint(ui.painter(), pos2(b.max.x + 9.0, r.center().y), Align2::LEFT_CENTER, caption, text_font(), d.label);
    }
    let on_now = *on && !mixed;
    resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, enabled, on_now, caption));
    resp
}

/// A dropdown box: the current choice, a chevron on the right, a green outline while open or
/// focused. Open a list on it with [`dropdown_list`].
pub fn dropdown(ui: &mut Ui, text: &str, width: f32, open: bool) -> Response {
    let d = tokens(ui.ctx());
    let enabled = ui.is_enabled();
    let (r, resp) = ui.allocate_exact_size(vec2(width, CONTROL_H), if enabled { Sense::click() } else { Sense::hover() });
    let fill = if enabled && resp.hovered() { d.field_hover } else { d.field };
    ui.painter().rect_filled(r, radius(), fill);
    if enabled && (open || resp.has_focus()) {
        ui.painter().rect_stroke(r, radius(), Stroke::new(1.0, d.accent), StrokeKind::Inside);
    }
    let color = if enabled { d.text } else { d.text_disabled };
    let clip = Rect::from_min_max(r.min, pos2(r.max.x - 22.0, r.max.y));
    crate::rtl::paint(&ui.painter().with_clip_rect(clip), pos2(r.min.x + 12.0, r.center().y), Align2::LEFT_CENTER, text, text_font(), color);
    let c = pos2(r.max.x - 14.0, r.center().y);
    let (a, b) = if open { (2.0, -2.0) } else { (-2.0, 2.0) };
    ui.painter().add(Shape::line(
        vec![c + vec2(-4.0, a), c + vec2(0.0, b), c + vec2(4.0, a)],
        Stroke::new(1.0, if enabled { d.label } else { d.text_disabled }),
    ));
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ComboBox, enabled, text));
    resp
}

/// A dropdown that lists `items` (rows of 25 pt, the chosen one marked with a green dot) and
/// returns the index picked.
pub fn dropdown_list(ui: &mut Ui, id_salt: &str, text: &str, width: f32, items: &[String], current: Option<usize>) -> Option<usize> {
    let id = ui.id().with(("dlg_dropdown", id_salt));
    let open = egui::Popup::is_id_open(ui.ctx(), id.with("popup"));
    let resp = dropdown(ui, text, width, open);
    let mut picked = None;
    let d = tokens(ui.ctx());
    egui::Popup::menu(&resp)
        .id(id.with("popup"))
        .frame(egui::Frame::new().fill(d.field).corner_radius(radius()).inner_margin(Margin::ZERO))
        .width(width)
        .show(|ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            for (i, it) in items.iter().enumerate() {
                let (r, row) = ui.allocate_exact_size(vec2(width, CONTROL_H), Sense::click());
                if row.hovered() || current == Some(i) {
                    ui.painter().rect_filled(r, radius(), d.field_hover);
                }
                crate::rtl::paint(ui.painter(), pos2(r.min.x + 12.0, r.center().y), Align2::LEFT_CENTER, it, text_font(), d.text);
                if current == Some(i) {
                    ui.painter().circle_filled(pos2(r.max.x - 14.0, r.center().y), 3.5, d.accent);
                }
                row.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::SelectableLabel, true, current == Some(i), it));
                if row.clicked() {
                    picked = Some(i);
                    ui.close();
                }
            }
        });
    picked
}

/// The ⌃⌄ pill beside a number field. Returns +1 (up) or −1 (down) when a half was clicked.
pub fn stepper(ui: &mut Ui, rect: Rect, id: egui::Id, enabled: bool) -> Option<f64> {
    let d = tokens(ui.ctx());
    let pill = CornerRadius::same(RADIUS);
    ui.painter().rect_filled(rect, pill, d.field);
    let mut out = None;
    let mid = rect.center().y;
    for (up, half) in [(true, Rect::from_min_max(rect.min, pos2(rect.max.x, mid))), (false, Rect::from_min_max(pos2(rect.min.x, mid), rect.max))] {
        let resp = ui.interact(half, id.with(up), if enabled { Sense::click() } else { Sense::hover() });
        let hot = enabled && resp.hovered();
        if hot {
            let corners =
                if up { CornerRadius { nw: RADIUS, ne: RADIUS, sw: 0, se: 0 } } else { CornerRadius { nw: 0, ne: 0, sw: RADIUS, se: RADIUS } };
            ui.painter().rect_filled(half, corners, d.field_hover);
        }
        let color = if !enabled {
            d.text_disabled
        } else if hot {
            d.text
        } else {
            d.label
        };
        let cx = rect.center().x;
        let pts = if up {
            vec![pos2(cx - 5.0, rect.min.y + 8.0), pos2(cx, rect.min.y + 4.0), pos2(cx + 5.0, rect.min.y + 8.0)]
        } else {
            vec![pos2(cx - 5.0, rect.max.y - 9.0), pos2(cx, rect.max.y - 5.0), pos2(cx + 5.0, rect.max.y - 9.0)]
        };
        ui.painter().add(Shape::line(pts, Stroke::new(1.2, color)));
        if resp.clicked() {
            out = Some(if up { 1.0 } else { -1.0 });
        }
    }
    out
}

/// Colours of a number field in this design language: `(fill, value text, unit text, outline)`.
pub fn field_colors(ctx: &egui::Context, focused: bool, enabled: bool) -> (Color32, Color32, Color32, Option<Color32>) {
    let d = tokens(ctx);
    if !enabled {
        return (d.field, d.text_disabled, d.text_disabled, None);
    }
    (d.field, d.text, d.label, focused.then_some(d.accent))
}

/// The corner radius of a field.
pub fn field_radius() -> CornerRadius {
    radius()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(ctx: &egui::Context, events: Vec<egui::Event>, f: &mut dyn FnMut(&mut Ui)) {
        let raw = egui::RawInput { screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(400.0, 200.0))), events, ..Default::default() };
        let _ = ctx.run_ui(raw, |ui| f(ui));
    }

    fn click_at(p: egui::Pos2) -> Vec<egui::Event> {
        vec![
            egui::Event::PointerMoved(p),
            egui::Event::PointerButton { pos: p, button: egui::PointerButton::Primary, pressed: true, modifiers: Default::default() },
            egui::Event::PointerButton { pos: p, button: egui::PointerButton::Primary, pressed: false, modifiers: Default::default() },
        ]
    }

    fn context() -> egui::Context {
        let ctx = egui::Context::default();
        crate::theme::install_fonts(&ctx, "en");
        crate::theme::apply(&ctx, &Tokens::for_brightness(crate::theme::Brightness::default()));
        ctx
    }

    #[test]
    fn controls_are_25_pt_tall_and_buttons_keep_a_minimum_width() {
        let ctx = context();
        let mut sizes = Vec::new();
        for _ in 0..2 {
            run(&ctx, vec![], &mut |ui| {
                sizes.clear();
                let mut on = false;
                sizes.push(button(ui, "OK", ButtonKind::Accent, 70.0).rect.size());
                sizes.push(button(ui, "A much longer button label", ButtonKind::Cancel, 70.0).rect.size());
                sizes.push(toggle_button(ui, "Paragraph Composer", true).rect.size());
                sizes.push(checkbox(ui, &mut on, false, "").rect.size());
                sizes.push(dropdown(ui, "Paragraph Styles", 155.0, false).rect.size());
                sizes.push(switch(ui, &mut on, false, "").rect.size());
            });
        }
        assert_eq!(sizes[0], vec2(70.0, CONTROL_H), "OK keeps the minimum width");
        assert!(sizes[1].x > 70.0 && sizes[1].y == CONTROL_H, "a long label widens the button: {:?}", sizes[1]);
        for (i, s) in sizes.iter().enumerate().take(5) {
            assert_eq!(s.y, CONTROL_H, "control {i} is 25 pt tall");
        }
        assert_eq!(sizes[5], vec2(SWITCH_W, SWITCH_H));
    }

    #[test]
    fn switch_and_checkbox_toggle_on_click_and_report_the_change() {
        let ctx = context();
        let mut on = false;
        let mut changed = false;
        for events in [vec![], click_at(pos2(30.0, 20.0))] {
            run(&ctx, events, &mut |ui| {
                changed |= switch(ui, &mut on, false, "In Justified Arabic Text").changed();
            });
        }
        assert!(on && changed, "a click turns the switch on");
        let (mut on, mut changed) = (false, false);
        for events in [vec![], click_at(pos2(12.0, 12.0))] {
            run(&ctx, events, &mut |ui| {
                changed |= checkbox(ui, &mut on, true, "Preview").changed();
            });
        }
        assert!(on && changed, "a click on a mixed checkbox turns it on");
    }

    #[test]
    fn a_disabled_button_does_not_click() {
        let ctx = context();
        let mut clicked = false;
        for events in [vec![], click_at(pos2(20.0, 12.0))] {
            run(&ctx, events, &mut |ui| {
                ui.add_enabled_ui(false, |ui| {
                    clicked |= button(ui, "OK", ButtonKind::Accent, 70.0).clicked();
                });
            });
        }
        assert!(!clicked);
    }

    #[test]
    fn the_stepper_reports_up_and_down() {
        let ctx = context();
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(STEPPER_W, CONTROL_H));
        let mut got = Vec::new();
        for (events, expect) in [(click_at(pos2(10.0, 5.0)), 1.0), (click_at(pos2(10.0, 20.0)), -1.0)] {
            run(&ctx, vec![], &mut |ui| {
                stepper(ui, rect, egui::Id::new("st"), true);
            });
            run(&ctx, events, &mut |ui| {
                if let Some(v) = stepper(ui, rect, egui::Id::new("st"), true) {
                    got.push((v, expect));
                }
            });
        }
        assert_eq!(got, vec![(1.0, 1.0), (-1.0, -1.0)]);
    }
}
