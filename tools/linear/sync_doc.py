#!/usr/bin/env python3
import os
import re
import sys
from pathlib import Path
from linear_api import Linear

API_KEY = os.getenv("LINEAR_API_KEY")
TEAM_ID = os.getenv("LINEAR_TEAM_ID")
PROJECT_NAME = os.getenv("LINEAR_PROJECT_NAME", "SWAP Roadmap")

def usage():
    print("Usage: sync_doc.py --file <path> [--project <name>] [--phase <Phase heading>]")

def extract_title_and_body(md: str):
    lines = md.strip().splitlines()
    title = lines[0].lstrip('# ').strip() if lines else 'Document'
    return title, md

def main(argv):
    if not API_KEY or not TEAM_ID:
        print("Set LINEAR_API_KEY and LINEAR_TEAM_ID", file=sys.stderr)
        return 2
    path = None
    project = PROJECT_NAME
    phase = None
    i = 0
    while i < len(argv):
        if argv[i] == '--file':
            path = argv[i+1]; i += 2
        elif argv[i] == '--project':
            project = argv[i+1]; i += 2
        elif argv[i] == '--phase':
            phase = argv[i+1]; i += 2
        else:
            i += 1
    if not path:
        usage(); return 2

    md = Path(path).read_text(encoding='utf-8')
    title, body = extract_title_and_body(md)
    lr = Linear(API_KEY)
    proj = lr.find_project_by_name(project)
    project_id = proj['id'] if proj else None
    label_ids = None
    if phase:
        phase_key = phase.split(' — ')[0].strip()
        lbl = lr.find_label(TEAM_ID, phase_key)
        if lbl:
            label_ids = [lbl['id']]
    issue = lr.create_issue(TEAM_ID, title, body, project_id=project_id, label_ids=label_ids)
    print(f"Created Linear issue {issue['identifier']} for {path}")
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

