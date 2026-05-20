# Controle remoto do cliente por HTTP

`mu_client` pode expor um servidor HTTP local de controle quando iniciado com
`--control-http`.

Ele serve para testar e inspecionar o estado do cliente sem mexer na rede do
jogo. No modo grafico, o endpoint espelha o fluxo de login/server select/
character select/world da Bevy runtime e a shell visual correspondente; no
modo `--headless`, continua sendo um smoke server deterministico. As rotas
`friend`, `guild`, `duel`, `events` e `gens` tambem espelham as shells visiveis, e
`friend` e `guild` aceitam comandos de subview para testar as telas internas.
Quando a sessao responde com as listas sociais, o runtime sobrepoe o
roster/score/roles decodificados no shell correspondente ate o logout ou
disconnect.
`character-create` abre a shell visivel de criacao, e `create-character`
submete o nome informado para o worker de bootstrap.

## Como iniciar

```bash
rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- \
  --control-http 127.0.0.1:0
```

Para smoke headless, use:

```bash
rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- \
  --headless \
  --control-http 127.0.0.1:0
```

O binario imprime o estado inicial e a URL efetiva do servidor. Se a
validacao de assets falhar no boot, o estado inicial sera `asset-check-failed`
e o servidor continua disponivel para inspeção local.

## Consultar estado

`GET /state` e `GET /` retornam JSON com o estado atual:

- `state`
- `ui_route`
- `session_phase`
- `last_command`
- `selected_character_name`
- `friend_name`
- `guild_master_player_id`
- `guild_player_name`
- `guild_role`
- `guild_assignment_type`
- `friend_screen_state`
- `guild_screen_state`
- `vault_money_amount`
- `inventory_move_from_slot`
- `inventory_move_to_slot`
- `command_count`

Exemplo:

```json
{"state":"ready-for-login","ui_route":"login","session_phase":"ready-for-login","last_command":null,"selected_character_name":null,"friend_name":null,"guild_master_player_id":null,"guild_player_name":null,"guild_role":null,"guild_assignment_type":null,"friend_screen_state":null,"guild_screen_state":null,"vault_money_amount":null,"inventory_move_from_slot":null,"inventory_move_to_slot":null,"command_count":0}
```

## Enviar comandos

`POST /command?name=...` aceita estes comandos:

- `boot`
- `asset-check-failed`
- `ready-for-login`
- `server-select`
- `options`
- `character-select`
- `character-create`
- `create-character`
- `select-character`
- `loading`
- `world`
- `chat`
- `npc`
- `shop`
- `game-shop`
- `trade`
- `party`
- `gate`
- `events`
- `gens`
- `friend`
- `friend-add`
- `friend-delete`
- `friend-roster`
- `friend-inbox`
- `friend-compose`
- `friend-chat-rooms`
- `guild`
- `guild-summary`
- `guild-members`
- `guild-union`
- `guild-no-guild`
- `guild-error`
- `guild-join`
- `guild-role-assign`
- `vault-deposit`
- `vault-withdraw`
- `inventory-move`
- `duel`
- `quests`
- `mu-helper`
- `login-success`
- `login-failure`
- `logout-login`
- `logout-character`
- `disconnect`
- `exit`
- `ping`

`server-select`, `options`, `character-select`, `character-create`,
`create-character`, `loading`, `world`, `chat`, `npc`, `select-character`, `shop`, `game-shop`,
`trade`, `party`, `gate`, `events`, `gens`, `friend`, `friend-roster`, `friend-inbox`,
`friend-compose`, `friend-chat-rooms`, `guild`, `guild-summary`,
`guild-members`, `guild-union`, `guild-no-guild`, `guild-error`,
`guild-role-assign`, `duel`, `quests`, `mu-helper`, `login-success` e
`login-failure` alteram a
rota/session state do runtime grafico. `options` abre a janela compartilhada
de options no auth shell. `character-create` abre a shell visivel de
criacao com lista base de classes, prompt de nome e botoes create/cancel.
`create-character` envia o nome recebido para o session worker e continua o
bootstrap apenas quando o servidor confirma a criacao; o nome pode vir no
body bruto, em `character=` ou como texto puro. Nomes com menos de 4
caracteres ou ausentes retornam `400` no control plane. `select-character`
envia o nome recebido para o session worker e continua o bootstrap apenas
quando o personagem for nomeado.
Se o nome vier vazio, a resposta sera `400` e o cliente continua em
character select.
`chat` abre a shell visivel de chat, que agora aceita texto digitado e Enter
para enviar mensagem publica quando a sessao esta logada.
`mu-helper` abre a shell visivel do MU Helper com o snapshot existente do
runtime. `duel` abre a shell visivel de duel com o snapshot existente do
runtime. `events` abre a shell visivel de events com o snapshot existente do
EventManager. `gens` abre a shell visivel de Gens com o snapshot existente do
GensManager e, quando a sessao esta logada, pede uma vez o ranking live,
hidrata o titulo local da classe Gens pela tabela legacy de 14 ranks e
reaplica o snapshot decodificado quando a resposta chega.
`friend-roster`, `friend-inbox`, `friend-compose` e
`friend-chat-rooms` selecionam as subvisoes da janela de friend;
`friend-add` e `friend-delete` enviam as requisicoes de add/delete do friend
atraves da sessao viva e aceitam o nome no body ou em `friend=`; se o nome
vier vazio, a resposta sera `400`.
`guild-summary`, `guild-members`, `guild-union`, `guild-no-guild` e
`guild-error` selecionam as subvisoes da janela de guild. `guild-join` envia o
pacote de join da guild atraves da sessao viva e aceita o guild master player
ID no body ou em `master_id=`; se o payload vier incompleto, a resposta sera
`400`. `guild-role-assign` envia o pacote de role assignment da guild
atraves da sessao viva e aceita o player no body ou em `player=`, o role em
`role=` e o tipo em `type=`; se o payload vier incompleto, a resposta sera
`400`. `vault-deposit` e `vault-withdraw` atualizam a rota visivel para
inventory, aceitam o valor no body ou em `amount=` e enviam o pacote de
transferencia de vault pela sessao viva; se o valor vier ausente ou zero, a
resposta sera `400`. `inventory-move` atualiza a rota visivel para
inventory, aceita `from_slot=` e `to_slot=` com os slots lineares do
inventory e envia o pacote de movimento de item pela sessao viva; se o
payload vier incompleto, a resposta sera `400`. Ao abrir `guild-union` com a
sessao logada, o runtime tambem envia uma vez a requisicao de alliance list
antes de manter a shell visivel. `exit` atualiza o estado para `exit`,
encerra o servidor e solicita saida do runtime grafico. Os demais apenas
atualizam o snapshot.

O comando tambem pode vir no corpo da requisicao como `name=...` ou como texto
puro.

Exemplos:

```bash
curl http://127.0.0.1:12345/state
curl -X POST 'http://127.0.0.1:12345/command?name=ready-for-login'
curl -X POST 'http://127.0.0.1:12345/command?name=server-select'
curl -X POST 'http://127.0.0.1:12345/command?name=options'
curl -X POST 'http://127.0.0.1:12345/command?name=login-success'
curl -X POST 'http://127.0.0.1:12345/command?name=character-create'
curl -X POST 'http://127.0.0.1:12345/command?name=create-character&character=Astra'
curl -X POST 'http://127.0.0.1:12345/command?name=chat'
curl -X POST 'http://127.0.0.1:12345/command?name=select-character' -d 'Astra'
curl -X POST 'http://127.0.0.1:12345/command?name=select-character&character=Astra'
curl -X POST 'http://127.0.0.1:12345/command?name=friend-add&friend=Astra'
curl -X POST 'http://127.0.0.1:12345/command?name=friend-delete&friend=Astra'
curl -X POST 'http://127.0.0.1:12345/command?name=guild-join&master_id=4660'
curl -X POST 'http://127.0.0.1:12345/command?name=guild-role-assign&player=Astra&role=64&type=2'
curl -X POST 'http://127.0.0.1:12345/command?name=vault-deposit&amount=250'
curl -X POST 'http://127.0.0.1:12345/command?name=vault-withdraw&amount=125'
curl -X POST 'http://127.0.0.1:12345/command?name=inventory-move&from_slot=0&to_slot=1'
curl -X POST 'http://127.0.0.1:12345/command?name=npc'
curl -X POST 'http://127.0.0.1:12345/command?name=shop'
curl -X POST 'http://127.0.0.1:12345/command?name=game-shop'
curl -X POST 'http://127.0.0.1:12345/command?name=trade'
curl -X POST 'http://127.0.0.1:12345/command?name=party'
curl -X POST 'http://127.0.0.1:12345/command?name=gate'
curl -X POST 'http://127.0.0.1:12345/command?name=events'
curl -X POST 'http://127.0.0.1:12345/command?name=gens'
curl -X POST 'http://127.0.0.1:12345/command?name=friend'
curl -X POST 'http://127.0.0.1:12345/command?name=friend-compose'
curl -X POST 'http://127.0.0.1:12345/command?name=friend-chat-rooms'
curl -X POST 'http://127.0.0.1:12345/command?name=guild'
curl -X POST 'http://127.0.0.1:12345/command?name=guild-members'
curl -X POST 'http://127.0.0.1:12345/command?name=guild-union'
curl -X POST 'http://127.0.0.1:12345/command?name=duel'
curl -X POST 'http://127.0.0.1:12345/command?name=quests'
curl -X POST 'http://127.0.0.1:12345/command?name=mu-helper'
curl -X POST 'http://127.0.0.1:12345/command' -d 'name=ping'
curl -X POST 'http://127.0.0.1:12345/command' -d 'exit'
```

## Respostas

- `200` para estado consultado e comandos aceitos.
- `400` para comando ausente ou desconhecido.
- `404` para rota inexistente.

`GET /__shutdown` existe apenas para encerramento interno do servidor.
