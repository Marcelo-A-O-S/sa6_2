//import {Tabela} from "../js.api/Apiaddtabela.js"
import {Apiaddtabela} from "../../js.api/Apiaddtabela.js"
        document.addEventListener('DOMContentLoaded', function() {
            var form = document.getElementById('cadastroProjetoForm');
            form.addEventListener('submit', function(event) {
                event.preventDefault(); 

               
                var nomeGrupo = Document.getElementById('nomeGrupo').value;
                var numeroCodigo = document.getElementById('numeroCodigo').value;
                var qualAtividade = document.getElementById('qualAtividade').value;
                var quemResponsavel = document.getElementById('quemResponsavel').value;
                var tempoSprint = document.getElementById('tempoSprint').value;
                var projetoDependencia = document.getElementById('projetoDependencia').value;

                var storedNomeGrupo = localStorage.getItem('nomeGrupo');
      
                if (nomeGrupo && numeroCodigo && qualAtividade && quemResponsavel && tempoSprint && projetoDependencia) {
                    var tabelaProjetos = document.getElementById('tabelaProjetos');
                    var newRow = tabelaProjetos.insertRow();

                    
                    newRow.innerHTML = `
                        <td>${numeroCodigo}</td>
                        <td>${qualAtividade}</td>
                        <td>${quemResponsavel}</td>
                        <td>${tempoSprint}</td>
                        <td>${projetoDependencia}</td>
                        <td>
                            <button onclick="window.location.href='EditarProjeto.html'" class="btn">Editar</button>
                            <button onclick="window.location.href='RemoverProjeto.html'" class="btn red">Remover</button>
                        </td>
                    `;
                    window.location.href = "TabelaGerenciamento.html";

                } 
                
                if (nomeGrupo === storedNomeGrupo) {
                    window.location.href = "TabelaGerenciamento.html";
                }

                else {
                    alert('Por favor, preencha todos os campos obrigatórios.');
                }
            });
        });