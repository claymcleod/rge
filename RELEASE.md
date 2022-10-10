# Release

  * [ ] Update version in `Cargo.toml`.
  * [ ] Run tests: `cargo test`.
  * [ ] Run linting: `cargo clippy`.
  * [ ] Run fmt: `cargo fmt`.
  * [ ] Stage changes: `git add Cargo.lock Cargo.toml`.
  * [ ] Create git commit: `git commit -m "release: bumps version to v0.1.0"`.
  * [ ] Create git tag: `git tag v0.1.0`.
  * [ ] Push release: `git push && git push --tags`.
  * [ ] Publish the new crate: `cargo publish`.
  * [ ] Create a release on the Github page for this tag outlining the features
    that were released.