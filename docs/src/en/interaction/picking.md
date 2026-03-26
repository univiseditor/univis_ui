# Picking Backend

File: `src/interaction/picking.rs`

## What It Does

- reads pointer locations
- converts pointer coordinates from viewport space into world space
- evaluates candidate nodes with an SDF rounded-box test
- rejects hits clipped by ancestors through `UClip`
- removes an ancestor hit if a deeper child wins in the same path

## Important Details

- the main query targets entities that carry `UInteraction`
- final ordering combines:
  - tree depth
  - root capsule precedence
  - local depth inside the same root

## Clip Accuracy

The backend walks clip ancestors, converts the pointer into local space for each one, and evaluates `sd_rounded_box` against clip bounds. This keeps picking aligned with visible clipping.
