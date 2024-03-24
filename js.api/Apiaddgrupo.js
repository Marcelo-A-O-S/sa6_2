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
}