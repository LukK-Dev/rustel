use super::errors;
use fern::colors;

pub fn init_logger() -> Result<(), errors::ApplicationError> {
    let colors = colors::ColoredLevelConfig::new()
        .trace(colors::Color::BrightBlue)
        .info(colors::Color::BrightGreen)
        .debug(colors::Color::Magenta);

    match fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}]: {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(errors::ApplicationError::from(err)),
    }
}
