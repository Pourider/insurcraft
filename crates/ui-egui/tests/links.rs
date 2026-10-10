//! Links: the GitHub (support) button is one click away everywhere; Help menu, About dialog and
//! home screen open the InsurCraft pages and credit the original PdfCraft project.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use insurcraft_engine::links;
use insurcraft_ui_egui::{Dialog, InsurCraftApp};

fn harness(setup: impl FnOnce(&mut InsurCraftApp) + 'static) -> Harness<'static, InsurCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = InsurCraftApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn github_button_in_the_top_bar_opens_support() {
    let mut h = harness(|_| {});
    h.get_by_label("GitHub").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::SUPPORT));
    assert!(links::SUPPORT.ends_with("/insurcraft/issues"));
}

#[test]
fn home_screen_links() {
    for (label, url) in [
        ("Get help", links::SUPPORT),
        ("InsurCraft web page", links::APP_PAGE),
        ("InsurCraft on GitHub", links::GITHUB),
        ("Original project (PdfCraft)", "https://github.com/storytold/pdfcraft"),
    ] {
        let mut h = harness(|_| {});
        h.get_by_label("Help and feedback");
        h.get_by_label(label).click();
        h.run_steps(2);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(url), "{label}");
    }
}

#[test]
fn about_dialog_shows_the_brand_credit_and_links() {
    let pdf = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";
    // With a document open, so the home screen's own links are not on screen.
    let mut h = harness(move |app| {
        app.open_bytes("one.pdf", None, pdf.to_vec()).unwrap();
        app.dialog = Some(Dialog::About);
    });
    assert!(h.query_all_by_label("InsurCraft").count() >= 1, "the mark (alt text)");
    h.get_by_label("Based on PdfCraft by the ArtCraft team");
    h.get_by_label("Get help").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::SUPPORT));
}

#[test]
fn help_commands_open_each_link() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert_eq!(insurcraft_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
}

/// Runs frames until the node labelled `label` exists and has stopped moving, then returns it.
///
/// The About dialog is a modal centred on its own size, and switching tabs changes that size, so
/// for a few frames afterwards the modal grows and recentres and every control in it moves. A
/// fixed number of frames is a guess at how long that takes: it was too few after the 0.5.0
/// contributors refresh (78 names instead of 5) on some machines, the click on Table landed where
/// the button had been, and the table never opened (#671).
fn settled<'a>(h: &'a mut Harness<'static, InsurCraftApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let mut last = None;
    for _ in 0..60 {
        h.run_steps(1);
        let now = h.query_by_label(label).map(|n| n.rect());
        if now.is_some() && now == last {
            return h.get_by_label(label);
        }
        last = now;
    }
    panic!("{label:?} never appeared or never stopped moving");
}

#[test]
fn about_dialog_has_contributors_and_models_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    settled(&mut h, "Contributors").click();
    // The owner is always in the compiled-in credits (contributors/contributors.json), shown by username.
    settled(&mut h, "@echelon");
    settled(&mut h, "Table").click();
    settled(&mut h, "PRs");
    settled(&mut h, "Display name").click();
    settled(&mut h, "Brandon Thomas");
    settled(&mut h, "Models").click();
    h.run_steps(4);
    assert!(h.query_all_by_label("Anthropic").count() >= 1);
}
