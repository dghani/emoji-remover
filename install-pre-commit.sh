#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/target/release/emoji-remover"

# Check if emoji-remover is in PATH
if command -v emoji-remover &> /dev/null; then
    EMOJI_REMOVER_CMD="emoji-remover"
    echo "Found emoji-remover in PATH"
elif [ -f "$BINARY_PATH" ]; then
    EMOJI_REMOVER_CMD="$BINARY_PATH"
    echo "Using local build"
else
    echo "Building emoji-remover..."
    cargo build --release
    EMOJI_REMOVER_CMD="$BINARY_PATH"
fi

if [ ! -d .git ]; then
    echo "Error: This script must be run from a git repository root"
    exit 1
fi

mkdir -p .git/hooks

cat > .git/hooks/pre-commit << EOF
#!/bin/bash

echo "Running emoji remover..."

# Get list of staged files
FILES=\$(git diff --cached --name-only --diff-filter=ACM)

if [ -n "\$FILES" ]; then
    # Run emoji-remover on staged files only
    echo "\$FILES" | xargs $EMOJI_REMOVER_CMD --quiet
    
    # Re-add the modified files
    echo "\$FILES" | xargs git add
fi

exit 0
EOF

chmod +x .git/hooks/pre-commit

echo "Pre-commit hook installed successfully!"
echo "The hook will automatically remove emojis from staged files before each commit."
echo ""
echo "To uninstall, run: rm .git/hooks/pre-commit"