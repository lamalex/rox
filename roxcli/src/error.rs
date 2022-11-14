#[derive(Debug, miette::Diagnostic, thiserror::Error)]
#[diagnostic(severity(warning))]
#[error("there was a problem reading metadata in \"{msg}\"")]
pub struct MetadataWarning {
    msg: String,
    source: std::io::Error,
}

impl MetadataWarning {
    pub fn render(msg: String, source: std::io::Error) {
        let reporter = miette::GraphicalReportHandler::new();
        let mut buf = String::new();
        reporter
            .render_report(&mut buf, &Self { msg, source })
            .expect("could not render report");

        println!("{buf}");
    }
}
