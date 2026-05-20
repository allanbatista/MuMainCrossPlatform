# Migracao dos GitHub Actions para Node 24
Status: READY_FOR_PLAN

## Objetivo
Atualizar os workflows em `.github/workflows` para usar versoes de actions com runtime Node 24, preservando o comportamento atual dos jobs.

## Usuarios e Jornadas
- Mantenedor de CI: atualiza os workflows e evita warnings de runtime obsoleto.
- Runner GitHub-hosted: executa as actions atualizadas sem mudar o resultado dos jobs.
- Revisor tecnico: confere que o diff ficou limitado a trocas de versao e que os YAMLs seguem validos.

## Inventario de Produto

| Saida | Rota/pasta | Action atual | Versao alvo | Estado |
|---|---|---|---|---|
| Checkout em workflows | `.github/workflows/*.yml` | `actions/checkout@v4` | `actions/checkout@v6` | a migrar |
| Cache em workflows | `.github/workflows/*.yml` | `actions/cache@v4` | `actions/cache@v5` | a migrar |
| Upload de artefato | `.github/workflows/*.yml` | `actions/upload-artifact@v4` | `actions/upload-artifact@v6` | a migrar |
| Claude automation | `.github/workflows/claude.yml` | `anthropics/claude-code-action@v1` | sem alteracao | fora do escopo da troca de Node direta |

## Criterios de Aceitacao

- Todas as referencias diretas a `actions/checkout`, `actions/cache` e `actions/upload-artifact` nos workflows sao atualizadas para releases Node24-compatible.
- Nao restam referencias `@v4` para essas actions.
- Os workflows continuam validos em YAML.
- O diff nao altera logica de CI, apenas a versao das actions.

## Escopo

Inclui os arquivos em `.github/workflows` que usam actions mantidas pelo GitHub.

Fora do escopo: `dtolnay/rust-toolchain@stable`, a implementacao interna de actions compostas de terceiros, e qualquer mudanca de job/script nao relacionada ao runtime Node.

## Perguntas em Aberto

Nenhuma.
