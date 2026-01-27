# Agents

This document describes the automated agents, bots, and external services that interact with this repository (CI, dependency managers, automation bots, monitoring integrations, and any AI assistants). Use this as the canonical reference for what agents exist, where their configuration lives, who owns them, and how to safely add/remove or modify them.

> Replace any <placeholders> in this file with the actual names, owners, and configuration paths used by the project.

## Purpose

Agents automate routine tasks so maintainers can focus on design and development. Typical responsibilities include:
- Running CI builds and tests
- Scanning for vulnerabilities and license issues
- Auto-updating dependencies
- Merging or gating PRs according to project rules
- Running scheduled maintenance tasks (formatting, codegen)
- Notifying teams about build/status events

This file documents:
- Which agents are active (and their responsibilities)
- Configuration locations and examples
- Safety, security, and permissions guidance
- Onboarding, troubleshooting, and removal procedures

## Current agents (example list)

Fill in this list with the real agents used in this repository:

- GitHub Actions — CI and test runner
  - Purpose: build, test, and packaging
  - Config: `.github/workflows/*.yml`
  - Owner: `<maintainer-username-or-team>`
- Dependabot — automatic dependency updates
  - Purpose: update dependencies and open PRs
  - Config: `.github/dependabot.yml`
  - Owner: `<maintainer-username-or-team>`
- Renovate (optional) — dependency management alternative
  - Purpose: auto PRs for dependency bumps according to rules
  - Config: `renovate.json` or `.github/renovate.json`
  - Owner: `<maintainer>`
- Code scanning / SAST (CodeQL)
  - Purpose: run security analysis on pushes/PRs
  - Config: `.github/workflows/codeql.yml`
  - Owner: `<security-contact>`
- Merging bot (Mergify, Probot app)
  - Purpose: auto-merge PRs that pass checks and meet rules
  - Config: `.mergify.yml` or app settings
  - Owner: `<maintainer>`
- Coverage reporter (codecov / Coveralls)
  - Purpose: upload coverage reports for PRs
  - Config: workflow steps, repository secret `CODECOV_TOKEN`
  - Owner: `<maintainer>`
- ChatOps / Notifications (Slack/GitHub Apps)
  - Purpose: notify channels about CI status, releases
  - Config: Integration settings, Secrets (webhook URLs)
  - Owner: `<oncall-or-team>`
- AI Assistant / Automation Scripts
  - Purpose: scripted project maintenance or generation (if used)
  - Config: `<path/to/agent/config>`
  - Owner: `<maintainer>`

If an agent does not exist in the repo, remove it from this list.

## Where agent configuration lives

Common config file locations:
- GitHub Actions workflows: `.github/workflows/*.yml`
- Dependabot config: `.github/dependabot.yml`
- Renovate config: `.github/renovate.json` or `renovate.json`
- Mergify: `.mergify.yml`
- Code scanning: `.github/workflows/codeql.yml` (or other workflow files)
- Bot apps: `.github/apps/*`, `.github/probot.js`, or provisioning scripts in `/tools` or `/scripts`
- Scheduling / cron jobs: defined in workflow `on: schedule` sections

Keep a short README or comment near each config file explaining the intent, owner, and how to make safe changes.

## Adding a new agent — checklist

Before adding or enabling a new automated agent, follow this checklist:

1. Design & justification
   - Document the goal and expected behavior in an issue or RFC.
2. Owner & contact
   - Assign a maintainer or team responsible for the agent.
3. Permissions & principle of least privilege
   - Determine the minimum permissions required and prefer short-lived tokens or OIDC where possible.
4. Secrets & credentials
   - Store secrets in GitHub Actions Secrets or Vault; never commit credentials to the repo.
5. Config in repo
   - Add configuration files under `.github/` or a dedicated `/ops` directory with clear comments.
6. Review & approvals
   - Changes to agent configuration (workflows, dependabot, merge rules) require at least one code review from an owner.
7. Testing
   - Test workflows locally (see "Testing locally") and in a fork or feature branch before enabling.
8. Monitoring & alerts
   - Ensure failures are routed to an on-call or team channel.
9. Rollback plan
   - Document how to disable the agent quickly (e.g., remove workflow file, toggle app permissions).
10. Security review
   - For agents that run code or have broad write permissions, perform a brief security review.

## Security & permissions

- Use the principle of least privilege. Grant only the scopes required for the agent to do its job.
- Prefer GitHub OIDC or short-lived tokens over long-lived personal access tokens.
- Store tokens and secrets in repository or organization secrets, not in files.
- Restrict environment secrets to specific workflows if supported.
- Review third-party app permissions before installing them on the organization.
- Periodically rotate credentials and audit access.
- For self-hosted agents/runners:
  - Keep the runner host patched and isolated.
  - Limit network access and ensure runners run in ephemeral environments (e.g., ephemeral VMs/containers).
  - Do not use runners with elevated access to secrets unless necessary and well-audited.

## Testing agents locally

- GitHub Actions:
  - Use act (https://github.com/nektos/act) for local testing of actions when feasible.
  - Use a fork or branch in a test repository to validate workflows that require secrets or organization settings.
- Dependabot / Renovate:
  - Validate configuration with their online/CLI config checkers before merging.
- Bots and Probot apps:
  - Run locally with ngrok for webhooks and validate behavior before deploying.
- Always run CI jobs on non-critical branches first and ensure logs are readable and actionable.

Example: run a workflow with act
```sh
# install act (if not installed)
# run a named GitHub Action locally
act -j <job-id> -P ubuntu-latest=ghcr.io/catthehacker/ubuntu:full-latest
```

Note: act has limitations and may not exactly replicate the hosted runner environment.

## Observability & alerts

- Ensure workflows log useful diagnostic information on failure.
- Configure persistent storage or artifacts for important job outputs (test results, core dumps).
- Integrate CI failures and security alerts into your team’s notification channel (Slack, email).
- Set up paging/escalation for critical pipelines if required.

## Troubleshooting & emergency procedures

Common troubleshooting steps:
1. Re-run the failed workflow from the Actions UI to capture transient failures.
2. Inspect job logs and artifacts attached to the failed run.
3. If a workflow needs to be disabled immediately:
   - Rename or temporarily move the workflow file (e.g., `.github/workflows/ci.yml.disabled`) or
   - Use GitHub UI to disable actions for the repo and/or revert the last change that introduced the problem.
4. For dependency-autoupdate agents opening noisy PRs:
   - Adjust the configuration (update schedule, grouping rules) or temporarily disable the agent.
5. If a bot is merging undesired PRs:
   - Restrict its permissions or remove webhook/app until you fix rules.

Document any escalation path and contact information for the owner(s) above.

## Ownership & maintenance

- Each agent MUST have a listed owner (individual or team). Replace the placeholder below:
  - CI: `<ci-owner>`
  - Dependabot: `<dependabot-owner>`
  - Mergify/Auto-merge: `<merge-owner>`
  - Security scanning: `<security-owner>`

Owners are responsible for:
- Reviewing and approving config changes
- Responding to alerts and failed jobs in a timely manner
- Updating agent configs when the repository structure or tooling changes
- Ensuring agent security and least-privilege posture

## Removal / decommissioning

To remove an agent:
1. Disable it in any external service dashboard (Dependabot/renovate UI, third-party bot settings).
2. Remove or archive configuration files in the repo.
3. Revoke any tokens/credentials used by the agent.
4. Update this AGENTS.md to remove the agent and record the date and reason for removal.
5. Communicate the change to stakeholders.

## Examples / templates

A minimal GitHub Actions workflow to run tests:

```yaml
# .github/workflows/ci.yml
name: CI
on:
  push:
    branches: [ main ]
  pull_request:
    types: [ opened, synchronize, reopened ]

jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: cargo build --workspace --all-targets --release
      - name: Test
        run: cargo test --workspace --all
```

A minimal Dependabot config:

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5
```

A Renovate example (if used):

```json
// .github/renovate.json
{
  "extends": ["config:base"],
  "timezone": "UTC",
  "schedule": ["before 3am on monday"]
}
```

## FAQ

Q: Who should I contact when CI is broken?
A: Contact the CI owner listed in the Ownership section; if unavailable, open an issue and add the `ci` label.

Q: Can I give an agent admin permissions to the repo?
A: Only with explicit justification and a security review. Prefer scoped permissions and an organization-level service account.

Q: Where do I put agent-related scripts?
A: Use `/tools`, `/scripts`, or `.github/actions` with README comments so future maintainers understand intent.

## Change log

- 2026-01-19 — Initial AGENTS.md template added. Replace placeholders with real values.

---

If you want, I can:
- Populate the "Current agents" section with actual agents discovered in the repository (I can inspect the repo and list workflows and config files).
- Produce pull request text to add/modify any agent config files.
- Generate a short checklist PR template for adding new agents.

Tell me which of the above you want next and I will proceed. 
