//import {getGrupo} from "../js.api/Apiaddgetgrupo.js"
import {Apigetgrupo} from "../../js.api/Apigetgrupo.js"
document.addEventListener('DOMContentLoaded', function() {
    var entrarBtn = document.getElementById('entrarBtn');
    entrarBtn.addEventListener('click', function() {
        var nome_grupo = document.getElementById('nomeGrupo').value;
        var senha_grupo = document.getElementById('senhaGrupo').value;
        
        var storedNomeGrupo = localStorage.getItem('nomeGrupo');
        var storedSenhaGrupo = localStorage.getItem('senhaGrupo');
        
        if (nome_grupo === storedNomeGrupo && senha_grupo === storedSenhaGrupo) {
            window.location.href = 'TabelaGerenciamento.html';       
        }
        
        else {
            alert('Por favor, insira o nome do grupo e a senha.');
        }
    });
});