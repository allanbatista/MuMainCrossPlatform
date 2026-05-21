# Character Delete Packet Wiring

Status: READY_FOR_PLAN

## Goal

Teach the Rust auth shell to surface the character-delete route and submit the
legacy delete-character packet for the selected roster entry, while keeping the
login/character-select flow safe when the delete fails.

## Users And Journeys

- QA/developer opens the character-delete surface from the character-select
  flow, confirms the target name, and submits the local security code.
- Player/developer sees the same character-delete route in the graphical
  runtime instead of a dead route constant.
- QA/developer can drive the flow locally with control HTTP without changing
  the headless boot behavior.

## Product Inventory

| Surface | Route/Page | Slug/ID | User-visible label | Output type | Filters | Datasets/permissions | Empty/loading/error/locked behavior | Persona differences |
|---|---|---|---|---|---|---|---|---|
| Character delete | `UiRoute::CharacterDelete` | `character-delete` | Character Delete | auth shell + control command | selected character, security code | local session/bootstrap only | invalid state stays on a safe auth surface; delete failure remains visible | players/developers see the delete confirmation shell; QA can drive it through local smoke |
| Delete submit | `POST /command?name=delete-character` | `delete-character` | Delete Character | control HTTP command | character name, security code | local control-http only | missing input or send failure stays on the delete surface with an error notice | QA/dev can submit the legacy delete packet locally |

## Requirements

- The character-select delete action must have a visible auth-shell route.
- The selected character name must carry into the delete flow.
- The bootstrap worker must send the legacy delete-character packet with the
  selected name and supplied security code.
- The delete response must keep the route safe on failure and return to
  character select on success.
- Headless and control-http smoke behavior must remain deterministic.
- Every code change must have automated tests.
- Usage docs must mention the new delete flow and the control-plane command.

## Acceptance Criteria

- AC-01. The graphical auth shell can render a visible character-delete route.
- AC-02. The local delete submit path sends the expected delete-character
  packet for the selected roster entry.
- AC-03. Delete success returns the client to character select or a refreshed
  roster surface.
- AC-04. Delete failure keeps the client on a safe auth surface instead of
  exiting.
- AC-05. Automated tests cover the delete route shell, packet send path, and
  response classification.
- AC-06. Player-facing docs describe the delete flow and its local smoke path.

## Scope

In scope:

- Character-delete auth shell route.
- Local control-http command for delete submit.
- Bootstrap packet send and response handling.
- Usage docs and KB tracking.

Out of scope:

- New protocol shapes.
- Persisted account/security-code storage.
- New login or world network flows.

## Open Questions

None blocking. Assumption: the delete flow uses the currently selected
character name plus a local security code string provided through control HTTP.
