export class Apigetgrupo{
    async getApigetgrupo(nome, senha){
        return await fetch (`http://127.0.0.1:6969/grupo/${nome}/${senha}`,{
            method: "GET"
        })
        .then((response) => response.json())
        .then((data)=> {return data})
         

    }
}