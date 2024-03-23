use crate::models::*;
use reqwasm::{http::Request, Error};

const BASE_URL: &str = "http://localhost:8000";

pub async fn fetch_all_projetos() -> Result<Vec<Projeto>, Error> {
    Request::get(&format!("{BASE_URL}/all_projetos"))
        .send()
        .await?
        .json()
        .await
}

pub async fn create_projeto(projeto: &str) -> Result<Projeto, Error> {
    Request::post(&format!("{BASE_URL}/projeto/{projeto}"))
        .json(&projeto)
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    Ok(vec.remove(0))
}


pub async fn delete_projeto(id: String) -> Result<AffectedRows, Error> {
    Request::delete(&format!("{BASE_URL}/deletar_projeto/{id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await
}


pub async fn create_grupo(grupo_input: &str) -> Result<Grupo, Error> {
    Request::post(&format!("{BASE_URL}/grupo/{grupo_input}"))
        .json(&grupo)
        .send()
        .await?
        .json()
        .await
}