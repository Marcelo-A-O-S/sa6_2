export class Apiaddgrupos {
    async getApiaddgrupos() {
        let Grupo = await fetch("http://127.0.0.1:6969/grupo/<grupo_input>", {
            method: "POST"
        })
            .then((response) => {
                return response.json()
            });
            if (response.ok) {
                let data = await response.json();
                return data;
            } else {
                throw new Error('Erro ao fazer a solicitação: ' + response.status);
            }
        } catch (error) {
            console.error('Erro:', error);
            return null; 
        }
    }
