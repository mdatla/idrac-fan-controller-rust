# Branch Protection Ruleset

The `main` branch is protected using GitHub Rulesets.

## View Current Protection

Active ruleset: https://github.com/mdatla/idrac-fan-controller-rust/settings/rules

## What This Ruleset Does

### 🎯 Target
- Protects the `main` branch only

### 🛡️ Protection Rules

#### 1. Require Pull Request
- **Required approvals:** 0 (solo developer)
- **Dismiss stale reviews:** Yes (when new commits pushed)
- **Require code owner review:** No
- **Require last push approval:** No
- **Require conversation resolution:** Yes (all comments must be resolved)

#### 2. Require Status Checks
- **Must pass before merge:** `build-and-push` (GitHub Actions workflow)
- **Require up to date:** Yes (must be synced with main)

#### 3. Block Force Pushes
- Prevents `git push --force`
- Protects against rewriting history

#### 4. Require Linear History
- No merge commits allowed
- Enforces rebase or squash merges
- Keeps git history clean

#### 5. Restrict Deletions
- Cannot delete the `main` branch

### 👤 Bypass Permissions
- **None** - Even repository admins must follow these rules
- To bypass: Temporarily disable the ruleset in emergencies

## What This Means for Your Workflow

### ❌ No Longer Allowed
```bash
# Direct push to main
git push origin main

# Force push
git push --force origin main

# Delete main branch
git push origin --delete main
```

### ✅ New Workflow
```bash
# 1. Create feature branch
git checkout -b feature/new-thing

# 2. Make changes and commit
git add .
git commit -m "Add new thing"

# 3. Push feature branch
git push origin feature/new-thing

# 4. Create PR on GitHub
# Go to: https://github.com/mdatla/idrac-fan-controller-rust/pulls
# Click "New pull request"

# 5. Wait for CI/CD to pass (build-and-push workflow)

# 6. Merge PR on GitHub
# Click "Squash and merge" or "Rebase and merge"
```

## Modifying the Ruleset

To change protection rules:

1. Edit `.github/branch-protection-ruleset.json`
2. Re-import via GitHub UI or API
3. Or modify directly in GitHub UI: Settings → Rules → Rulesets

## Ruleset Details

### Required Approvals: 0
Since you're a solo developer, no approvals are required. Change to `1` if you want to review your own PRs.

```json
"required_approving_review_count": 0
```

### Status Check: build-and-push
This is the job name from `.github/workflows/docker-build-push.yml`. It must pass before merging.

```json
"required_status_checks": [
  {
    "context": "build-and-push"
  }
]
```

**Note:** This check won't appear until the workflow has run at least once successfully.

## Temporarily Disabling

If you need to bypass in an emergency:

1. Go to: https://github.com/mdatla/idrac-fan-controller-rust/settings/rules
2. Click on "Protect main branch" ruleset
3. Change **Enforcement status** to "Disabled"
4. Make your changes
5. Re-enable by changing back to "Active"

## Testing the Ruleset

After importing, test that it works:

```bash
# Try to push directly to main (should fail)
git checkout main
echo "test" >> README.md
git add README.md
git commit -m "test direct push"
git push origin main

# Expected error:
# remote: error: GH013: Repository rule violations found
```

If you see this error, the ruleset is working! ✅

## Troubleshooting

### Status check never appears
- Make sure the workflow has run successfully at least once
- Check that the job name is exactly `build-and-push`
- View workflow runs: https://github.com/mdatla/idrac-fan-controller-rust/actions

### Can't merge PR even though CI passed
- Ensure branch is up to date with main
- Click "Update branch" button in PR
- Wait for CI to run again

### Need to make emergency fix
- Temporarily disable ruleset (see "Temporarily Disabling" above)
- Or create a quick PR and merge immediately after CI passes

## Learn More

- [GitHub Rulesets Documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets)
- [Branch Protection Best Practices](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches)
