export class Apigetgrupo{
    async getApigetgrupo(){
        let getGrupo = await fetch ("htpp://127..0.0.1:6969/grupo/<nome>/<senha>")
        method: "GET"
        .then( (response) => {
            return response.json()
        })

        return getGrupo; 

    }
}