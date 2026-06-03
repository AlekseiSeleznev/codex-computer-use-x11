#!/usr/bin/env bash
set -Eeuo pipefail

PLUGIN_NAME="codex-computer-use-x11"
DEFAULT_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
OUTPUT_DIR="dist/release"
TARGET_TRIPLE="$DEFAULT_TARGET_TRIPLE"
SKIP_BUILD=0
CHECK=0

usage() {
    cat <<HELP
Usage: scripts/package-release.sh [--output-dir DIR] [--target-triple TRIPLE] [--skip-build] [--check]

Build a self-contained codex-computer-use-x11 Codex plugin release tarball and
matching SHA256 sidecar. By default this runs cargo build --release.
HELP
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --output-dir)
            OUTPUT_DIR="${2:?--output-dir requires a value}"
            shift
            ;;
        --target-triple)
            TARGET_TRIPLE="${2:?--target-triple requires a value}"
            shift
            ;;
        --skip-build)
            SKIP_BUILD=1
            ;;
        --check)
            CHECK=1
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "unknown argument: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
    shift
done

SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION_FILE="$REPO_DIR/VERSION"
VERSION="$(tr -d '[:space:]' < "$VERSION_FILE")"
CARGO_VERSION="$(python3 - "$REPO_DIR/Cargo.toml" <<'PY'
import pathlib, re, sys
text = pathlib.Path(sys.argv[1]).read_text(encoding='utf-8')
match = re.search(r'(?m)^version\s*=\s*"([^"]+)"', text)
if not match:
    raise SystemExit('could not read package version from Cargo.toml')
print(match.group(1))
PY
)"
if [ "$VERSION" != "$CARGO_VERSION" ]; then
    echo "VERSION ($VERSION) does not match Cargo.toml version ($CARGO_VERSION)" >&2
    exit 1
fi

if [ "$SKIP_BUILD" -eq 1 ]; then
    BINARY_SOURCE="${CODEX_X11_PACKAGE_BINARY:-$REPO_DIR/target/release/codex-computer-use-x11}"
else
    (cd "$REPO_DIR" && cargo build --release)
    BINARY_SOURCE="$REPO_DIR/target/release/codex-computer-use-x11"
fi
if [ ! -x "$BINARY_SOURCE" ]; then
    echo "release binary is not executable: $BINARY_SOURCE" >&2
    exit 1
fi

mkdir -p "$OUTPUT_DIR"
OUTPUT_DIR="$(cd "$OUTPUT_DIR" && pwd)"
ARTIFACT="${PLUGIN_NAME}-v${VERSION}-${TARGET_TRIPLE}.tar.gz"
ARTIFACT_PATH="$OUTPUT_DIR/$ARTIFACT"
SHA_PATH="$ARTIFACT_PATH.sha256"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/${PLUGIN_NAME}-package.XXXXXX")"
cleanup() {
    rm -rf "$WORK_DIR"
}
trap cleanup EXIT

BUNDLE_DIR="$WORK_DIR/$PLUGIN_NAME"
python3 "$REPO_DIR/scripts/lib/plugin-bundle.py" \
    --repo-dir "$REPO_DIR" \
    --dest "$BUNDLE_DIR" \
    --binary "$BINARY_SOURCE" \
    --version "$VERSION" \
    --release-metadata \
    --artifact "$ARTIFACT"

rm -f "$ARTIFACT_PATH" "$SHA_PATH"
(
    cd "$WORK_DIR"
    tar --sort=name --owner=0 --group=0 --numeric-owner --mtime='UTC 1970-01-01' -czf "$ARTIFACT_PATH" "$PLUGIN_NAME"
)
(
    cd "$OUTPUT_DIR"
    sha256sum "$ARTIFACT" > "$ARTIFACT.sha256"
)

forbidden_listing_check() {
    local artifact_path="$1"
    local listing
    listing="$(tar -tzf "$artifact_path")"
    if printf '%s\n' "$listing" | grep -E '(^|/)\.git/|(^|/)target/|(^|/)\.codex/session/|(^|/)\.secrets|\.local\.env$|(^|/)\.env$|\.bak(\.|$)' >/dev/null; then
        echo "release tarball contains forbidden files" >&2
        printf '%s\n' "$listing" >&2
        return 1
    fi
}

check_artifact() {
    (
        cd "$OUTPUT_DIR"
        sha256sum --check "$ARTIFACT.sha256"
    )
    forbidden_listing_check "$ARTIFACT_PATH"
    local extract_dir="$WORK_DIR/check"
    mkdir -p "$extract_dir"
    tar -xzf "$ARTIFACT_PATH" -C "$extract_dir"
    python3 - "$extract_dir/$PLUGIN_NAME" "$VERSION" <<'PY'
import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
version = sys.argv[2]
mcp = json.loads((root / '.mcp.json').read_text(encoding='utf-8'))
server = mcp['mcpServers']['codex-computer-use-x11']
assert server['command'] == './bin/codex-computer-use-x11'
assert server['args'] == ['mcp']
assert server['cwd'] == '.'
plugin = json.loads((root / '.codex-plugin/plugin.json').read_text(encoding='utf-8'))
assert plugin['name'] == 'codex-computer-use-x11'
assert plugin['version'] == version
assert plugin['interface']['displayName'] == 'X11 Computer Use'
metadata = json.loads((root / 'RELEASE-METADATA.json').read_text(encoding='utf-8'))
assert metadata['version'] == version
assert metadata['baseline'] == 'x11-ewmh / Cinnamon X11'
PY
    local doctor_json="$WORK_DIR/doctor.json"
    "$extract_dir/$PLUGIN_NAME/bin/codex-computer-use-x11" doctor --json > "$doctor_json"
    python3 - "$doctor_json" "$VERSION" <<'PY'
import json, pathlib, sys
report = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
assert report['project'] == 'codex-computer-use-x11'
assert report['version'] == sys.argv[2]
assert report['backend'] == 'x11-ewmh'
assert isinstance(report.get('readiness'), dict)
PY
}

if [ "$CHECK" -eq 1 ]; then
    check_artifact
fi

printf '%s\n' "$ARTIFACT_PATH"
printf '%s\n' "$SHA_PATH"
