mod services;
mod utils;

use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Exibir últimas medições
    #[arg(short, long)]
    recente: bool,

    /// Desabilitar SSL
    #[arg(short, long, default_value_t)]
    unsecure: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    let data = services::river_info_fetcher(cli.unsecure).await;

    match cli.recente {
        true => {
            let mut last_level: Option<f32> = None;

            data.into_iter().for_each(move |item| {
                utils::display_river_info(&item, &last_level);
                last_level = Some(item.level);
            });
        }

        _ => {
            if !data.is_empty() {
                let item = &data[data.len() - 1];
                utils::display_river_info(item, &None);
            }
        }
    }

    Ok(())
}
