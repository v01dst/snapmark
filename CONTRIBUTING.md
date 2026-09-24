# Contributing to snapmark

Thanks for helping improve snapmark.

## Before you start

    cargo fmt --check
    cargo test
    cargo clippy --all-targets --all-features -- -D warnings

## Good first contributions

- Improve search behavior or ranking.
- Add shell completion support.
- Expand import/export compatibility.
- Add regression tests for storage and CLI edge cases.
- Improve the local dashboard without adding a cloud dependency.

## Pull requests

Keep each PR focused on one change. Include tests for behavior changes and describe any CLI or data-format compatibility impact.

For storage changes, pay particular attention to atomic writes and backward-compatible JSON handling.
