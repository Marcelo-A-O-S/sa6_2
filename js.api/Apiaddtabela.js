 export class Apiaddtabela{
    async getApiaddtabela(){
        let Tabela = await fetch ("htpp://127..0.0.1:8080/projeto/<projeto_input>")
        method: "POST"
        .then( (response) => {
            return response.json()
        })

        return Tabela; 

}
}