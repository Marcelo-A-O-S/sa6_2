export class Apiaddgrupos {
    async getApiaddgrupos() {
        let Grupo = await fetch("htpp://127..0.0.1:8080/grupo/<grupo_input>", {
            method: "POST"
        })
            .then((response) => {
                return response.json()
            });
        return Grupo;
    }
    async createGrupos(grupo) {
        console.log(JSON.stringify(grupo))
        await fetch("", {
            body: JSON.stringify(grupo),
            method: 'POST',
            mode: "no-cors",
            headers: {
                'Content-Type': 'application/json'
            }
        }).then((response) => response.json())
            .then((data) => { console.log(data) })
            .catch((err) => {
                console.log(err)
            })

    }
}