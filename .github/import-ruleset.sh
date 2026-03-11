#!/bin/bash

# Import GitHub Ruleset
# Requires: GitHub CLI (gh) or Personal Access Token

REPO_OWNER="mdatla"
REPO_NAME="idrac-fan-controller-rust"

echo "🔒 Importing branch protection ruleset for ${REPO_OWNER}/${REPO_NAME}"
echo ""

# Check if GitHub CLI is installed
if command -v gh &> /dev/null; then
    echo "✅ Using GitHub CLI (gh)"
    echo ""
    
    # Create ruleset using GitHub CLI
    gh api \
        --method POST \
        -H "Accept: application/vnd.github+json" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        "/repos/${REPO_OWNER}/${REPO_NAME}/rulesets" \
        --input .github/branch-protection-ruleset.json
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ Ruleset imported successfully!"
        echo ""
        echo "View at: https://github.com/${REPO_OWNER}/${REPO_NAME}/settings/rules"
    else
        echo ""
        echo "❌ Failed to import ruleset"
        echo ""
        echo "Try manually:"
        echo "1. Go to https://github.com/${REPO_OWNER}/${REPO_NAME}/settings/rules"
        echo "2. Click 'New ruleset' → 'Import a ruleset'"
        echo "3. Upload: .github/branch-protection-ruleset.json"
    fi
else
    echo "❌ GitHub CLI (gh) not found"
    echo ""
    echo "Install GitHub CLI:"
    echo "  macOS:   brew install gh"
    echo "  Linux:   https://github.com/cli/cli/blob/trunk/docs/install_linux.md"
    echo "  Windows: https://github.com/cli/cli/releases"
    echo ""
    echo "Or use curl with Personal Access Token:"
    echo ""
    echo "export GITHUB_TOKEN='your_token_here'"
    echo "curl -L \\"
    echo "  -X POST \\"
    echo "  -H 'Accept: application/vnd.github+json' \\"
    echo "  -H 'Authorization: Bearer \$GITHUB_TOKEN' \\"
    echo "  -H 'X-GitHub-Api-Version: 2022-11-28' \\"
    echo "  https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/rulesets \\"
    echo "  -d @.github/branch-protection-ruleset.json"
    echo ""
    echo "Or manually import via GitHub UI:"
    echo "1. Go to https://github.com/${REPO_OWNER}/${REPO_NAME}/settings/rules"
    echo "2. Click 'New ruleset' → 'Import a ruleset'"
    echo "3. Upload: .github/branch-protection-ruleset.json"
fi
