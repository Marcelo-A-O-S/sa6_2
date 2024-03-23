use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Grupo {
    pub nome_grupo: String,
    pub senha_grupo: String,
    pub gerencia_projeto: String,
    pub scrum_master: String,
    pub product_owner: String,
    pub equipe_dev: String,
    pub descricao_grupo: String,
}

#[derive(Debug, Deserialize)]
pub struct Projeto {
    pub nome_projeto: String,
    pub descricao_projeto: String,
    pub gerencia_projeto: String,
    pub scrum_master: String,
    pub product_owner: String,
    pub equipe_dev: String,
    pub descricao_projeto: String,
}

#[derive(Debug, Deserialize)]
pub struct AfectedRows {
    pub affected_rows: u64,
}

#[derive(Deserialize)]
pub struct RowId {
    pub id: String,
}