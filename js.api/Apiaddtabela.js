export class Apiaddtabela {
        async getApiaddtabela(projeto_input) {
            try {
                let Tabela = await fetch(`http://127.0.0.1:6969/projeto/${projeto_input}`, {
                    method: "POST"
                });
    
                if (Tabela.ok) {
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
    }
    