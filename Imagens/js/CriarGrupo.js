
import {Apiaddgrupos} from "../../js.api/Apiaddgrupo.js"
import {Grupo} from "../../js.api/models/Grupo.js"
document.addEventListener('DOMContentLoaded', function() {
  var elem = document.querySelector('.container');
  elem.scrollIntoView({ behavior: 'smooth', block: 'start' });

  var form = document.getElementById('cadastroGrupoForm');
  form.addEventListener('submit',async function(event) {
      event.preventDefault(); 
      var nomeGrupo = document.getElementById('nomeGrupo').value;
      var senhaGrupo = document.getElementById('senhaGrupo').value;
      var gerenciaProjeto = document.getElementById('gerenciaProjeto').value;
      var scrumMaster = document.getElementById('scrumMaster').value;
      var productOwner = document.getElementById('productOwner').value;
      var equipeDesenvolvimento = document.getElementById('equipeDesenvolvimento').value;
      var descricaoGrupo = document.getElementById('descricaoGrupo').value;
      if (nomeGrupo && senhaGrupo && gerenciaProjeto && scrumMaster && productOwner && equipeDesenvolvimento && descricaoGrupo) {
          localStorage.setItem('nomeGrupo', nomeGrupo);
          localStorage.setItem('senhaGrupo', senhaGrupo);
          const apiaddgrupo = new Apiaddgrupos();
          let grupo = new Grupo()
          grupo.nomeGrupo = nomeGrupo;
          grupo.senhaGrupo = senhaGrupo;
          grupo.ScrumMaster = scrumMaster;
          grupo.gerenciaProjeto = gerenciaProjeto;
          grupo.productOwner = productOwner;
          grupo.equipeDesenvolvimento = equipeDesenvolvimento;
          grupo.descricaoGrupo = descricaoGrupo;
          let response = await apiaddgrupo.getApiaddgrupos(grupo)
          console.log(response);
          alert('Grupo criado com sucesso!');
          window.location.href = 'EntrarGrupo.html';
      } else {
          alert('Por favor, preencha todos os campos obrigatórios.');
      }
  });
});