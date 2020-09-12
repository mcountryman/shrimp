
use clap::Clap;

#[derive(Clap, Clone)]
pub struct AppOpts {
  #[clap(short, long, env = "DATABASE")]
  pub database: String,
}
