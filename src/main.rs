mod app;

use app::App;

fn main() -> color_eyre::Result<()> {
    let _guard = sentry::init((
        "https://62cf52ffca04346f14507c4a08db9b2c@o4510067097665536.ingest.de.sentry.io/4510081161756752",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            // Capture user IPs and potentially sensitive headers when using HTTP server integrations
            // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
            send_default_pii: true,
            ..Default::default()
        },
    ));
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}
