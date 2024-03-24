export class Apigetgrupo{
    async getApigetgrupo(){
        let getGrupo = await fetch ("htpp://127..0.0.1:8080/grupo/<nome>/<senha>")
        method: "GET"
        .then( (response) => {
            return response.json()
        })

        return getGrupo; 

    }

    async getAcademiaBynomeGrupo(nomeGrupo){
        let Grupos = [];
        Grupos = await fetch(`/Frontend/src/js/data/DataAcademias.json?nomeGrupo=${nomeGrupo}`)
        .then((response)=>{
            return response.json();
        })
        return ;

}
}