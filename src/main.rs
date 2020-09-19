mod config;
mod launcher;
mod registry;
mod err;

use anyhow::{Context, Result};
use launcher::launch_browser;

fn main() -> Result<()> {
    init_logger()?;

    let args = std::env::args().skip(1);

    for arg in args {
        match arg.as_str() {
            "--register" => {
                println!("Register app as browser.");
                let _ = registry::register()
                    .context("unable to register app as browser. Try running the application as administrator.")?;
                return Ok(());
            }
            "--unregister" => {
                println!("Unregister app.");
                let _ = registry::unregister()
                    .context("unable to unregister app. Try running the application as administrator.")?;
                return Ok(());
            }
            url => {
                launch_browser(url)
                    .context("failed to launch url")?;
            }
        }
    }

    Ok(())
}

fn init_logger() -> Result<()> {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let log_path = std::env::current_exe()
        .context("failed to start logger.")?
        .parent()
        .unwrap()
        .join("log")
        .join("output.log");

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(log_path)
        .context("failed to start logger.")?;
    let logfile = Box::new(logfile);

    let file_appender = Appender::builder().build("logfile", logfile);

    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build();
    let console = Box::new(console);

    let console_appender = Appender::builder().build("console", console);

    let root = Root::builder()
        .appender("logfile")
        .appender("console")
        .build(LevelFilter::Info);

    let config = Config::builder()
        .appender(file_appender)
        .appender(console_appender)
        .build(root)
        .context("failed to start logger.")?;

    log4rs::init_config(config)
    .context("failed to start logger.")?;

    Ok(())
}
