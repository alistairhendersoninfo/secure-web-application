#!/usr/bin/env python3
import os
import re
from pathlib import Path
from linear_api import Linear

PROJECT_NAME = os.getenv("LINEAR_PROJECT_NAME", "SWAP Roadmap")
TEAM_ID = os.getenv("LINEAR_TEAM_ID")
API_KEY = os.getenv("LINEAR_API_KEY")
DRY = os.getenv("DRY_RUN") == "1"

if not API_KEY or not TEAM_ID:
    raise SystemExit("Set LINEAR_API_KEY and LINEAR_TEAM_ID env vars")

def parse_roadmap(path: Path):
    phases = {}
    current_phase = None
    in_deliverables = False
    for line in path.read_text().splitlines():
        if line.startswith("## Phase "):
            current_phase = line.replace("## ", "").strip()
            phases[current_phase] = {"deliverables": []}
            in_deliverables = False
        elif line.strip().startswith("4.") or line.strip().startswith("5."):
            # ignore numbering; we look for '- Deliverables:'
            pass
        elif line.strip().startswith("- Deliverables"):
            in_deliverables = True
        elif line.strip().startswith("- Acceptance"):
            in_deliverables = False
        elif in_deliverables and line.strip().startswith("- ") and current_phase:
            phases[current_phase]["deliverables"].append(line.strip()[2:])
    return phases

def main():
    lr = Linear(API_KEY)
    roadmap = parse_roadmap(Path("docs/ROADMAP.md"))
    proj = lr.find_project_by_name(PROJECT_NAME)
    if proj:
        print(f"Using existing project: {proj['name']} ({proj['id']})")
        project_id = proj["id"]
    else:
        if DRY:
            print(f"[DRY] Would create project: {PROJECT_NAME}")
            project_id = None
        else:
            p = lr.create_project(PROJECT_NAME)
            project_id = p["id"]
            print(f"Created project: {PROJECT_NAME} ({project_id})")

    # Ensure phase labels exist
    phase_labels = {}
    for phase in roadmap.keys():
        phase_key = phase.split(" — ")[0].strip()  # e.g., Phase 1 — ...
        label = lr.find_label(TEAM_ID, phase_key)
        if label:
            phase_labels[phase_key] = label["id"]
        elif not DRY:
            lbl = lr.create_label(TEAM_ID, phase_key)
            phase_labels[phase_key] = lbl["id"]
            print(f"Created label {phase_key}")
        else:
            print(f"[DRY] Would create label {phase_key}")

    for phase, data in roadmap.items():
        phase_key = phase.split(" — ")[0].strip()
        label_ids = [phase_labels.get(phase_key)] if phase_key in phase_labels else None
        # Create tracker issue per phase
        title = phase
        desc = f"Tracking issue for {phase} — see docs/ROADMAP.md\n\nAcceptance criteria in Roadmap."
        if DRY:
            print(f"[DRY] Would create phase issue: {title}")
            parent_id = None
        else:
            issue = lr.create_issue(TEAM_ID, title, desc, project_id=project_id, label_ids=label_ids)
            parent_id = issue["id"]
            print(f"Created phase issue: {title} ({issue['identifier']})")
        # Create deliverable child issues
        for d in data["deliverables"]:
            child_title = d
            child_desc = f"Deliverable from {phase}: {d}. See related SPEC/PLAN in docs/."
            if DRY:
                print(f"[DRY] Would create child task: {child_title}")
            else:
                child = lr.create_issue(TEAM_ID, child_title, child_desc, project_id=project_id, parent_id=parent_id, label_ids=label_ids)
                print(f"  - Created: {child['identifier']} {child_title}")

if __name__ == "__main__":
    main()

