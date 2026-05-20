# Controle remoto do cliente por HTTP

`mu_client` pode expor um servidor HTTP local de controle quando iniciado com
`--control-http`.

Ele serve para testar e inspecionar o estado do cliente sem mexer na rede do
jogo. No modo grafico, o endpoint espelha o fluxo de login/server select/
character select/world da Bevy runtime e a shell visual correspondente; no
modo `--headless`, continua sendo um smoke server deterministico. As rotas
`friend`, `guild` e `duel` tambem espelham as shells visiveis, e `friend`
e `guild` aceitam comandos de subview para testar as telas internas.

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
- `friend_screen_state`
- `guild_screen_state`
- `command_count`

Exemplo:

```json
{"state":"ready-for-login","ui_route":"login","session_phase":"ready-for-login","last_command":null,"friend_screen_state":null,"guild_screen_state":null,"command_count":0}
```

## Enviar comandos

`POST /command?name=...` aceita estes comandos:

- `boot`
- `asset-check-failed`
- `ready-for-login`
- `server-select`
- `options`
- `character-select`
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
- `friend`
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

`server-select`, `options`, `character-select`, `loading`, `world`, `chat`,
`npc`, `select-character`, `shop`, `game-shop`, `trade`, `party`, `gate`,
`friend`, `friend-roster`, `friend-inbox`, `friend-compose`,
`friend-chat-rooms`, `guild`, `guild-summary`, `guild-members`,
`guild-union`, `guild-no-guild`, `guild-error`, `duel`, `quests`,
`mu-helper`, `login-success` e `login-failure` alteram a
rota/session state do runtime grafico. `options` abre a janela compartilhada
de options no auth shell. `select-character` envia o nome recebido para o
session worker e continua o bootstrap apenas quando o personagem for
nomeado.
Se o nome vier vazio, a resposta sera `400` e o cliente continua em
character select.
`chat` abre a shell visivel de chat, que agora aceita texto digitado e Enter
para enviar mensagem publica quando a sessao esta logada.
`mu-helper` abre a shell visivel do MU Helper com o snapshot existente do
runtime. `duel` abre a shell visivel de duel com o snapshot existente do
runtime. `friend-roster`, `friend-inbox`, `friend-compose` e
`friend-chat-rooms` selecionam as subvisoes da janela de friend;
`guild-summary`, `guild-members`, `guild-union`, `guild-no-guild` e
`guild-error` selecionam as subvisoes da janela de guild. `exit`
atualiza o estado para `exit`, encerra o servidor e solicita saida do runtime
grafico. Os demais apenas atualizam o snapshot.

O comando tambem pode vir no corpo da requisicao como `name=...` ou como texto
puro.

Exemplos:

```bash
curl http://127.0.0.1:12345/state
curl -X POST 'http://127.0.0.1:12345/command?name=ready-for-login'
curl -X POST 'http://127.0.0.1:12345/command?name=server-select'
curl -X POST 'http://127.0.0.1:12345/command?name=options'
curl -X POST 'http://127.0.0.1:12345/command?name=login-success'
curl -X POST 'http://127.0.0.1:12345/command?name=chat'
curl -X POST 'http://127.0.0.1:12345/command?name=select-character' -d 'Astra'
curl -X POST 'http://127.0.0.1:12345/command?name=select-character&character=Astra'
curl -X POST 'http://127.0.0.1:12345/command?name=npc'
curl -X POST 'http://127.0.0.1:12345/command?name=shop'
curl -X POST 'http://127.0.0.1:12345/command?name=game-shop'
curl -X POST 'http://127.0.0.1:12345/command?name=trade'
curl -X POST 'http://127.0.0.1:12345/command?name=party'
curl -X POST 'http://127.0.0.1:12345/command?name=gate'
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
