# Migracao dos GitHub Actions para Node 24
Status: READY_FOR_PLAN

## Objetivo
Atualizar os workflows em `.github/workflows` para usar versoes de actions com runtime Node 24, preservando o comportamento atual dos jobs.

## Usuarios e Jornadas
- Mantenedor de CI: atualiza os workflows e evita warnings de runtime obsoleto.
- Runner GitHub-hosted: executa as actions atualizadas sem mudar o resultado dos jobs.
- Revisor tecnico: confere que o diff ficou limitado a trocas de versao e que os YAMLs seguem validos.

## Inventario de Produto

| Saida | Slug/ID | Rotulo visivel | Rota/pasta | Action atual | Versao alvo | Filtros | States | Diferencas por persona | Estado |
|---|---|---|---|---|---|---|---|---|---|
| Checkout em workflows | `checkout-node24` | Checkout | `.github/workflows/*.yml` | `actions/checkout@v4` | `actions/checkout@v6` | nenhum | loading/success/error | nenhuma | a migrar |
| Cache em workflows | `cache-node24` | Cache | `.github/workflows/*.yml` | `actions/cache@v4` | `actions/cache@v5` | chave de cache do workflow | loading/restore/save/error | nenhuma | a migrar |
| Upload de artefato | `artifact-node24` | Upload de artefato | `.github/workflows/*.yml` | `actions/upload-artifact@v4` | `actions/upload-artifact@v6` | nome do artefato e caminho de upload | loading/success/error | nenhuma | a migrar |
| Claude automation | `claude-automation` | Claude automation | `.github/workflows/claude.yml` | `anthropics/claude-code-action@v1` | sem alteracao | nenhum | loading/success/error | nenhuma | fora do escopo |

## Nao Funcionais

- Seguranca: a mudanca toca apenas referencias de workflow; nao altera segredos, tokens ou logs.
- Disponibilidade: o comportamento dos jobs permanece o mesmo; apenas a versao das actions e atualizada.
- Acessibilidade/mobile: nao aplicavel.

## Criterios de Aceitacao

- AC-01. Todas as referencias diretas a `actions/checkout`, `actions/cache` e `actions/upload-artifact` nos workflows sao atualizadas para releases Node24-compatible.
- AC-02. Nao restam referencias `@v4` para essas actions.
- AC-03. Os workflows continuam validos em YAML.
- AC-04. O diff nao altera logica de CI, apenas a versao das actions.

## Escopo

Inclui os arquivos em `.github/workflows` que usam actions mantidas pelo GitHub.

Fora do escopo: `dtolnay/rust-toolchain@stable`, a implementacao interna de actions compostas de terceiros, e qualquer mudanca de job/script nao relacionada ao runtime Node.

## Open Questions

None.
