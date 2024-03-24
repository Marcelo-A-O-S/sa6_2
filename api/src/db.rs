use crate::utils::macros::map;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};
use surrealdb::{
    dbs::{Response, Session},
    kvs::Datastore,
    sql::{thing, Value},
};

use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;



async fn query_database(query: String, table_name: String) -> surrealdb::Result<()>{
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root".into(),
        password: "root".into(),
    })
    .await?;

    db.use_ns("teste").use_db("teste").await?;

    let query = db
        .query(query)
        .bind(("table", table_name, ))
        .await?;
    dbg!(query);

    Ok(())
}




#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

}
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
#[serde(rename_all = "camelCase")]
pub struct Projeto {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    numero_codigo: String,
    pertence_grupo: String,
    qual_atividade: String,
    quem_responsavel: String,
    tempo_sprint: String,
    projeto_dependencia: String,


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




#[derive(Clone)]
pub struct DB {
    pub datastore: Arc<Datastore>,
    pub session: Session,
}

impl DB {
    pub async fn execute(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<Response>, crate::error::Error> {
        let res = self.datastore.execute(query, &self.session, vars).await?;
        Ok(res)
    }

    pub async fn add_grupo(&self, nome_grupo: Grupo) -> Result<serde_json::Value, crate::error::Error> {
        let sql = "CREATE grupo SET nome_grupo = $nome_grupo, senha_grupo = $senha_grupo, gerencia_projeto = $gerencia_projeto, scrum_master = $scrum_master, product_owner = $product_owner, equipe_dev = $equipe_dev, descricao_grupo = $descricao_grupo";
        let serialized = serde_json::to_string(&nome_grupo).unwrap();


        let vars: BTreeMap<String, Value> =
            map!["nome_grupo".into() => Value::Strand(serialized.into())];

        let res = self.execute(&sql, Some(vars)).await?;


        query_database(sql.to_string(), "grupo".to_string()).await;

        let first_res = res.into_iter().next().expect("não recebeu resposta");

        Ok(first_res.result?.into_json())
    }

    pub async fn login_grupo(&self, username: &str, password: &str) -> Result<serde_json::Value, crate::error::Error> {
        let sql = "SELECT * FROM usuario WHERE username = $username AND password = $password";
        let vars: BTreeMap<String, Value> = map![
        "username".into() => Value::Strand(username.into()),
        "password".into() => Value::Strand(password.into()), // daria pra fazer um hash para a senha mas, vaffaculo?
    ];

        let res = self.execute(sql, Some(vars)).await?;
        query_database(sql.to_string(), "grupo".to_string()).await;

        let first_res = res.into_iter().next().expect("nome do grupo ou senha invalida");

        Ok(first_res.result?.into_json())
    }


        pub async fn add_projeto(&self, projeto: Projeto) -> Result<serde_json::Value, crate::error::Error> {


        let sql = "CREATE projeto SET numero_codigo = $numeroCodigo, pertence_grupo = $pertenceGrupo, qual_atividade = $qualAtividade, quem_responsavel = $quemResponsavel, tempo_sprint = $tempoSprint, projeto_dependencia = $projetoDependencia";
            let serialized = serde_json::to_string(&projeto).unwrap();

        let vars: BTreeMap<String, Value> =
            map!["numero_codigo".into() => Value::Strand(serialized.into())];
        let res = self.execute(sql, Some(vars)).await?;


            query_database(sql.to_string(), "projeto".to_string()).await;

            let first_res = res.into_iter().next().expect("não recebeu resposta");

        Ok(first_res.result?.into_json())
    }



    pub async fn get_all_projetos(&self) -> Result<serde_json::Value, crate::error::Error> {
        let sql = "SELECT * FROM projeto ORDER BY hora_criacao ASC";

        let res = self.execute(sql, None).await?;
        query_database(sql.to_string(), "projeto".to_string()).await;

        let first_res = res.into_iter().next().expect("não recebeu resposta");


        Ok(first_res.result?.into_json())
    }


    pub async fn deletar_projeto(&self, id: &str) -> Result<AffectedRows, crate::error::Error> {
        let sql = "Delete $th";
        let tid = format!("{}", id);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
        let _ = self.execute(sql, Some(vars)).await?;
        query_database(sql.to_string(), "grupo".to_string()).await;

        Ok(AffectedRows { rows_affected: 1 })
    }

    pub async fn update_projeto(&self, projeto: Projeto) -> Result<serde_json::Value, crate::error::Error> {
        let sql = "UPDATE projeto SET numero_codigo = $numeroCodigo, pertence_grupo = $pertenceGrupo, qual_atividade = $qualAtividade, quem_responsavel = $quemResponsavel, tempo_sprint = $tempoSprint, projeto_dependencia = $projetoDependencia WHERE id = $id";
        let serialized = serde_json::to_string(&projeto).unwrap();

        let vars: BTreeMap<String, Value> =
            map!["numero_codigo".into() => Value::Strand(serialized.into())];
        let res = self.execute(sql, Some(vars)).await?;
        query_database(sql.to_string(), "projeto".to_string()).await;

        let first_res = res.into_iter().next().expect("não recebeu resposta");

        Ok(first_res.result?.into_json())
    }
}
