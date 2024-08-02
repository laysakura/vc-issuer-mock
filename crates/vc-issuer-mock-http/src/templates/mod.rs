use tera::Tera;

/// Build HTML template.
///
/// # Panics
///
/// If any template file is missing.
pub(crate) fn init_templates() -> Tera {
    let mut tera = Tera::default();

    tera.add_raw_templates(vec![(
        "index.html",
        include_str!("../../templates/index.html"),
    )])
    .expect("Failed to add templates");

    tera
}
