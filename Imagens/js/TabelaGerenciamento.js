/* import {gerenciamento} from "../js.api/Apigetallproject.js"
import {deletar} from "../js.api/Apideletarproject.js"   */
import { Apigetallproject } from "../../js.api/Apigetallproject.js"
import { Apideletarproject } from "../../js.api/Apideletarproject.js"

document.addEventListener('DOMContentLoaded', function () {


    function editarProjeto(numeroCodigo, qualAtividade, quemResponsavel, tempoSprint, projetoDependencia) {
        window.location.href = `EditarProjeto.html?numeroCodigo=${numeroCodigo}&qualAtividade=${encodeURIComponent(qualAtividade)}&quemResponsavel=${encodeURIComponent(quemResponsavel)}&tempoSprint=${encodeURIComponent(tempoSprint)}&projetoDependencia=${encodeURIComponent(projetoDependencia)}`;


    }


    var tabelaProjetos = document.getElementById('tabelaProjetos');

    var newRow = tabelaProjetos.insertRow();
    newRow.innerHTML = `
                <td>${dadosProjeto.numeroCodigo}</td>
                <td>${dadosProjeto.qualAtividade}</td>
                <td>${dadosProjeto.quemResponsavel}</td>
                <td>${dadosProjeto.tempoSprint}</td>
                <td>${dadosProjeto.projetoDependencia}</td>
                <td>
                    <button onclick="editarProjeto(${dadosProjeto.numeroCodigo}, '${dadosProjeto.qualAtividade}', '${dadosProjeto.quemResponsavel}', '${dadosProjeto.tempoSprint}', '${dadosProjeto.projetoDependencia}')" class="btn">Editar</button>
                    <button onclick="removerProjeto(${dadosProjeto.numeroCodigo})" class="btn red">Remover</button>
                </td>
            `;


    var nome_grupo = document.getElementById('nomeGrupo');
    nome_grupo.textContent = "(nome do grupo)";

    var scrumMaster = document.getElementById('scrumMaster');
    scrumMaster.textContent = "(SCRUM Master)";

    var productOwner = document.getElementById('productOwner');
    productOwner.textContent = "(Product Owner)";

    var equipeDesenvolvimento = document.getElementById('equipeDesenvolvimento');
    equipeDesenvolvimento.textContent = "(Equipe de Desenvolvimento)";
});