export class Apiaddtabela{
        async getApiaddtabela(){
            let Tabela = await fetch ("http://127.0.0.1:6969/grupo/<grupo_input>")
            method: "GET"
            .then( (response) => {
                return response.json()
            })
    
            return Tabela; 
    
        }
    }
    