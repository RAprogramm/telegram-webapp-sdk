#!/usr/bin/env bash
set -euo pipefail

# Определяем последнюю версию из CHANGELOG.md (первая строка, начинающаяся с "## [")
TAG=$(grep -m1 -oP '^## \[\K[0-9]+\.[0-9]+\.[0-9]+' CHANGELOG.md)
TAG="v$TAG"

echo "📦 Preparing release for $TAG"

# Проверяем, существует ли уже релиз
if gh release view "$TAG" >/dev/null 2>&1; then
  echo "⚠️ Release $TAG already exists on GitHub. Nothing to do."
  exit 0
fi

# Вырезаем секцию для этого тега
notes=$(awk "/^## \\[$(echo "$TAG" | sed 's/^v//')\\]/ {flag=1; next} /^## \\[/ && flag {exit} flag" CHANGELOG.md)

if [ -z "$notes" ]; then
  echo "❌ Could not extract changelog section for $TAG"
  exit 1
fi

# Создаём релиз
gh release create "$TAG" \
  --title "$TAG" \
  --notes "$notes"

echo "✅ GitHub release $TAG created."

