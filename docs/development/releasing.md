# Releasing

## Workflows

| Workflow | Trigger | Does |
| --- | --- | --- |
| `ci.yml` | Push to any branch, pull requests | Discipline, typecheck, verify, in a Fedora container |
| `release.yml` | Push of a `v*` tag | Builds RPM, DEB and AppImage in parallel, publishes them to the release |

Both install Rust with rustup into an explicit `CARGO_HOME`, because a
container job runs as root while `HOME` points elsewhere and rustup would
otherwise install to a prefix the path does not include.

## Cutting a release

1. Move the entries under `## Unreleased` in `CHANGELOG.md` into a new
   `## x.y.z` section.
2. Set the version in the same place in all four:

```sh
package.json                                       "version"
Cargo.toml                                         [workspace.package] version
CHANGELOG.md                                       the newest heading
packaging/fedora/app.ticket.native.metainfo.xml    <release version=...>
```

3. Run the gate and make sure nothing reports FAIL or SKIP:

```sh
bun run verify
```

4. Commit, push, and wait for CI to pass. Do not tag before CI is green; a
   broken tag burns a release run.
5. Tag and push the tag:

```sh
git tag v0.2.0
git push origin v0.2.0
```

The release workflow builds the three packages and attaches them to the
release. `publish` runs only after all three build jobs succeed, so a partial
release cannot happen.

## Version consistency

The application reads its own version from `package.json` at build time and
compares it against the newest release tag on GitHub. If `package.json` and the
tag disagree, every running copy will either miss an update or announce one
that does not exist.

## Re-running a failed release

Re-running a failed run uses the workflow file from the commit that triggered
it, so a fix in the workflow will not apply. Move the tag instead:

```sh
git tag -f v0.2.0 <fixed-commit>
git push --force origin v0.2.0
```

Only do this while the release has no assets. Once people have downloaded a
package, cut a new version instead.
