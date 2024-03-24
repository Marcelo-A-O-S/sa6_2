export class Apideletarproject{
    async getApideletarproject(){
        let deletar = await fetch ("htpp://127..0.0.1:8080/DELETE/deletar_projeto/<id>")
        method: "DELETE"
        .then( (response) => {
            return response.json()
        })

        return deletar; 

}
}