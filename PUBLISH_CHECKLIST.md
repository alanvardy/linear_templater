# Publish Checklist

## Publish to Cargo 

This checklist is just here for me to reduce the friction of publishing new versions.

Code changes

1. Update dependencies and make sure nothing broke with `./update_test.sh`
2. Change version in Cargo.toml and in this document (do a global find and replace)
3. Update CHANGELOG.md with version number
4. Update README.md with help text `cargo run -- -h`
5. Add any new examples to README.md
6. Open PR for version and wait for it to pass
7. Commit and merge PR

8. Build release

```bash
git checkout main
git pull
```

9. Publish to Cargo with `cargo publish`

