export class Apigetallproject{
    async getApigetallproject(){
         let gerenciamento = await fetch ("htpp://127..0.0.1:6969//all_projetos")
        method: "GET"
        .then( (response) => {
            return response.json()
        })

        return gerenciamento; 

  }
}