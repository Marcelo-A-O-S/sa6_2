 export class Apiaddtabela{
    async getApiaddtabela(){
<<<<<<< HEAD
        let Tabela = await fetch ("http://127..0.0.1:8080/projeto/<projeto_input>")
=======
        let Tabela = await fetch ("http://127.0.0.1:6969/projeto/<projeto_input>")
>>>>>>> 970a553be043aee4bc7d7bcdb65e27d31943c0fe
        method: "POST"
        .then( (response) => {
            return response.json()
        })

        return Tabela; 

}
}