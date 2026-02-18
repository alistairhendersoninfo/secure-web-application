---
name: New Plugin Proposal
about: Propose a new plugin integration (e.g., nftables, Coraza, AppArmor)
title: '[PLUGIN] '
labels: plugin
assignees: ''
---

Plugin name
What tool does this integrate with?

Category
firewall | waf | httpd | lsm | av | ids | logs | other

What it does
High-level description of the capability.

Inputs/Config
Expected input schema (versioned). Include examples.

Outputs
Rendered config artifacts, plan/apply steps, health checks, diagnostics.

Sandbox requirements
WASM capabilities needed (filesystem paths, no network by default, CPU/mem bounds).

Security considerations
Attack surface and mitigation (validation, least privilege, safe defaults).

Testing plan
Unit/integration tests, fixtures, target environments.

