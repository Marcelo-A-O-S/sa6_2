#[macro_use]
extern crate rocket;

use crate::db::{AffectedRows, DB, Projeto, Grupo};
use cors::*;
use rocket::{serde::json::Json, State};
use std::{io::ErrorKind, sync::Arc};
use surrealdb::{dbs::Session, kvs::Datastore};

mod cors;
mod db;
mod error;
mod utils;


use rocket::http::Status;



#[patch("/projeto", format = "json", data = "<projeto_input>")]
async fn update_projeto(projeto_input: &str, db: &State<DB>) -> Result<serde_json::Value, std::io::Error> {

    let deserialized: Projeto = serde_json::from_str(projeto_input).unwrap();

    let tasks = db
        .update_projeto(deserialized)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "deu merda tio"))?;
    Ok(tasks)
}

#[post("/grupo", format = "json", data = "<grupo_input>")]
async fn add_grupo(grupo_input: &str, db: &State<DB>) -> Result<serde_json::Value, std::io::Error> {

    let deserialized: Grupo = serde_json::from_str(grupo_input).unwrap();
    let tasks = db
        .add_grupo(deserialized)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "deu merda tio"))?;
    Ok(tasks)
}

#[get("/get_grupo/<nome>/<senha>", format="json")]
async fn get_grupo(nome: &str, senha: &str, db: &State<DB>) -> Result<serde_json::Value, std::io::Error> {
    let tasks = db
        .login_grupo(nome, senha)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "deu merda tio"))?;
    Ok(tasks)
}


#[post("/projeto", format = "json", data = "<projeto_input>")]
async fn add_projeto(projeto_input: &str, db: &State<DB>) -> Result<serde_json::Value, std::io::Error> {

    let deserialized: Projeto = serde_json::from_str(projeto_input).unwrap();

    let tasks = db
        .add_projeto(deserialized)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "deu merda tio"))?;
    Ok(tasks)
}




#[get("/all_projetos")]
async fn get_all_projetos(db: &State<DB>) -> Result<serde_json::Value, std::io::Error> {
    let tasks = db
        .get_all_projetos()
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "deu merda tio"))?;
    Ok(tasks)
}

#[delete("/deletar_projeto/<id>")]
async fn deletar_projeto(id: &str, db: &State<DB>) -> Result<Json<AffectedRows>, std::io::Error> {
    let affected_rows = db
        .deletar_projeto(id)
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "não deu pra deletar"))?;
    Ok(Json(affected_rows))
}

#[launch]
async fn rocket() -> _ {
    let datastore = Arc::new(Datastore::new("memory").await.unwrap());
    let session = Session::owner().with_ns("my_ns").with_db("my_db");

    let db = DB { datastore, session };

    rocket::build()
        .mount(
            "/",
                routes![get_all_projetos, deletar_projeto, add_projeto, add_grupo, get_grupo, update_projeto]
        )
        .attach(CORS)
        .manage(db)
}

