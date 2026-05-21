# Inventario inicial do port Rust

Status permitidos: `area alvo`, `a portar`, `referencia`,
`suporte de validacao`, `fora do runtime`, `pendente de decisao`.

Nenhuma entrada desta fase pode ser marcada como concluida sem evidencia de
validacao associada.

Este inventario cobre as raizes rastreadas no repositorio. O inventario
granular dos caminhos legados sob `ClientLibrary/`, `src/`, `src/MuEditor/`,
`src/bin/`, `src/ThirdParty/`, `src/dependencies/` e `tests/` vive em
`port_rust/crates/mu_test_support/src/source_inventory.rs`.

| Entrada | Status inicial | Evidencia exigida | Observacao |
|---|---|---|---|
| `.github/` | `suporte de validacao` | CI Rust especifico ou decisao documentada | Workflows atuais ficam como suporte e referencia de validacao. |
| `.gemini/` | `fora do runtime` | Evidencia de uso ou decisao de descarte | Configuracao assistiva, sem impacto direto no runtime. |
| `.features/` | `fora do runtime` | Decisao de manter apenas como workflow | Specs/planos operacionais nao entram no cliente Rust. |
| `.idea/` | `fora do runtime` | Decisao de nao aplicabilidade | Metadados de IDE nao entram no runtime nem no pacote Rust. |
| `.memory/` | `fora do runtime` | Decisao de manter apenas como memoria operacional | Regras/tarefas de agentes nao entram no runtime nem no pacote Rust. |
| `ClientLibrary/` | `referencia` | Testes/provas de equivalencia de rede | Base C# para contratos cliente-servidor existentes. |
| `ConstantsReplacer/` | `referencia` | Decisao sobre substituicao ou descarte | Ferramenta auxiliar atual, sem migracao nesta fase. |
| `cmake/` | `suporte de validacao` | Decisao de coexistencia/remocao futura | Continua servindo ao build legado. |
| `docs/` | `referencia` | Docs Rust atualizados quando houver superficie | Documentacao atual orienta comportamento esperado. |
| `port_rust/` | `area alvo` | Cargo workspace, testes e docs alinhados ao plano | Workspace que recebe as camadas portadas e as ferramentas Rust. |
| `scripts/` | `referencia` | Decisao por script na matriz abaixo | Conversores, geradores e empacotamento alimentam `mu_asset_pipeline` e o pacote Rust. |
| `src/` | `referencia` | Inventario granular e validacao por area | Cliente C++ legado permanece como referencia funcional. |
| `tests/` | `suporte de validacao` | Equivalente Rust ou fixture comparativa | Testes existentes orientam equivalencia futura. |
| `.editorconfig` | `suporte de validacao` | Politica Rust equivalente ou reutilizacao | Pode orientar formatacao do port. |
| `.gitattributes` | `suporte de validacao` | Decisao de atributos para artefatos Rust | Regras de atributos continuam na raiz. |
| `.gitignore` | `suporte de validacao` | Regras para `target/` se necessario | Avaliar ignore especifico quando Cargo gerar artefatos. |
| `.gitmodules` | `referencia` | Decisao sobre submodulos/assets futuros | Submodulos atuais seguem como referencia. |
| `AGENTS.md` | `referencia` | Regras aplicadas ao port | Instrucoes obrigatorias para agentes e humanos. |
| `CLAUDE.md` | `referencia` | Confirmacao de apontamento para `AGENTS.md` | Mantem ponte para instrucoes principais. |
| `CMakeLists.txt` | `referencia` | Decisao de coexistencia com Cargo | Build raiz legado nao muda nesta fase. |
| `CMakePresets.json` | `referencia` | Decisao de equivalencia de presets | Presets atuais continuam para CMake. |
| `README.md` | `referencia` | README Rust alinhado ao status real | README raiz segue como entrada do projeto atual. |
| `TRANSLATION_SYSTEM_INTEGRATION.md` | `referencia` | Plano futuro de i18n Rust | Referencia para internacionalizacao futura. |
| `stylecop.json` | `fora do runtime` | Decisao de nao aplicabilidade ou equivalente | Regra .NET, sem uso direto no runtime Rust. |
| `toolchain-x64.cmake` | `referencia` | Decisao de toolchain Rust Windows x64 futura | Referencia para ambiente Windows x64. |
| `toolchain-x86.cmake` | `referencia` | Decisao sobre suporte x86 futuro | Referencia para suporte x86 futuro. |

## Decisoes por script

| Entrada | Decisao | Evidencia exigida | Observacao |
|---|---|---|---|
| `scripts/client_converter/` | Reutilizar como referencia ate paridade Rust | Relatorios do `mu_asset_pipeline` ou fixtures equivalentes | Converte texturas, modelos, terreno, mapas, objetos e valida assets. |
| `scripts/generate_items/` | Portar ou absorver no `mu_asset_pipeline` | Catalogo gerado comparado com fontes legadas | Gera catalogo compartilhado de itens. |
| `scripts/generate_monster_spots/` | Portar ou absorver no `mu_asset_pipeline` | Fixtures de monstros/spawns comparadas com fontes legadas | Gera dados de monstros e spots por mundo. |
| `scripts/generate_skill_visual_matrix.sh` | Usar como fixture ou descartar por decisao documentada | Matriz visual Rust equivalente ou descarte registrado | Ajuda a auditar representacao visual de skills. |
| `scripts/normalize_filenames/` | Usar como referencia ou descartar por decisao documentada | Pipeline Rust demonstrando normalizacao de caminhos | Apoia higiene de nomes de assets. |
| `scripts/package_rust_client.py` | Manter ate empacotamento Rust definitivo | Smoke test de pacote ou substituto Rust documentado | Monta o bundle release com executavel e assets convertidos. |
| `scripts/remaster/` | Fora do runtime; experimento opt-in | Decisao explicita antes de qualquer uso em release | Prototipos de remasterizacao de texturas. |
| `scripts/remaster_glb.py` | Fora do runtime; experimento opt-in | Decisao explicita antes de qualquer uso em release | Remasteriza texturas embutidas/externas em GLB via API. |
| `scripts/remaster_glb_v2.py` | Fora do runtime; experimento opt-in | Decisao explicita antes de qualquer uso em release | Remasteriza GLB completo e valida candidato. |
| `scripts/remaster_image_test.py` | Fora do runtime; experimento opt-in | Decisao explicita antes de qualquer uso em release | PoC de remasterizacao de imagens via APIs externas. |
| `scripts/test_remaster_glb_v2.py` | Suporte de validacao do experimento | Teste Python quando o fluxo V2 for mantido | Testa preservacao de metadados do remaster V2. |
