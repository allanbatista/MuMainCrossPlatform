# Editor/Admin Rust

O port Rust modela a entrada do editor/admin como um gate de autorizacao e
como o shell do editor em si.

## Gate de acesso

- `mu_editor_admin::AdminAuthState` guarda se o cliente esta em modo release e
  se o acesso administrativo foi autorizado.
- Em modo release, a entrada do editor/admin fica oculta.
- Fora do modo release, usuarios sem permissao veem estado bloqueado.
- Usuarios autorizados veem a entrada pronta para abrir o editor.

## Core do editor

- `mu_editor_admin::EditorCoreState` guarda o shell aberto/fechado, as flags dos
  paineis item/skill/dev/console, os hover flags e as opcoes de idioma.
- `mu_ui::admin_core_screen` renderiza a rota `admin-core` a partir do estado
  do shell, da console e do dev editor.
- As snapshots cobrem shell fechado/aberto, toolbar actions e a visibilidade
  dos paineis.

## Console

- `mu_editor_admin::EditorConsoleState` guarda os buffers separados de editor e
  jogo e o caminho do log diario.
- O console continua splitado em dois paineis, com copy/clear por buffer e
  limpeza imediata dos logs em memoria.

## Dev editor

- `mu_editor_admin::DevEditorState` agrupa os overrides de camera default e
  orbital, os toggles de render, os flags de debug, o cull radius de item, as
  distancias de login e os dados de diagnostico grafico.
- O reset da camera default usa `CameraConfig::for_main_scene_default_camera()`;
  o orbital re-semeia o trapezio de visao a partir da camera da cena principal.
- `DevEditorGraphicsState` formata a string de debug copiada para a clipboard.

## UI

- `mu_ui::admin_screen` renderiza a rota `editor-admin` a partir do estado de
  autorizacao.
- As snapshots cobrem os estados `hidden`, `blocked` e `ready`.

## Uso

- Builds de jogador ficam em modo release e nao mostram a entrada do editor.
- Builds de dev/admin podem liberar o acesso e abrir o editor quando a
  autorizacao estiver ativa.

## Proxima etapa

- `admin-item-editor` e `admin-skill-editor` continuam a expansao do
  editor/admin equivalente.
