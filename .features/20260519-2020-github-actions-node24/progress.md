# Migracao dos GitHub Actions para Node 24
Status atual: `DOC.SPEC`, `DOC.PLAN`, `F1.S1.T1`, `F2.S1.T1` e `F2.S1.T2` concluidos.

Proximo passo concreto: nenhum.

## Controle documental

| ID | Status | Owner/subagent | Planned files | Actual files touched | Required evidence | Produced evidence | Blocker/cause |
|---|---|---|---|---|---|---|---|
| DOC.SPEC | done | local | `.features/20260519-2020-github-actions-node24/spec.md` | `.features/20260519-2020-github-actions-node24/spec.md` | spec em `READY_FOR_PLAN` | spec criado | nenhum |
| DOC.PLAN | done | local | `.features/20260519-2020-github-actions-node24/plan.md` | `.features/20260519-2020-github-actions-node24/plan.md` | plano em `READY_FOR_EXEC` | plano criado | nenhum |
| DOC.PROGRESS | done | local | `.features/20260519-2020-github-actions-node24/progress.md` | `.features/20260519-2020-github-actions-node24/progress.md` | progress sincronizado com a execucao real | este arquivo | nenhum |

## Phases / Tasks

### F1. Atualizacao dos workflows

| ID | Status | Owner/subagent | Planned files | Actual files touched | Required evidence | Produced evidence | Blocker/cause |
|---|---|---|---|---|---|---|---|
| F1.S1.T1 | done | local | `.github/workflows/claude.yml`, `.github/workflows/mingw-build-dev.yml`, `.github/workflows/mingw-build-pr.yml`, `.github/workflows/mingw-build.yml`, `.github/workflows/rust-client-windows.yml`, `.github/workflows/rust-client.yml` | `.github/workflows/claude.yml`, `.github/workflows/mingw-build-dev.yml`, `.github/workflows/mingw-build-pr.yml`, `.github/workflows/mingw-build.yml`, `.github/workflows/rust-client-windows.yml`, `.github/workflows/rust-client.yml` | referencias Node24-compatible atualizadas | `actions/checkout@v6`, `actions/cache@v5`, `actions/upload-artifact@v6` | nenhum |

### F2. Validacao

| ID | Status | Owner/subagent | Planned files | Actual files touched | Required evidence | Produced evidence | Blocker/cause |
|---|---|---|---|---|---|---|---|
| F2.S1.T1 | done | local | nenhum | nenhum | ausencia de referencias `@v4` para checkout/cache/upload-artifact | `rtk git grep -n -E 'actions/(checkout|cache|upload-artifact)@v4' -- .github/workflows` sem saida | nenhum |
| F2.S1.T2 | done | local | nenhum | nenhum | `git diff --check` limpo e workflows validos | `rtk git diff --check`; `rtk python3 -c 'import pathlib, yaml; [yaml.safe_load(pathlib.Path(f).read_text()) for f in pathlib.Path(".github/workflows").glob("*.yml")]'`; `rtk ruby -e 'require "yaml"; Dir[".github/workflows/*.yml"].each { |f| YAML.load_file(f) }'` | nenhum |

## AC Traceability

| AC | Tasks | Validation evidence |
|---|---|---|
| AC-01 | F1.S1.T1 | checkout/cache/upload-artifact atualizados para Node24-compatible majors |
| AC-02 | F1.S1.T1 | diff limitado a bump de versao nas workflows |
| AC-03 | F2.S1.T1 | busca sem `@v4` remanescente |
| AC-04 | F2.S1.T2 | `git diff --check` limpo e YAML valido |
