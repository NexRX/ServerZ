# Remaining daisyUI Integration Work

This is the current, reduced checklist. Completed component wrappers, variants, showcase examples, modal/drawer behavior, OTP synchronization, carousel anchors/indicators, and Cally output integration have been removed from this list.

## 1. Verify browser behavior

These items are implemented in Rust/HTML but still need browser verification:

- [ ] Verify Cally date selection, keyboard navigation, month navigation, and min/max constraints.
- [ ] Verify carousel smooth scrolling, slide anchors, indicators, and keyboard focus behavior.
- [ ] Verify native dialog modal focus trapping, Escape handling, backdrop closing, and trigger behavior.
- [ ] Verify Popover API modal behavior in supported browsers.
- [ ] Verify drawer overlay closing, end placement, and initially-open behavior.
- [ ] Verify OTP typing, paste, autofill, deletion, reset, and screen-reader behavior.
- [ ] Verify accordion and collapse keyboard behavior and group isolation.
- [ ] Verify megamenu nested popovers and mobile fallback behavior.

## 2. Complete showcase coverage

The showcase exists in `src/app.rs`, but the following examples still need to be added or connected to the reusable Rust wrappers:

- [x] Add complete Cally constraints/output example using the reusable component.
- [x] Add carousel controls and active indicator examples using anchor controls and smooth-scroll buttons.
- [x] Add native dialog and Popover API modal examples.
- [x] Add reusable drawer markup and overlay example.
- [x] Add accordion radio-group examples.
- [x] Add rating half-star and reset examples.
- [x] Add nested details submenu examples.
- [x] Add megamenu responsive/mobile examples.
- [x] Add table pinned-row and pinned-column examples.
- [x] Add footer responsive direction examples.
- [x] Add representative shared color, style, and size variant coverage to the showcase.

## 3. Remaining component API improvements

The base wrappers and most documented parts now exist. The following API refinements remain:

### Typed enums instead of free-form strings

- [x] Replace mask half modifiers with the typed `MaskHalf` enum.
- [x] Replace aura size and duration parameters with typed `AuraSize` and numeric duration values.
- [x] Replace megamenu layout parameters with the typed `MegamenuLayout` enum.
- [x] Add typed FAB action/label/tooltip helpers.
- [x] Add typed table style combinations.
- [x] Add typed rating mode and reset helpers.
- [x] Add typed tab panel selection and accessibility relationship support.

### Component-specific behavior

- [x] Add selected tab panels with `hidden` and `aria-labelledby` synchronization support.
- [x] Add a native details-based `menu_paged` submenu primitive.
- [x] Add responsive megamenu mobile popover fallback in the showcase.
- [ ] Add carousel runtime active-slide synchronization if anchor-only navigation is insufficient.
- [x] Add explicit accordion group APIs that carry a unique group name.
- [x] Add typed carousel vertical direction and alignment helpers.
- [x] Add range direction handling that only emits `range-vertical` when vertical.

## 4. daisyUI feature parity gaps

These are lower-level coverage gaps still worth implementing after the browser and showcase checks:

- [x] Chat metadata and image examples with a documented bubble color.
- [x] Dock label examples.
- [x] Footer responsive horizontal/vertical examples.
- [x] Hero background-image and overlay composition examples.
- [x] List column grow/wrap composition examples.
- [x] Stats vertical layout with figure, description, and action examples.
- [x] Steps icon and custom `data-content` examples.
- [x] Timeline box, compact, and vertical direction examples.
- [x] Table zebra, pinned row, and pinned column examples.
- [x] Swap rotate, flip, active, and indeterminate examples.
- [x] Mask shape and half-mask examples in the showcase.
- [x] Aura style, size, and color examples in the showcase.
- [x] FAB flower and tooltip examples.
- [x] Megamenu active indicator, wide layout, and responsive examples.

## 5. Dependency and asset decisions

- [x] Keep Cally loaded from `https://unpkg.com/cally`; it is a browser custom element dependency and is not part of the Rust asset bundle.
- [x] No bundling changes required; the existing Tailwind 4 asset pipeline remains unchanged.
- [x] Confirm generated CSS includes the daisyUI classes used by the Rust sources.
- [x] Confirm generated stylesheet version is daisyUI 5.7.50.

## 6. Tests and accessibility

- [x] Add Rust tests for shared enum-to-class mappings.
- [x] Add countdown clamping test coverage.
- [ ] Add browser tests for modal, drawer, accordion, carousel, OTP, tabs, dropdowns, and calendar.
- [ ] Check keyboard focus order and Escape behavior.
- [ ] Check `aria-current`, `aria-live`, `aria-label`, `role`, and form-label relationships.
- [ ] Check unique IDs for dialogs, drawers, popovers, carousel slides, and radio groups.
- [ ] Check dark/custom theme contrast in the showcase.

## 7. Cleanup

- [ ] Exercise the shared enums in the showcase to eliminate dead-code warnings.
- [ ] Remove duplicate component-specific variant enums where shared enums are safe and compatible.
- [x] Document browser-dependent behavior in the verification section.
- [ ] Keep existing public component signatures backward compatible where practical.

## Validation commands

```sh
cargo fmt --check
cargo check
```

Browser verification is required for behavior that depends on Cally, Popover API, native dialogs, scrolling, or inline scripts.
