# Release Pipeline Architecture

This document explains how GitTop's automated release and distribution pipeline works.

## Overview

```
1. Developer pushes tag: v0.2.0
   │
2. release.yml extracts version: 0.2.0
   │
3. Builds create:
   │  • gittop-windows-x86_64.zip
   │  • gittop-linux-x86_64.tar.gz
   │
4. GitHub Release created with prerelease=false
   │
5. release-meta artifact saved:
   │  • tag: v0.2.0
   │  • is_prerelease: false
   │
6. Downstream workflows trigger:
   │
   ├─▶ aur.yml
   │   • Downloads new tarball
   │   • Updates pkgver=0.2.0
   │   • Recalculates sha256sums
   │   • Pushes to AUR
   │
   └─▶ chocolatey.yml
       • Downloads new zip
       • Calculates checksum
       • Updates nuspec + install script
       • Pushes gittop.0.2.0.nupkg
```

## Workflows

### 1. CI (`ci.yml`)

**Trigger:** Push to `main`, PRs to `main`

**Purpose:** Validate that the code builds on Windows.

| Job | Description |
|-----|-------------|
| `build-windows` | Builds release binary on Windows |

---

### 2. Release (`release.yml`)

**Trigger:** Push tag matching `v*.*.*` (including `-rc`, `-alpha`, `-beta` suffixes)

**Purpose:** Build release artifacts and create GitHub Release.

| Job | Description |
|-----|-------------|
| `build-windows` | Builds Windows `.zip`, extracts version from tag |
| `build-linux` | Builds Linux `.tar.gz` with desktop integration files |
| `release` | Creates GitHub Release, uploads artifacts, saves metadata for downstream |

**Key Output:** `release-meta` artifact containing:
- `tag` — The release tag (e.g., `v0.2.0`)
- `is_prerelease` — Boolean flag (`true` or `false`)

This artifact is consumed by downstream workflows to ensure they operate on the **exact** release that triggered them.

---

### 3. AUR Distribution (`aur.yml`)

**Trigger:** `workflow_run` after `Release` completes successfully

**Purpose:** Publish stable releases to [Arch User Repository](https://aur.archlinux.org/packages/gittop-bin).

| Step | Description |
|------|-------------|
| Download metadata | Gets `release-meta` artifact from triggering workflow |
| Skip prereleases | Exits early if `is_prerelease == true` |
| Publish | Updates PKGBUILD version/checksums, pushes to AUR |

---

### 4. Chocolatey Distribution (`chocolatey.yml`)

**Trigger:** `workflow_run` after `Release` completes successfully

**Purpose:** Publish stable releases to [Chocolatey Community Repository](https://community.chocolatey.org/packages/gittop).

| Step | Description |
|------|-------------|
| Download metadata | Gets `release-meta` artifact from triggering workflow |
| Skip prereleases | Exits early if `is_prerelease == true` |
| Download Windows zip | Fetches from GitHub Release |
| Calculate checksum | SHA256 of the zip file |
| Update package files | Replaces `{{VERSION}}` and `{{CHECKSUM}}` placeholders |
| Pack & Push | Runs `choco pack` and `choco push` |

---

## Packaging Files

### AUR (`packaging/aur/stable-bin/`)

| File | Purpose |
|------|---------|
| `PKGBUILD` | Build instructions for Arch Linux |
| `gittop.install` | Post-install hooks (icon cache, desktop database) |

The PKGBUILD uses `${pkgver}` variable in the source URL, so version updates are automatic.

### Chocolatey (`packaging/chocolatey/`)

| File | Purpose |
|------|---------|
| `gittop.nuspec` | Package metadata (uses `{{VERSION}}` placeholder) |
| `tools/chocolateyInstall.ps1` | Install script (uses `{{VERSION}}`, `{{CHECKSUM}}`) |
| `tools/chocolateyUninstall.ps1` | Uninstall script |

### Scoop (`packaging/scoop/`)

| File | Purpose |
|------|---------|
| `gittop.json` | Scoop manifest |

> **Note:** Scoop publishing is not yet automated. The manifest can be submitted to a Scoop bucket manually or via a dedicated bucket repository.

---

## Adding New Package Managers

To add a new distribution target:

1. Create `packaging/<manager>/` with required files
2. Create `.github/workflows/<manager>.yml` with:
   ```yaml
   on:
     workflow_run:
       workflows: ["Release"]
       types: [completed]
   ```
3. Download `release-meta` artifact using `run-id: ${{ github.event.workflow_run.id }}`
4. Skip prereleases based on `is_prerelease` value
5. Add required secrets to GitHub repository settings
