use crate::{core::Logger, utils::Cache};
use serde::{Deserialize, Serialize};

const URI: &str = "https://alertablu.blumenau.sc.gov.br/static/data/nivel_oficial.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct NivelItem {
    #[serde(rename(deserialize = "nivel"))]
    pub level: f32,

    #[serde(rename(deserialize = "horaLeitura"))]
    pub datetime: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RiverInfo {
    #[serde(rename(deserialize = "niveis"))]
    levels: Vec<NivelItem>,
}

pub async fn river_info_fetcher(unsecure: bool, uri: Option<String>) -> Vec<NivelItem> {
    let get_json_uri = match uri {
        Some(val) => val,
        None => String::from(URI),
    };

    let resp = reqwest::Client::builder()
        .danger_accept_invalid_certs(unsecure)
        .build()
        .unwrap()
        .get(get_json_uri)
        .send()
        .await;

    let result = match resp {
        Ok(res) => {
            let result = res.text().await.unwrap();
            Cache::save(result.clone());

            result
        }
        Err(error) => {
            // TODO: should verify if has data in cache
            // when cache exists, should transform this message in a warning
            // else, should be a error em finish the routine.
            Logger::print(&format!(
                "Erro ao tentar recuperar dados atualizados: {}",
                error
            ));

            match Cache::exists() {
                true => Cache::get(),
                _ => String::from(""),
            }
        }
    };

    if result.is_empty() {
        Logger::panic("Não foi possível exibir os dados no momento, tente novamente mais tarde.");
    }

    serde_json::from_str::<RiverInfo>(&result).unwrap().levels
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn should_return_vector_of_nivel_item_from_request() {
        let mock_server = MockServer::start().await;

        let body = r#"{"niveis":[{"nivel":6.51,"horaLeitura":"2023-10-11T12:00:04Z"},{"nivel":6.47,"horaLeitura":"2023-10-11T13:00:03Z"}]}"#;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&mock_server)
            .await;

        let result = river_info_fetcher(false, Some(String::from(&mock_server.uri()))).await;

        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    #[should_panic(
        expected = "Não foi possível exibir os dados no momento, tente novamente mais tarde."
    )]
    async fn should_panic_when_cannot_load_data() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let _ = river_info_fetcher(false, Some(String::from(&mock_server.uri()))).await;
    }
}
