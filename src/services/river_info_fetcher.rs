use serde::{Deserialize, Serialize};

use crate::utils::{cache_file_exists, get_cache_file, save_cache_file};

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

pub async fn river_info_fetcher() -> Vec<NivelItem> {
    let resp = reqwest::Client::builder()
        // TODO: remove or enable invalid certs using args
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(URI)
        .send()
        .await;

    let result = match resp {
        Ok(res) => {
            let result = res.text().await.unwrap();
            save_cache_file(result.clone());

            result
        }
        Err(error) => {
            // TODO: should verify if has data in cache
            // when cache exists, should transform this message in a warning
            // else, should be a error em finish the routine.
            print!(
                "Erro ao tentar recuperar dados atualizados: {}",
                error.to_string()
            );

            match cache_file_exists() {
                true => get_cache_file(),
                _ => String::from(""),
            }
        }
    };

    if !result.is_empty() {
        let deserialized: RiverInfo = serde_json::from_str(&result).unwrap();
        return deserialized.levels;
    }

    panic!("Não foi possível exibir os dados no momento, tente novamente mais tarde.");
}
