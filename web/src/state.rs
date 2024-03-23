use std::rc::Rc;

use yew::Reducible;

use crate::models::Projeto;

use crate::models::Grupo;

pub enum ProjetoAction {
    Set(Vec<Projeto>),
    Add(Projeto),
    Delete(String),
}

#[derive(Default)]
pub struct ProjetoState {
    pub projetos: Vec<Projeto>,
}


impl Reducible for ProjetoState {
    type Action = ProjetoAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_projeto = match action {
            ProjetoAction::Set(projetos) => projetos,
            ProjetoAction::Add(projeto) => {
                let mut projetos = self.projetos.clone();
                projetos.push(projeto);
                projetos
            }
            ProjetoAction::Delete(id) => {
                let mut projetos = self.projetos.clone();
                projetos.retain(|projeto| projeto.id != id);
                projetos
            }
        };
    }
}

pub enum GrupoAction {
    Set(Vec<Grupo>),
    Add(Grupo),
}

#[derive(Default)]
pub struct GrupoState {
    pub grupos: Vec<Grupo>,
}


impl Reducible for GrupoState {
    type Action = GrupoAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_grupo = match action {
            GrupoAction::Set(grupos) => grupos,
            GrupoAction::Add(grupo) => {
                let mut grupos = self.grupos.clone();
                grupos.push(grupo);
                grupos
            }
        };
    }
}


