# Provider Auth Contract S5 Closure

Date: 2026-06-13
Status: accepted

## Scope

- `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md` `S5-A` and `S5-B`

## Senior decision

- Do not add a second internal provider-auth trait family.
- Reuse the existing private `ModelProvider` auth seam as the provider-auth contract:
  - `auth_manager`
  - `auth`
  - `api_auth`
  - `account_state`
  - runtime engine selection

## Why

- Adding another trait family here would duplicate the current provider owner.
- The repo rule requires new refactors to add real functionality and extend the existing core solution instead of creating a parallel stack.
- Current `model-provider` tests already exercise heterogeneous provider runtime/auth behavior for OpenAI/Codex, Claude/Anthropic, Gemini, Copilot, and Bedrock.

## Verification basis

- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-model-provider`
- `git diff --check`
- `ontoindex analyze`

## Outcome

- The provider credential routing refactor plan is complete.
