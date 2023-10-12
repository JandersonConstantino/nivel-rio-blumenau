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

async fn program(cli: Cli) {
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    program(Cli::parse()).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::MockLogger;
    use mockall::predicate::eq;
    use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn should_return_only_first_item() {
        let mock_server = MockServer::start().await;
        let body = r#"{"niveis":[{"nivel":6.51,"horaLeitura":"2023-10-11T12:00:04Z"},{"nivel":6.47,"horaLeitura":"2023-10-11T13:00:03Z"}]}"#;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&mock_server)
            .await;

        let ctx = MockLogger::print_context();

        ctx.expect()
            .once()
            .times(1)
            .with(eq("11/10/2023 10:00:03 - 6.47 metros "))
            .returning(|_| ());

        program(Cli {
            url: Some(mock_server.uri()),
            recente: false,
            unsecure: false,
        })
        .await;
    }
}
