
use clap::Clap;

#[derive(Clap, Clone)]
pub struct AppOpts {
  #[clap(short, long, env = "PORT", default_value = "8080")]
  pub port: u16,

  #[clap(short, long, env = "REDIS_URL")]
  pub redis_url: String,
}
