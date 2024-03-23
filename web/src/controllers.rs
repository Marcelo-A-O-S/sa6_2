use crate::{
    state::{GrupoAction, ProjetoAction, ProjetoState, GrupoState},
    api,
};

use web_sys::{js_sys::Array, wasm_bindgen::JsValue};
use yew::UseReducerHandle;

pub struct GrupoController {
    state: UseReducerHandle<GrupoState>,
}

pub struct ProjetoController {
    state: UseReducerHandle<ProjetoState>,
}

impl GrupoController {
    pub fn new(state: UseReducerHandle<GrupoState>) -> Self {
        Self { state }
    }

    pub fn init_grupo(&self) {
        let state = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match api::fetch_all_grupos().await {
                Ok(grupos) => {
                    state.dispatch(GrupoAction::Set(grupos));
                }
                Err(e) => {
                    log::error!("Failed to fetch grupos: {:?}", e);
                }
            }
        });
    }

    pub fn add_grupo(&self, grupo_input: &str) {
        let state = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match api::create_grupo(grupo_input).await {
                Ok(grupo) => {
                    state.dispatch(GrupoAction::Add(grupo));
                }
                Err(e) => {
                    log::error!("Failed to add grupo: {:?}", e);
                }
            }
        });
    }
}

impl ProjetoController {
    pub fn new(state: UseReducerHandle<ProjetoState>) -> Self {
        Self { state }
    }

    pub fn init_projetos(&self) {
        let state = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match api::fetch_all_projetos().await {
                Ok(projetos) => {
                    state.dispatch(ProjetoAction::Set(projetos));
                }
                Err(e) => {
                    log::error!("Failed to fetch projetos: {:?}", e);
                }
            }
        });
    }

    pub fn add_projeto(&self, projeto_input: &str) {
        let state = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match api::create_projeto(projeto_input).await {
                Ok(projeto) => {
                    state.dispatch(ProjetoAction::Add(projeto));
                }
                Err(e) => {
                    log::error!("Failed to add projeto: {:?}", e);
                }
            }
        });
    }

    pub fn delete_projeto(&self, id: String) {
        let state = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match api::delete_projeto(id).await {
                Ok(_) => {
                    state.dispatch(ProjetoAction::Delete(id));
                }
                Err(e) => {
                    log::error!("Failed to delete projeto: {:?}", e);
                }
            }
        });
    }
}