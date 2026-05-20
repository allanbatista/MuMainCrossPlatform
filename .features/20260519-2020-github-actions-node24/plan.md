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

| Slug | Query | Components | Filtro | Saida | Observacao |
|---|---|---|---|---|---|
| `checkout-node24` | `actions/checkout` | checkout step | `@v6` | repo checkout | Node24-compatible release |
| `cache-node24` | `actions/cache` | cache step | `@v5` | cache restore/save | Node24-compatible release |
| `artifact-node24` | `actions/upload-artifact` | artifact upload | `@v6` | artifact upload | Node24-compatible release |

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

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1 | refs atualizadas para Node24-compatible releases |
| AC-02 | F1.S1.T1 | diff limitado a bump de versao nas workflows |
| AC-03 | F2.S1.T1 | busca sem `@v4` remanescente |
| AC-04 | F2.S1.T2 | `git diff --check` limpo e YAML valido |
