#[derive(clap::Parser, Debug)]
pub struct Config {
    #[clap(long, env, required = true)]
    pub database_url: String,
    #[clap(long, env, default_value_t = 4000)]
    pub port: u16,
    #[clap(long, env, num_args = 1.., value_delimiter = ',')]
    pub allowed_origins: Vec<String>,
}
