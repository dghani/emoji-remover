#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/target/release/emoji-remover"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Building emoji-remover..."
    cargo build --release
fi

if [ ! -d .git ]; then
    echo "Error: This script must be run from a git repository root"
    exit 1
fi

mkdir -p .git/hooks

cat > .git/hooks/pre-commit << EOF
#!/bin/bash

echo "Running emoji remover..."

$BINARY_PATH . --dry-run

if [ \$? -eq 0 ]; then
    $BINARY_PATH .
    
    git add -u
    
    exit 0
else
    echo "Pre-commit hook failed"
    exit 1
fi
EOF

chmod +x .git/hooks/pre-commit

echo "Pre-commit hook installed successfully!"
echo "The hook will automatically remove emojis from all supported files before each commit."
echo ""
echo "To uninstall, run: rm .git/hooks/pre-commit"