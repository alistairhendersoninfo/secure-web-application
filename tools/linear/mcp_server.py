#!/usr/bin/env python3
"""
MCP Server Wrapper for Linear

Exposes two tools over MCP so MCP-capable clients (e.g., Claude Desktop) can:
- bootstrap_roadmap(project_name?): create a Linear project, phases, and deliverables from docs/ROADMAP.md
- sync_doc(file, phase?): create/update a Linear issue from a repo markdown doc

Env vars required:
- LINEAR_API_KEY
- LINEAR_TEAM_ID
- LINEAR_PROJECT_NAME (optional, defaults to "SWAP Roadmap")

Install dependencies in your MCP runtime:
- pip install mcp requests pyyaml

Claude Desktop config (example):
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
"""

import os
import sys
from pathlib import Path

# Local modules
from linear_api import Linear  # type: ignore

LINEAR_API_KEY = os.getenv("LINEAR_API_KEY")
LINEAR_TEAM_ID = os.getenv("LINEAR_TEAM_ID")
LINEAR_PROJECT_NAME = os.getenv("LINEAR_PROJECT_NAME", "SWAP Roadmap")

def require_env():
    missing = []
    if not LINEAR_API_KEY:
        missing.append("LINEAR_API_KEY")
    if not LINEAR_TEAM_ID:
        missing.append("LINEAR_TEAM_ID")
    if missing:
        raise RuntimeError(f"Missing env: {', '.join(missing)}")

def parse_roadmap(path: Path):
    phases = {}
    current_phase = None
    in_deliverables = False
    for line in path.read_text(encoding='utf-8').splitlines():
        if line.startswith("## Phase "):
            current_phase = line.replace("## ", "").strip()
            phases[current_phase] = {"deliverables": []}
            in_deliverables = False
        elif line.strip().startswith("- Deliverables"):
            in_deliverables = True
        elif line.strip().startswith("- Acceptance"):
            in_deliverables = False
        elif in_deliverables and line.strip().startswith("- ") and current_phase:
            phases[current_phase]["deliverables"].append(line.strip()[2:])
    return phases

def do_bootstrap(project_name: str | None = None) -> str:
    require_env()
    lr = Linear(LINEAR_API_KEY)  # type: ignore
    project_name = project_name or LINEAR_PROJECT_NAME
    proj = lr.find_project_by_name(project_name)
    if proj:
        project_id = proj["id"]
        info = f"Using existing project: {proj['name']} ({project_id})"
    else:
        p = lr.create_project(project_name)
        project_id = p["id"]
        info = f"Created project: {project_name} ({project_id})"

    roadmap = parse_roadmap(Path("docs/ROADMAP.md"))

    # Ensure phase labels
    phase_labels = {}
    for phase in roadmap.keys():
        phase_key = phase.split(" — ")[0].strip()
        lbl = lr.find_label(LINEAR_TEAM_ID, phase_key)
        if not lbl:
            lbl = lr.create_label(LINEAR_TEAM_ID, phase_key)
        phase_labels[phase_key] = lbl["id"]

    # Create phase issues and deliverables
    created = [info]
    for phase, data in roadmap.items():
        phase_key = phase.split(" — ")[0].strip()
        label_ids = [phase_labels.get(phase_key)] if phase_key in phase_labels else None
        phase_issue = lr.create_issue(LINEAR_TEAM_ID, phase, f"Tracking issue for {phase} — see docs/ROADMAP.md", project_id=project_id, label_ids=label_ids)
        created.append(f"Phase: {phase_issue['identifier']} {phase}")
        for d in data["deliverables"]:
            child = lr.create_issue(LINEAR_TEAM_ID, d, f"Deliverable from {phase}: {d}", project_id=project_id, parent_id=phase_issue["id"], label_ids=label_ids)
            created.append(f"  - {child['identifier']} {d}")
    return "\n".join(created)

def do_sync_doc(file: str, phase: str | None = None) -> str:
    require_env()
    lr = Linear(LINEAR_API_KEY)  # type: ignore
    proj = lr.find_project_by_name(LINEAR_PROJECT_NAME)
    project_id = proj['id'] if proj else None
    md = Path(file).read_text(encoding='utf-8')
    title = Path(file).stem.replace('_', ' ').title()
    label_ids = None
    if phase:
        phase_key = phase.split(' — ')[0].strip()
        lbl = lr.find_label(LINEAR_TEAM_ID, phase_key)
        if lbl:
            label_ids = [lbl['id']]
    issue = lr.create_issue(LINEAR_TEAM_ID, title, md, project_id=project_id, label_ids=label_ids)
    return f"Created Linear issue {issue['identifier']} for {file}"

def main():
    # Try to load MCP; if unavailable, provide a CLI fallback.
    try:
        from mcp.server.stdio import stdio_server  # type: ignore
        from mcp.server import Server  # type: ignore
    except Exception as e:
        print("MCP not installed. Install with: pip install mcp", file=sys.stderr)
        print("CLI fallback usage:", file=sys.stderr)
        print("  python3 tools/linear/mcp_server.py bootstrap [project]")
        print("  python3 tools/linear/mcp_server.py sync_doc <file> [phase]", file=sys.stderr)
        if len(sys.argv) >= 2 and sys.argv[1] in ("bootstrap", "sync_doc"):
            cmd = sys.argv[1]
            if cmd == "bootstrap":
                project = sys.argv[2] if len(sys.argv) > 2 else None
                print(do_bootstrap(project))
                return 0
            else:
                if len(sys.argv) < 3:
                    print("Missing file", file=sys.stderr)
                    return 2
                file = sys.argv[2]
                phase = sys.argv[3] if len(sys.argv) > 3 else None
                print(do_sync_doc(file, phase))
                return 0
        return 2

    server = Server("linear-mcp")

    @server.tool()
    def bootstrap_roadmap(project_name: str | None = None) -> str:
        """Create Linear project, phases, and deliverables from docs/ROADMAP.md."""
        return do_bootstrap(project_name)

    @server.tool()
    def sync_doc(file: str, phase: str | None = None) -> str:
        """Create/update a Linear issue from a repo markdown doc."""
        return do_sync_doc(file, phase)

    stdio_server(server).run()
    return 0

if __name__ == "__main__":
    sys.exit(main())

