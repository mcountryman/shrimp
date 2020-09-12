
use clap::Clap;

#[derive(Clap, Clone)]
pub struct AppOpts {
  #[clap(short, long, default_value = "8080")]
  pub port: u16,

  #[clap(short, long, env = "DATABASE")]
  pub database: String,
}
