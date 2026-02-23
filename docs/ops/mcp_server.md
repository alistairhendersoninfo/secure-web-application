# MCP Server — Linear Integration Wrapper

This repository ships a simple MCP server exposing Linear tools so Claude (or other MCP clients) can manage the Linear project directly.

Tools
- bootstrap_roadmap(project_name?) — Create/Use a Linear Project, create Phase issues (from docs/ROADMAP.md), and add child deliverable tasks
- sync_doc(file, phase?) — Create/update a Linear issue from a local markdown doc so humans can edit in Linear

Requirements
- Python 3.10+
- pip install mcp requests pyyaml
- Env vars:
  - LINEAR_API_KEY
  - LINEAR_TEAM_ID
  - LINEAR_PROJECT_NAME (optional; defaults to "SWAP Roadmap")

Run (MCP)
```bash
python3 tools/linear/mcp_server.py
```

Claude Desktop config (example)
```json
{
  "mcpServers": {
    "linear": {
      "command": "python3",
      "args": ["tools/linear/mcp_server.py"],
      "env": {
        "LINEAR_API_KEY": "<your-key>",
        "LINEAR_TEAM_ID": "<team-id>",
        "LINEAR_PROJECT_NAME": "SWAP Roadmap"
      }
    }
  }
}
```

CLI Fallback
If `mcp` is not installed, the script can run in CLI mode:
```bash
# Bootstrap project + phases + deliverables
LINEAR_API_KEY=… LINEAR_TEAM_ID=… python3 tools/linear/mcp_server.py bootstrap "SWAP Roadmap"

# Sync a doc to Linear
LINEAR_API_KEY=… LINEAR_TEAM_ID=… python3 tools/linear/mcp_server.py sync_doc docs/security/auth_store_spec.md "Phase 1 — Core Security Foundations"
```

Notes
- The server reads specs from `docs/` and writes human-readable copies to Linear.
- Re-run sync_doc after doc updates to keep Linear in sync.
