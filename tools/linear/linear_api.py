import os
import requests

LINEAR_API = "https://api.linear.app/graphql"

class Linear:
    def __init__(self, api_key: str):
        self.api_key = api_key

    def _post(self, query: str, variables: dict):
        r = requests.post(
            LINEAR_API,
            headers={"Authorization": f"Bearer {self.api_key}", "Content-Type": "application/json"},
            json={"query": query, "variables": variables},
            timeout=30,
        )
        r.raise_for_status()
        data = r.json()
        if "errors" in data:
            raise RuntimeError(f"Linear error: {data['errors']}")
        return data["data"]

    def find_project_by_name(self, name: str):
        q = """
        query($query: String!) {
          projects(filter: { name: { eq: $query } }, first: 1) { nodes { id name } }
        }
        """
        d = self._post(q, {"query": name})
        nodes = d["projects"]["nodes"]
        return nodes[0] if nodes else None

    def create_project(self, name: str, team_id: str = None):
        # teamId is optional; project can be cross-team
        q = """
        mutation($input: ProjectCreateInput!) {
          projectCreate(input: $input) { success project { id name } }
        }
        """
        inp = {"name": name}
        if team_id:
            inp["teamId"] = team_id
        d = self._post(q, {"input": inp})
        return d["projectCreate"]["project"]

    def create_issue(self, team_id: str, title: str, description: str, project_id: str = None, parent_id: str = None, label_ids=None):
        q = """
        mutation($input: IssueCreateInput!) {
          issueCreate(input: $input) { success issue { id identifier title } }
        }
        """
        inp = {"teamId": team_id, "title": title, "description": description}
        if project_id:
            inp["projectId"] = project_id
        if parent_id:
            inp["parentId"] = parent_id
        if label_ids:
            inp["labelIds"] = label_ids
        d = self._post(q, {"input": inp})
        return d["issueCreate"]["issue"]

    def find_label(self, team_id: str, name: str):
        q = """
        query($team: String!, $name: String!) {
          teams(filter: { id: { eq: $team } }, first: 1) { nodes { labels(filter: { name: { eq: $name } }, first: 1) { nodes { id name } } } }
        }
        """
        d = self._post(q, {"team": team_id, "name": name})
        nodes = d["teams"]["nodes"]
        if not nodes:
            return None
        labels = nodes[0]["labels"]["nodes"]
        return labels[0] if labels else None

    def create_label(self, team_id: str, name: str, color: str = "#55CC99"):
        q = """
        mutation($input: IssueLabelCreateInput!) { issueLabelCreate(input: $input) { success issueLabel { id name color } } }
        """
        d = self._post(q, {"input": {"teamId": team_id, "name": name, "color": color}})
        return d["issueLabelCreate"]["issueLabel"]
