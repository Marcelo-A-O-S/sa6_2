export class Apiaddgrupos {
    async getApiaddgrupos() {
        let Grupo = await fetch("htpp://127..0.0.1:6969/grupo/<grupo_input>", {
            method: "POST"
        })
            .then((response) => {
                return response.json()
            });
        return Grupo;

    }
}