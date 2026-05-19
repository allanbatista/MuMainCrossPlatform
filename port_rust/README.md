# Port Rust

`port_rust` e a area paralela para iniciar o port incremental em Rust do projeto.
Esta etapa cria apenas o scaffold inicial: ainda e nao jogavel / not playable.

## Status

- Inicializado como workspace Cargo isolado.
- Incremental: o cliente legado continua sendo a referencia funcional.
- Sem runtime jogavel, UI, assets, audio ou editor completos em Rust nesta fase;
  o crate `mu_gameplay` ja porta o estado isolado de duelo, mas ainda nao ha
  loop de jogo jogavel.
- O modo `--control-http` e um servidor HTTP local de teste para consultar
  estado e enviar comandos; detalhes em `docs/control-http.md`.
- O binario `mu_fake_server` e um servidor fake de connect-server para testes
  locais, sem persistencia, configurado inteiramente por CLI; detalhes em
  `docs/face_server.md`.
- As configuracoes persistidas ficam em `config/client.toml`; elas cobrem
  video, audio, camera, controles, performance, rede e idioma, e nao gravam
  segredos como senha ou token. O zoom orbital fica salvo em `[Camera] zoom`.
- Quando `--asset-root` e informado, o root precisa conter
  `manifest.muasset.json` e os assets convertidos com hashes coerentes; falhas
  de manifest ou checksum mantem o cliente em `asset-check-failed`.
- Nenhum arquivo de implementacao legado deve ser movido para ca sem uma fase planejada.
- A progressao de personagem e a curva de master level do port Rust estao
  documentadas em `docs/character-progression.md`.
- A camada compartilhada de combate, buffs e experiencia esta documentada em
  `docs/combat-buffs-experience.md`.
- A camada de skills, efeitos e audio esta documentada em
  `docs/skills-effects-audio.md`.

## Validacao local prevista

```bash
rtk cargo metadata --manifest-path port_rust/Cargo.toml --no-deps --format-version 1
rtk cargo fmt --manifest-path port_rust/Cargo.toml --check
rtk cargo test --manifest-path port_rust/Cargo.toml --workspace
rtk cargo run --manifest-path port_rust/Cargo.toml -p port_rust -- --status
rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0
```

O comando `--status` deve imprimir uma mensagem contendo `initialized`,
`incremental` e `not playable`.

O cliente Rust pode iniciar um servidor de controle local com:

```bash
rtk cargo run --manifest-path port_rust/Cargo.toml -p mu_client -- --headless --control-http 127.0.0.1:0
```

O servidor expõe:

- `GET /state` para consultar o estado atual;
- `POST /command?name=boot|asset-check-failed|ready-for-login|exit|ping` para
  mudar o estado ou encerrar o processo.

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
- `docs/combat-buffs-experience.md` para os helpers de combate, buffs e
  experiencia compartilhada.
- `docs/skills-effects-audio.md` para o catalogo de skills, requisitos, delay e
  filas puras de efeitos/audio.

## Dependencias dos scripts de port

As dependencias para reaproveitar `scripts/` estao documentadas em
`docs/script-dependencies.md`.

## Gate por fase

Antes de comecar cada fase do port, deve existir uma etapa de planejamento
individual com objetivo, escopo, arquivos planejados, comandos de validacao e
evidencias esperadas. Nenhuma fase de execucao deve comecar sem esse registro.

## Inventario

O inventario inicial esta em `docs/inventory.md`. Ele lista as areas atuais do
projeto, o status inicial e a evidencia exigida antes de qualquer area ser
considerada equivalente no port Rust.
