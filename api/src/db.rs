use crate::{prelude::W, utils::macros::map};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};
use chrono::{DateTime, Utc};
use surrealdb::{
        dbs::{Response, Session},
        kvs::Datastore,
        sql::{thing, Array, Object, Value},
};






#[derive(Debug, Serialize, Deserialize)]
pub struct Grupo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub nome_grupo: String,
    pub senha_grupo: String,
    pub gerencia_projeto: String,
    pub scrum_master: String,
    pub product_owner: String,
    pub equipe_dev: String,
    pub descricao_grupo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hora_criacao: Option<DateTime<Utc>>,

}
// <&str as Into<String>>::into("")
impl From<Grupo> for Value {
    fn from(val: Grupo) -> Self {
        match val.id {
            Some(v) => map![
                <&str as Into<String>>::into("id") => v.into(),
                <&str as Into<String>>::into("nome_grupo") => val.nome_grupo.into(),
                <&str as Into<String>>::into("senha_grupo") => val.senha_grupo.into(),
                <&str as Into<String>>::into("gerencia_projeto") => val.gerencia_projeto.into(),
                <&str as Into<String>>::into("scrum_master").into() => val.scrum_master.into(),
                <&str as Into<String>>::into("product_owner") => val.product_owner.into(),
                <&str as Into<String>>::into("equipe_dev") => val.equipe_dev.into(),
                <&str as Into<String>>::into("descricao_grupo") => val.descricao_grupo.into(),
            ]
            .into(),

            None => map![
                <&str as Into<String>>::into("nome_grupo") => val.nome_grupo.into(),
                <&str as Into<String>>::into("senha_grupo") => val.senha_grupo.into(),
                <&str as Into<String>>::into("gerencia_projeto") => val.gerencia_projeto.into(),
                <&str as Into<String>>::into("scrum_master").into() => val.scrum_master.into(),
                <&str as Into<String>>::into("product_owner") => val.product_owner.into(),
                <&str as Into<String>>::into("equipe_dev") => val.equipe_dev.into(),
                <&str as Into<String>>::into("descricao_grupo") => val.descricao_grupo.into(),
            ]
            .into(),
        }
    }
}

impl Creatable for Grupo {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Projeto {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    numero_codigo: String,
    pertence_grupo: String,
    qual_atividade: String,
    quem_responsavel: String,
    tempo_sprint: String,
    projeto_dependencia: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hora_criacao: Option<DateTime<Utc>>,

}

impl From<Projeto> for Value {
    fn from(val: Projeto) -> Self {
        match val.id {
            Some(v) => map![
                <&str as Into<String>>::into("id") => v.into(),
                <&str as Into<String>>::into("numero_codigo") => val.numero_codigo.into(),
                <&str as Into<String>>::into("pertence_grupo") => val.pertence_grupo.into(),
                <&str as Into<String>>::into("qual_atividade") => val.qual_atividade.into(),
                <&str as Into<String>>::into("quem_responsavel") => val.quem_responsavel.into(),
                <&str as Into<String>>::into("tempo_sprint") => val.tempo_sprint.into(),
                <&str as Into<String>>::into("projeto_dependencia") => val.projeto_dependencia.into(),
            ]
            .into(),

            None => map![
                <&str as Into<String>>::into("numero_codigo") => val.numero_codigo.into(),
                <&str as Into<String>>::into("pertence_grupo") => val.pertence_grupo.into(),
                <&str as Into<String>>::into("qual_atividade") => val.qual_atividade.into(),
                <&str as Into<String>>::into("quem_responsavel") => val.quem_responsavel.into(),
                <&str as Into<String>>::into("tempo_sprint") => val.tempo_sprint.into(),
                <&str as Into<String>>::into("projeto_dependencia") => val.projeto_dependencia.into(),
            ]
            .into(),
        }
    }
}


impl Creatable for Projeto {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}

pub trait Creatable: Into<Value> {}









pub struct DB {
    pub ds: Arc<Datastore>,
    pub sesh: Session,
}

impl DB {
    pub async fn execute(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<Response>, crate::error::Error> {
        let res = self.ds.execute(query, &self.sesh, vars).await?;
        Ok(res)
    }

    pub async fn add_grupo(&self, nome_grupo: String) -> Result<Object, crate::error::Error> {
        let sql = "CREATE grupo SET nome_grupo = $nomeGrupo, senha_grupo = $senhaGrupo, gerencia_projeto = $gerenciaProjeto, scrum_master = $scrumMaster, product_owner = $productOwner, equipe_dev = $equipeDev, descricao_grupo = $descricaoGrupo";
        let vars: BTreeMap<String, Value> =
            map!["nomeGrupo".into() => Value::Strand(nome_grupo.into())];

        let ress = self.execute(sql, Some(vars));

        let first_res = ress.into_iter().next().expect("não recebeu resposta");

        W(first_res.result?.first()).try_into()
    }

    pub async fn login_grupo(&self, nome: String, senha: String) -> Result<Object, crate::error::Error> {
        let sql = "SELECT * FROM grupo WHERE nome_grupo = $nomeGrupo AND senha_grupo = $senhaGrupo";
        let vars: BTreeMap<String, Value> = map!["nome_grupo".into() => Value::Strand(nome.into()), "senha_grupo".into() => Value::Strand(senha.into())];
        let ress = self.execute(sql, Some(vars));

        let first_res = ress.into_iter().next().expect("não recebeu resposta");

        W(first_res.result?.first()).try_into()
    }

    pub async fn add_projeto(&self, numero_codigo: String) -> Result<Object, crate::error::Error> {
        let sql = "CREATE projeto SET numero_codigo = $numeroCodigo, pertence_grupo = $pertenceGrupo, qual_atividade = $qualAtividade, quem_responsavel = $quemResponsavel, tempo_sprint = $tempoSprint, projeto_dependencia = $projetoDependencia";
        let vars: BTreeMap<String, Value> =
            map!["numero_codigo".into() => Value::Strand(numero_codigo.into())];
        let ress = self.execute(sql, Some(vars)).await?;

        let first_res = ress.into_iter().next().expect("não recebeu resposta");

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_all_projetos(&self) -> Result<Vec<Object>, crate::error::Error> {
        let sql = "SELECT * FROM projeto ORDER BY numero_codigo ASC";

        let res = self.execute(sql, None).await?;

        let first_res = res.into_iter().next().expect("não recebeu resposta");


        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }


    pub async fn delete_projeto(&self, id: &str) -> Result<AffectedRows, crate::error::Error> {
        let sql = "Delete $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let _ = self.execute(sql, Some(vars)).await?;

        Ok(AffectedRows { rows_affected: 1 })
    }
}
