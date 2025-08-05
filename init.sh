#!/bin/bash
set -euo pipefail

# Echoes of Beastlight - Project Initializer
# This script helps set up the metaprompt game generator

echo "🎮 Echoes of Beastlight - Metaprompt Game Generator"
echo "=================================================="
echo ""

# Check if we're in a git repository
if [ ! -d .git ]; then
    echo "❌ Error: Not in a git repository!"
    echo "Please run this from the root of your cloned repository."
    exit 1
fi

# Create necessary directories
echo "📁 Creating directory structure..."
mkdir -p .github/workflows
mkdir -p src
mkdir -p data
mkdir -p assets/{sprites,audio,maps,prompts}
mkdir -p docs
mkdir -p build

echo "✅ Directories created"
echo ""

# Check for OpenAI API key in environment
if [ -z "$OPENAI_API_KEY" ]; then
    echo "⚠️  Warning: OPENAI_API_KEY not found in environment"
    echo "   You'll need to add it as a GitHub secret for workflows to work"
    echo "   Go to: Settings → Secrets → Actions → New repository secret"
    echo ""
fi

# Provide next steps
echo "🚀 Next Steps:"
echo "1. Add your OpenAI API key as a GitHub secret (if not done)"
echo "2. Review and customize game-config.yaml"
echo "3. Push changes to trigger the workflows"
echo "4. Watch the magic happen in the Actions tab!"
echo ""
echo "Available workflows:"
echo "- generate-file.yml: Base workflow for file generation"
echo "- bootstrap-beastlight.yml: Full game bootstrapping"
echo "- metaprompt-executor.yml: Direct metaprompt execution"
echo ""
echo "To trigger manually:"
echo "  gh workflow run bootstrap-beastlight.yml"
echo "  gh workflow run metaprompt-executor.yml"
echo ""
echo "Happy game generating! 🎮✨"