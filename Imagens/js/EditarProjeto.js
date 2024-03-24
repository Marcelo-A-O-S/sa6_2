document.addEventListener('DOMContentLoaded', function() {
    const urlParams = new URLSearchParams(window.location.search);
    const numeroCodigo = urlParams.get('numeroCodigo');
    const qualAtividade = urlParams.get('qualAtividade');
    const quemResponsavel = urlParams.get('quemResponsavel');
    const tempoSprint = urlParams.get('tempoSprint');
    const projetoDependencia = urlParams.get('projetoDependencia');

    document.getElementById('numeroCodigo').value = numeroCodigo;
    document.getElementById('qualAtividade').value = qualAtividade;
    document.getElementById('quemResponsavel').value = projetoResponsavel;
    document.getElementById('tempoSprint').value = tempoSprint;
    document.getElementById('projetoDependencia').value = projetoDependencia;


   window.location.href = `TabelaGerenciamento.html?numeroCodigo=${numeroCodigo}&qualAtividade=${encodeURIComponent(qualAtividade)}&quemResponsavel=${encodeURIComponent(quemResponsavel)}&tempoSprint=${encodeURIComponent(tempoSprint)}&projetoDependencia=${encodeURIComponent(projetoDependencia)}`;
});
