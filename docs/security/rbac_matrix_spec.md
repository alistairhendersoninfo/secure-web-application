# SPEC: RBAC Matrix (Framework + Modules)

## Goals
- Define a simple, consistent RBAC model applied globally and per module.

## Roles
- Admin — full control including user and plugin management.
- Editor — edit configs and apply changes; cannot manage users or install plugins.
- Viewer — read-only access to configs, logs, and charts.
- No Access — cannot view or modify.

## Resources & Actions
- Framework (global)
  - Users: list, create, update roles, disable
  - Plugins: install, update, remove, enable/disable
  - Agents: register, decommission
  - Policies: edit system-wide settings
  - Logs/Events: view, export
- Module (per plugin category and instance)
  - Config: view, edit, diff, apply, rollback
  - Logs: view, filter, export
  - Hints: view, accept/reject

## Matrix (Summary)
- Admin: allow all actions on Framework and Modules
- Editor: allow Module Config (edit/diff/apply/rollback), Logs (view/export), Hints (view/accept/reject); deny Framework Users/Plugins/Policies
- Viewer: allow Logs (view/export), Config (view/diff), Hints (view); deny all writes
- No Access: deny all

## Enforcement
- Authorization checks at HTTP/gRPC entry points; context carries role and module scope.
- Postgres RLS complements RBAC for data isolation.
- UI gates actions based on role; hidden controls not relied upon for security.

## Acceptance Criteria
- RBAC roles and actions documented; enforcement points identified; per-module scoping defined.
