# Status

Updated 2026-10-20. A piece is done when its tests pass in the gate (`scripts/gates.sh`: Linux, with the
macOS and Windows lints) and the evidence its requirement names is recorded.

## Implemented

- **The bar** (kiln's CP-M1): a Cargo workspace whose lints deny panics, this repository's rules
  (`CLAUDE.md`), the requirement registry (`docs/requirements.toml`), the design's starting point
  (`docs/design/README.md`), the gate (`scripts/gates.sh`, run by kiln CI from `.kiln/ci.yaml`), and the
  vendoring tool (`scripts/vendor.py`, kiln's).
- **The `quiver` binary** says what it is (`quiver --version`, `quiver --help`), and nothing more yet.

## Next

- **CP-T2.1**: Focal's bus (branch `bus-replicated`) vendored behind `quiver-api` with `GroupStore` mandated; kiln vendors Quiver behind `Bus`; the UI's live feed of session events rides the ephemeral class.
- **CP-T2.2**: The persisted class's 1.4 s tail fixed at its cause (a disk stall never costs leadership), cheaper elections, and kiln's CI and deploy events on the persisted class.
- **CP-T2.3**: Typed acknowledgements stating the fate reached and its loss bound.

## Open

Everything else the track's plan names (kiln plan `01a157fdefc37396957ea10adb51a19f`): each goal, its criteria and how each is
measured are in `docs/requirements.toml`.
