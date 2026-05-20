# Port Rust

`port_rust` e a area paralela para o port incremental em Rust do projeto.
O cliente ja possui um boot grafico inicial em Bevy, mas ainda nao e uma
sessao de jogo completa.

## Status

- Inicializado como workspace Cargo isolado.
- Incremental: o cliente legado continua sendo a referencia funcional.
- O runtime grafico inicial abre via Bevy no modo sem `--headless`; as camadas
  de UI, assets, audio, gameplay, editor/admin e rede ainda seguem em paridade
  incremental antes de uma sessao jogavel completa. Esse boot ja mostra a
  shell visual de login/server select/character select, faz a transicao
  login -> server select -> character select -> world quando o fluxo de rede
  entrega o handoff do mapa; depois de `login-success`, ele pede
  automaticamente a character list usando o byte legado do idioma
  (`en`/`eng` -> `0`, `pt`/`por` -> `1`, `es`/`spn` -> `2`). Quando o roster
  chega, ele fica em character select ate receber um `select-character`
  explicito com o nome do personagem; o mesmo comando pode vir pelo
  `--control-http` usando o nome no body ou em `character=`. A rota visivel
  `character-create` tambem pode ser aberta pelo comando `character-create`
  no `--control-http`, mostrando a lista base de classes, o prompt de nome e
  os botoes create/cancel; `create-character` envia o nome para o worker,
  valida o minimo de 4 chars no control plane e reage ao retorno de
  sucesso/falha da rede. Na tela de character select, use `Up`/`Down` ou
  `Left`/`Right` e `Enter` para escolher um personagem sem depender do
  control-http. No world route, mostra uma
  world shell 3D
  visivel com terreno heightfield derivado dos dados do bundle, uma superficie
  em camadas vinda dos dois primeiros slots convertidos do bundle mais o alpha
  map, o lightmap do terreno do bundle, marcadores de local/remote player
  ancorados na superficie visivel do terreno e modelos convertidos reais para
  objects, NPCs e monstros.
- A janela compartilhada de Options tambem abre como a rota visivel
  `options` e pode ser acionada pelo smoke local de `--control-http`.
- A camera do world route agora segue o avatar local para manter o movimento
  enquadrado enquanto a cena 3D esta ativa; a roda do mouse ajusta o zoom e
  o valor fica salvo em `[Camera] zoom` ao sair do cliente grafico.
- O world route tambem mostra uma HUD visivel com as gauges e botoes do frame
  principal legado enquanto a cena 3D esta ativa.
- O world route tambem mostra o stack visual de HUD com chat, minimap e
  hotkeys legados enquanto a cena 3D esta ativa.
- O world route agora tambem cria um avatar local, envia o movimento via
  sessao viva e aplica as respostas autoritativas no runtime; a predicao local
  continua imediata para manter a resposta do controle.
- O world shell agora prioriza entidades proximas quando amostra object,
  npc, monster e remote player visiveis, em vez de pegar as primeiras
  entradas arbitrarias, para deixar a cena 3D mais parecida com a area do
  jogador sem perder o limite de performance.
- Se os slots de textura, o alpha map ou o lightmap do terreno nao estiverem
  disponiveis, o shell volta para o material solido de fallback sem sair do
  world route.
- A tecla `Tab` alterna entre o world route e a shell visual de inventory
  enquanto a sessao de mundo estiver ativa.
- As rotas `Npc` e `Shop` agora mostram shells visuais proprias e podem ser
  abertas pelo `--control-http` para smoke local.
- A rota `Trade` agora mostra uma shell visual propria e pode ser aberta pelo
  `--control-http` para smoke local.
- A rota `Party` agora mostra uma shell visual propria e pode ser aberta pelo
  `--control-http` para smoke local.
- A rota `Gate` agora mostra uma shell visual propria e pode ser aberta pelo
  `--control-http` para smoke local.
- A rota `Quests` agora mostra uma shell visual propria e pode ser aberta
  pelo `--control-http` para smoke local.
- A rota `Chat` agora mostra uma shell visual propria com rascunho de texto;
  quando a sessao esta logada, digitar texto e apertar Enter envia uma
  mensagem publica. Ela pode ser aberta pelo `--control-http` para smoke
  local.
- O modo `--control-http` e um servidor HTTP local de teste para consultar
  estado e enviar comandos; no runtime grafico, ele tambem espelha o fluxo de
  login/server select/character select/world. Detalhes em
  `docs/control-http.md`.
- O binario `mu_fake_server` e um servidor fake de connect-server para testes
  locais, sem persistencia, configurado inteiramente por CLI; detalhes em
  `docs/face_server.md`.
- As configuracoes persistidas ficam em `config/client.toml`; elas cobrem
  video, audio, camera, controles, performance, rede e idioma, e nao gravam
  segredos como senha ou token. O zoom da camera fica salvo em `[Camera] zoom`.
- O sistema de camera e os presets de zoom estao documentados em
  `docs/camera-system.md` e `docs/camera-config-zoom.md`.
- A janela de options compartilhada usa o mesmo `config/client.toml` em login, character
  select e gameplay; detalhes em `docs/options-window.md`.
- Quando `--asset-root` e informado, o root precisa conter
  `manifest.muasset.json` e os assets convertidos com hashes coerentes; falhas
  de manifest ou checksum mantem o cliente em `asset-check-failed`.
- Nenhum arquivo de implementacao legado deve ser movido para ca sem uma fase planejada.
- A progressao de personagem e a curva de master level do port Rust estao
  documentadas em `docs/character-progression.md`.
- A camada compartilhada de combate, buffs e experiencia esta documentada em
  `docs/combat-buffs-experience.md`.
- A camada de skills, efeitos, particulas e audio esta documentada em
  `docs/skills-effects-audio.md`.
- A camada de pets, summons e mounts esta documentada em
  `docs/pets-summons-mounts.md`.
- A camada de items, inventory, equipment e vault esta documentada em
  `docs/inventory.md`.
- A camada de NPC/dialog/shop esta documentada em `docs/npc-dialog-shop.md`.
- A camada de gatekeeper/castle access esta documentada em
  `docs/gatekeeper.md`.
- A camada de trade, player shop e mail esta documentada em
  `docs/trade-player-shop-mail.md`.
- A camada de friend e guild esta documentada em `docs/friend-guild.md`.
- A camada de friend e guild agora tambem abre shells visiveis no runtime
  grafico e pode ser acionada por `friend` e `guild` no smoke local; os
  comandos `friend-roster`, `friend-inbox`, `friend-compose`,
  `friend-chat-rooms`, `guild-summary`, `guild-members`, `guild-union`,
  `guild-no-guild`, e `guild-error` selecionam as subvisoes visiveis. Quando
  a sessao envia as respostas de listagem, o runtime decodifica friend/guild
  e sobrepoe o roster/score/roles live no mesmo shell.
- Quando `friend` ou `guild` abre com a sessao logada, o runtime envia uma
  vez a requisicao de listagem correspondente antes de manter a shell
  visivel, e limpa o snapshot decodificado no logout ou disconnect.
- A camada de duel esta documentada em `docs/quests-events-duel-gens.md` e
  agora tambem abre uma shell visivel no runtime grafico, acionavel por
  `duel` no smoke local.
- A camada de party esta documentada em `docs/party-ui.md` e cobre as
  snapshots de party info/list e o consumo do `PartyManager`.
- A camada de siege warfare esta documentada em `docs/siege-warfare.md` e
  cobre observer, soldier e commander na janela de castle siege.
- A camada de world entities usa fixtures para local/remote players, objetos
  estaticos, NPCs e monstros; o runtime de render consome os mesmos managers
  para manter a superficie de world/screenshot consistente.
- A camada de quests, events, duel e gens esta documentada em
  `docs/quests-events-duel-gens.md`.
- A camada de chat esta documentada em `docs/player-rust-client.md` e
  `docs/control-http.md` via a rota visual e o smoke local.
- A camada de GameShop esta documentada em `docs/game-shop.md` e cobre o
  runtime de catalog/details/storage/empty/error, a seguranca transacional, a
  UI snapshot do jogo e o shell visivel controlado por `game-shop`.
- A camada de MU Helper esta documentada em `docs/mu-helper.md` e cobre o
  modelo de configuracao, o runtime de execucao, os estados ativo/inativo/
  bloqueado e a shell visivel controlada por `mu-helper`.
- A camada de editor/admin equivalente esta documentada em
  `docs/admin-editor-rust.md` e cobre o gate de permissao, a rota `admin-core`,
  os item/skill editors, o shell do editor, o console, o dev editor e a entrada
  visivel quando o acesso e autorizado.

## Validacao local prevista

```bash
rtk cargo metadata --manifest-path port_rust/Cargo.toml --no-deps --format-version 1
rtk cargo fmt --manifest-path port_rust/Cargo.toml --all --check
rtk cargo test --manifest-path port_rust/Cargo.toml --workspace
rtk cargo clippy --manifest-path port_rust/Cargo.toml --workspace --all-targets -- -D warnings
rtk cargo run --manifest-path port_rust/Cargo.toml -p port_rust -- --status
rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0
rtk cargo build --manifest-path port_rust/Cargo.toml --release -p mu_client
rtk cargo run --manifest-path port_rust/Cargo.toml --release -p mu_client -- --headless
```

O comando `--status` deve imprimir uma mensagem contendo `initialized`,
`incremental` e `not playable`; isso ainda se refere a paridade jogavel
completa, nao ao boot grafico inicial.

O cliente Rust grafico inicial pode ser iniciado com:

```bash
rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client
```

O cliente Rust pode iniciar um servidor de controle local com:

```bash
rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0
```

O servidor expõe:

- `GET /state` para consultar o estado atual;
- `POST /command?name=boot|asset-check-failed|ready-for-login|server-select|
  character-select|character-create|create-character|select-character|loading|world|chat|npc|shop|game-shop|
  trade|mu-helper|login-success|login-failure|party|gate|friend|guild|duel|quests|logout-login|
  logout-character|disconnect|exit|ping` para
  mudar o estado ou encerrar o processo/runtime grafico.

`select-character` aceita o nome do personagem no body bruto ou em
`character=` e avanca o bootstrap para o envio do `select_character` pelo
session worker.

Ao iniciar, o binario imprime o estado inicial e a URL efetiva do servidor.

O servidor fake de connect-server pode ser iniciado com:

```bash
rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_fake_server -- \
  --bind 127.0.0.1:44405 \
  --server 0:100
```

Repita `--server` para adicionar mais entradas na lista de servidores. Cada
entrada usa o formato `CONNECT_INDEX:PERCENT`.

Veja tambem:

- `docs/control-http.md` para o fluxo completo do servidor HTTP de controle.
- `docs/face_server.md` para o fluxo completo do fake connect-server.
- `docs/character-progression.md` para a tabela de classes e as curvas de
  experiencia do personagem.
- `docs/camera-system.md` para o sistema de camera e
  `docs/options-window.md` para a janela de options compartilhada e a persistencia de
  configuracao.
- `docs/combat-buffs-experience.md` para os helpers de combate, buffs e
  experiencia compartilhada.
- `docs/skills-effects-audio.md` para o catalogo de skills, requisitos, delay,
  filas puras de efeitos/audio e a projecao de particulas.
- `docs/pets-summons-mounts.md` para as regras de pets, summons, mounts e os
  helpers de packet do slice.
- `docs/inventory.md` para o codec de item, as regras de inventory/equipment e
  o estado de vault.
- `docs/npc-dialog-shop.md` para o estado de dialogo NPC, buy/sell e repair de
  shop.
- `docs/gatekeeper.md` para o acesso gatekeeper, fee/public toggle e o fluxo
  de entrada do castelo.
- `docs/trade-player-shop-mail.md` para o estado de trade, player shop e mail.
- `docs/friend-guild.md` para as snapshots de friend, letter e guild.
- `docs/party-ui.md` para as snapshots de party info/list e o consumo do
  `PartyManager`.
- `docs/siege-warfare.md` para a snapshot de siege warfare e os modos
  observer, soldier e commander.
- `docs/quests-events-duel-gens.md` para o estado de quests, events, duel e
  gens.
- `docs/game-shop.md` para o runtime, a seguranca transacional e as snapshots
  de GameShop.
- `docs/mu-helper.md` para o modelo de configuracao do MU Helper, o runtime de
  execucao e os limites legados de salvamento.
- `docs/admin-editor-rust.md` para o gate de acesso e o shell do
  editor/admin equivalente.

## Dependencias dos scripts de port

As dependencias para reaproveitar `scripts/` estao documentadas em
`docs/script-dependencies.md`.

## Gate por fase

Antes de comecar cada fase do port, deve existir uma etapa de planejamento
individual com objetivo, escopo, arquivos planejados, comandos de validacao e
evidencias esperadas. Nenhuma fase de execucao deve comecar sem esse registro.

## Inventario

O inventario inicial esta em `port_rust/docs/inventory.md`. Ele lista as raizes
rastreadas do projeto, o status inicial e a evidencia exigida antes de qualquer
area ser considerada equivalente, descartada ou fora do runtime Rust.

O inventario granular dos caminhos legados vive em
`port_rust/crates/mu_test_support/src/source_inventory.rs` e e validado por
`port_rust/tests/rust/source_inventory.rs`. Ele cobre `ClientLibrary/`,
`src/`, `src/source/`, `src/MuEditor/`, `src/bin/`, `src/ThirdParty/`,
`src/dependencies/` e `tests/`.
