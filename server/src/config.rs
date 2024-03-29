use clap::Parser;

#[cfg(debug_assertions)]
static DEFAULT_LOG_LEVEL: &str = "info,server=debug";
#[cfg(not(debug_assertions))]
static DEFAULT_LOG_LEVEL: &str = "info";
static DEFAULT_MQTT_URL: &str = "localhost";

static LONG_ABOUT: &str = "\
A group project for MSE Software Engineering 2023.

The full documentation is embedded in the application at the url '/docs'.";

#[derive(Parser, Debug)]
#[command(
    name = "server", version,
    about = "A group project for SoftwEng",
    long_about = LONG_ABOUT,
)]
pub struct Config {
    /// The verbosity of the log output (error, warn, info, debug, trace)
    ///
    /// Note the syntax: "info,server=debug,hyper=warn" means that:
    ///
    /// - The server crate (our code) will print logs at level 'debug'.
    ///
    /// - The library hyper will print logs at level 'warn'.
    ///
    /// - All other crates (libraries) will print at level 'info'.
    #[arg(short, long, env, default_value = DEFAULT_LOG_LEVEL)]
    pub log_level: String,

    /// The port on which to run the axum server
    #[arg(short, long, env, default_value_t = 4000)]
    pub port: u16,

    /// Host of the MQTT broker to subscribe to for device measurements
    #[arg(short, long, env, default_value = DEFAULT_MQTT_URL)]
    pub mqtt_url: String,

    /// The port on which the MQTT broker listens
    #[arg(short = 'q', long, env, default_value_t = 1883)]
    pub mqtt_port: u16,
}
