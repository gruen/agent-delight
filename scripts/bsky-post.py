#!/usr/bin/env python3
"""
Post to BlueSky via the AT Protocol.

Usage:
    python3 scripts/bsky-post.py "Your post text here"
    python3 scripts/bsky-post.py --file SOCIAL.md --post 1

Requires env vars:
    TEMPO_BSKY_HANDLE       — e.g. tempo-devlog.bsky.social
    TEMPO_BSKY_APP_PASSWORD — app-specific password from BlueSky settings
"""

import json
import os
import re
import sys
import urllib.request
from datetime import datetime, timezone

API = "https://bsky.social/xrpc"


def api_call(endpoint, data, token=None):
    body = json.dumps(data).encode()
    headers = {"Content-Type": "application/json"}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    req = urllib.request.Request(f"{API}/{endpoint}", data=body, headers=headers)
    with urllib.request.urlopen(req) as resp:
        return json.loads(resp.read())


def login(handle, password):
    result = api_call("com.atproto.server.createSession", {
        "identifier": handle,
        "password": password,
    })
    return result["did"], result["accessJwt"]


def post(did, token, text):
    result = api_call("com.atproto.repo.createRecord", {
        "repo": did,
        "collection": "app.bsky.feed.post",
        "record": {
            "$type": "app.bsky.feed.post",
            "text": text,
            "createdAt": datetime.now(timezone.utc).isoformat(),
        },
    }, token=token)
    return result.get("uri", "")


def extract_post_from_social_md(path, post_number):
    """Extract a specific post from SOCIAL.md by number."""
    with open(path) as f:
        content = f.read()

    # Split on post headers (## Post N — ...)
    posts = re.split(r"^## Post \d+", content, flags=re.MULTILINE)
    if post_number < 1 or post_number >= len(posts):
        print(f"Post {post_number} not found. File has {len(posts) - 1} posts.", file=sys.stderr)
        sys.exit(1)

    block = posts[post_number]
    # Extract quoted lines (the actual post content)
    lines = []
    for line in block.split("\n"):
        line = line.strip()
        if line.startswith("> "):
            lines.append(line[2:])
        elif line == ">":
            lines.append("")
    return "\n".join(lines).strip()


def main():
    handle = os.environ.get("TEMPO_BSKY_HANDLE")
    password = os.environ.get("TEMPO_BSKY_APP_PASSWORD")

    if not handle or not password:
        print("Set TEMPO_BSKY_HANDLE and TEMPO_BSKY_APP_PASSWORD env vars.", file=sys.stderr)
        sys.exit(1)

    # Parse args
    if len(sys.argv) >= 4 and sys.argv[1] == "--file":
        text = extract_post_from_social_md(sys.argv[2], int(sys.argv[3]))
    elif len(sys.argv) >= 2:
        text = sys.argv[1]
    else:
        print(__doc__.strip(), file=sys.stderr)
        sys.exit(1)

    # Preview
    print(f"Posting as @{handle}:\n")
    print(text)
    print(f"\n({len(text)} chars)")

    if input("\nPost? [y/N] ").strip().lower() != "y":
        print("Cancelled.")
        return

    did, token = login(handle, password)
    uri = post(did, token, text)
    print(f"\nPosted: {uri}")


if __name__ == "__main__":
    main()
