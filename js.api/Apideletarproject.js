export class Apideletarproject{
    async getApideletarproject(){
        let deletar = await fetch ("htpp://127..0.0.1:6969/DELETE/deletar_projeto/<id>")
        method: "DELETE"
        .then( (response) => {
            return response.json()
        })

        return deletar; 

}
}