mod core;
mod services;
mod utils;

use clap::{command, Parser};
use utils::DisplayRiverInfo;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Exibir últimas medições
    #[arg(short, long)]
    recente: bool,

    /// Desabilitar SSL
    #[arg(short, long, default_value_t)]
    unsecure: bool,

    /// URL para sobrescrever a url padrão de busca dos dados
    #[arg(long)]
    url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    let data = services::river_info_fetcher(cli.unsecure, cli.url).await;

    match cli.recente {
        true => {
            let mut last_level: Option<f32> = None;

            data.into_iter().for_each(move |item| {
                DisplayRiverInfo::display_river_info(&item, &last_level);
                last_level = Some(item.level);
            });
        }

        _ => {
            if !data.is_empty() {
                let item = &data[data.len() - 1];
                DisplayRiverInfo::display_river_info(item, &None);
            }
        }
    }

    Ok(())
}
