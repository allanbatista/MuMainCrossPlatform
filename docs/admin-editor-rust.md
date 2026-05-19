# Editor/Admin Rust

O port Rust modela a entrada do editor/admin como um gate de autorizacao
separado do core do editor.

## Gate de acesso

- `mu_editor_admin::AdminAuthState` guarda se o cliente esta em modo release e
  se o acesso administrativo foi autorizado.
- Em modo release, a entrada do editor/admin fica oculta.
- Fora do modo release, usuarios sem permissao veem estado bloqueado.
- Usuarios autorizados veem a entrada pronta para abrir o editor.

## UI

- `mu_ui::admin_screen` renderiza a rota `editor-admin` a partir do estado de
  autorizacao.
- As snapshots cobrem os estados `hidden`, `blocked` e `ready`.

## Uso

- Builds de jogador ficam em modo release e nao mostram a entrada do editor.
- Builds de dev/admin podem liberar o acesso e abrir o editor quando a
  autorizacao estiver ativa.

## Proxima etapa

- `admin-core`, `admin-item-editor` e `admin-skill-editor` continuam a
  expansao do editor/admin equivalente.
