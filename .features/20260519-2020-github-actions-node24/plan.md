# Migracao dos GitHub Actions para Node 24
Status: READY_FOR_EXEC

## Interfaces / Contracts

- Inputs: os workflows em `.github/workflows/*.yml`.
- Saidas esperadas: atualizacao das referencias `uses:` para actions Node24-compatible.
- Referencias alvo:
  - `actions/checkout@v6`
  - `actions/cache@v5`
  - `actions/upload-artifact@v6`

## Inventario Tecnico

| Slug | Query | Components | Filter | Saida | Dataset/permission gating | Retailer/industry compatibility | Observacao |
|---|---|---|---|---|---|---|---|
| `checkout-node24` | `actions/checkout` | checkout step | `@v6` | repo checkout | N/A (workflow-only) | N/A | Node24-compatible release |
| `cache-node24` | `actions/cache` | cache step | `@v5` | cache restore/save | N/A (workflow-only) | N/A | Node24-compatible release |
| `artifact-node24` | `actions/upload-artifact` | artifact upload | `@v6` | artifact upload | N/A (workflow-only) | N/A | Node24-compatible release |

## Phases / Tasks

### F1. Atualizacao dos workflows

| ID | Owner | Planned files | Done when | Required evidence |
|---|---|---|---|---|
| F1.S1.T1 | local | `.github/workflows/claude.yml`, `.github/workflows/mingw-build-dev.yml`, `.github/workflows/mingw-build-pr.yml`, `.github/workflows/mingw-build.yml`, `.github/workflows/rust-client-windows.yml`, `.github/workflows/rust-client.yml` | Todas as referencias antigas sao trocadas para as versoes alvo | diff com somente bump de versao |

### F2. Validacao

| ID | Owner | Planned files | Done when | Required evidence |
|---|---|---|---|---|
| F2.S1.T1 | local | nenhum | Nenhuma referencia `actions/checkout@v4`, `actions/cache@v4`, `actions/upload-artifact@v4` sobra no repo | busca sem resultados |
| F2.S1.T2 | local | nenhum | YAML dos workflows continua valido e o diff fica limitado aos arquivos esperados | `git diff --check` e/ou parser YAML |

## Validation Gates

Validation Gate F1: `rtk git grep -n -E 'actions/(checkout|cache|upload-artifact)@v4' -- .github/workflows` returns no matches; `rtk git diff --check` is clean; workflow YAML files parse successfully.

Validation Gate F2: no `e2e-validator` handoff is required because this feature only bumps workflow action majors and does not change runtime, UI, or API behavior.

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1 | Validation Gate F1 test evidence: `actions/checkout@v6`, `actions/cache@v5`, and `actions/upload-artifact@v6` replace the older majors in the targeted workflows. |
| AC-02 | F1.S1.T1 | Validation Gate F1 test evidence: the diff only bumps the action majors in the workflow files listed in F1.S1.T1. |
| AC-03 | F2.S1.T1 | Validation Gate F1/F2 evidence: `rtk git grep -n -E 'actions/(checkout|cache|upload-artifact)@v4' -- .github/workflows` returns no matches. |
| AC-04 | F2.S1.T2 | Validation Gate F2 test evidence: `rtk git diff --check` is clean and the workflow YAML files parse successfully. |

## Parallelizacao

- Todas as atualizacoes de versao podem ser feitas em um unico pass nas workflows listadas em F1.S1.T1.
- A validacao F2 pode correr depois do patch sem bloquear revisao de logica, porque nao ha mudanca funcional.

## Risks

- Rollback por commit: se uma action major nova falhar em CI, reverter o commit do bump de versao.
- Integracao: o risco principal e sintatico/compatibilidade de YAML; nao ha impacto de runtime.
