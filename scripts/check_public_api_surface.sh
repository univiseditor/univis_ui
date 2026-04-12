#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="$ROOT_DIR/target/public_api_surface"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

BEVY_DEP='bevy = { version = "0.18.1", default-features = false, features = ["default_app", "default_platform", "common_api", "bevy_render", "bevy_core_pipeline", "bevy_sprite", "bevy_sprite_render", "bevy_gizmos_render", "bevy_pbr", "bevy_picking", "keyboard", "mouse"] }'

have_rg() {
  command -v rg >/dev/null 2>&1
}

match_quiet() {
  local pattern="$1"
  local file="$2"

  if have_rg; then
    rg -q -- "$pattern" "$file"
  else
    grep -Eq -- "$pattern" "$file"
  fi
}

find_example_matches() {
  local pattern="$1"

  if have_rg; then
    rg -n --glob '**/examples/*.rs' -- "$pattern" crates
  else
    find crates -type f -path '*/examples/*.rs' -print0 \
      | xargs -0 grep -nE -- "$pattern"
  fi
}

write_case() {
  local case_dir="$1"
  local body="$2"
  mkdir -p "$case_dir/src"
  cat >"$case_dir/Cargo.toml" <<EOF
[package]
name = "$(basename "$case_dir")"
version = "0.0.0"
edition = "2024"

[dependencies]
$BEVY_DEP
univis_ui = { path = "$ROOT_DIR" }
univis_ui_engine = { path = "$ROOT_DIR/crates/univis_ui_engine" }
univis_ui_style = { path = "$ROOT_DIR/crates/univis_ui_style" }
univis_ui_interaction = { path = "$ROOT_DIR/crates/univis_ui_interaction" }
univis_ui_widgets = { path = "$ROOT_DIR/crates/univis_ui_widgets" }
EOF
  printf '%s\n' "$body" >"$case_dir/src/main.rs"
}

run_success_case() {
  local name="$1"
  local body="$2"
  local case_dir="$TMP_DIR/$name"
  local log_file="$case_dir.log"

  write_case "$case_dir" "$body"
  echo "== Public API case: $name =="
  if ! CARGO_TARGET_DIR="$TARGET_DIR" cargo check --quiet --manifest-path "$case_dir/Cargo.toml" >"$log_file" 2>&1; then
    cat "$log_file"
    echo "Public API case '$name' failed unexpectedly." >&2
    exit 1
  fi
}

run_failure_case() {
  local name="$1"
  local body="$2"
  local expected_pattern="$3"
  local case_dir="$TMP_DIR/$name"
  local log_file="$case_dir.log"

  write_case "$case_dir" "$body"
  echo "== Public API negative case: $name =="
  if CARGO_TARGET_DIR="$TARGET_DIR" cargo check --quiet --manifest-path "$case_dir/Cargo.toml" >"$log_file" 2>&1; then
    cat "$log_file"
    echo "Public API case '$name' was expected to fail but passed." >&2
    exit 1
  fi

  if ! match_quiet "$expected_pattern" "$log_file"; then
    cat "$log_file"
    echo "Public API case '$name' failed, but not for the expected reason." >&2
    exit 1
  fi
}

run_success_case "facade_prelude_surface" '
use univis_ui::prelude::*;

fn main() {
    let _root = URootUi::screen();
    let _node = UNode::default();
    let _padding = USides::all(8.0);
    let _button = UButton::primary();
    let _panel = UPanel::card();
    let _select = USelect::new();
    let _label = UTextLabel::default();
    let _interaction = UInteraction::default();
    let _colors = UInteractionColors::default();
}
'

run_success_case "plugin_combinations" '
use bevy::prelude::*;
use univis_ui::UnivisUiPlugin;
use univis_ui_engine::UnivisEnginePlugin;
use univis_ui_interaction::interaction::UnivisInteractionPlugin;
use univis_ui_style::style::UnivisUiStylePlugin;
use univis_ui_widgets::widget::UnivisWidgetPlugin;
use univis_ui_widgets::widget::select::UnivisSelectPlugin;
use univis_ui_widgets::widget::text_field::UnivisTextFieldPlugin;
use univis_ui_widgets::widget::text_label::UnivisTextPlugin;

fn main() {
    let mut facade_app = App::new();
    facade_app.add_plugins(UnivisUiPlugin);

    let mut direct_app = App::new();
    direct_app.add_plugins((
        UnivisUiStylePlugin,
        UnivisEnginePlugin,
        UnivisInteractionPlugin,
        UnivisWidgetPlugin,
    ));

    let mut narrow_app = App::new();
    narrow_app.add_plugins((
        UnivisUiStylePlugin,
        UnivisEnginePlugin,
        UnivisInteractionPlugin,
        UnivisTextPlugin,
        UnivisSelectPlugin,
        UnivisTextFieldPlugin,
    ));
}
'

run_success_case "widget_schedule_surface" '
use univis_ui::widget::schedule::UnivisWidgetUpdateSet as FacadeWidgetSet;
use univis_ui_widgets::schedule::UnivisWidgetUpdateSet as WidgetSet;

fn main() {
    let _facade = FacadeWidgetSet::Build;
    let _widget = WidgetSet::Events;
    let _ = (_facade, _widget);
}
'

run_success_case "widget_layered_surface" '
use bevy::prelude::*;
use univis_ui::widget::interactive::{UButton, UnivisInteractiveWidgetPlugin};
use univis_ui::widget::visual::{UDivider, UnivisVisualWidgetPlugin};
use univis_ui_widgets::widget::interactive::UTextField;
use univis_ui_widgets::widget::visual::UProgressBar;

fn main() {
    let _button = UButton::primary();
    let _divider = UDivider::horizontal();
    let _progress = UProgressBar::default();
    let _text_field = UTextField::default();

    let mut app = App::new();
    app.add_plugins((UnivisVisualWidgetPlugin, UnivisInteractiveWidgetPlugin));

    let _ = (_button, _divider, _progress, _text_field, app);
}
'

run_success_case "deprecated_explicit_paths" '
use univis_ui::layout::layout_system::{UScreenRoot, UWorldRoot};

fn main() {
    let _screen = UScreenRoot;
    let _world = UWorldRoot::default();
    let _ = (_screen, _world);
}
'

run_failure_case "deprecated_not_in_prelude" '
use univis_ui::prelude::*;

fn main() {
    let _screen = UScreenRoot;
    let _world = UWorldRoot::default();
    let _ = (_screen, _world);
}
' 'UScreenRoot|UWorldRoot'

run_failure_case "engine_internal_not_public" '
use univis_ui_engine::internal::ComputedSize;

fn main() {
    let _size = ComputedSize::default();
}
' 'module `internal` is private|private module'

run_success_case "engine_query_surface" '
use univis_ui_engine::layout::query::{
    CachedUiContext, ComputedSize, IntrinsicSize, LayoutDepth, ResolvedRootStack, ResolvedRootUi,
    UiPickingContext,
};

fn main() {
    let _size = ComputedSize::default();
    let _intrinsic = IntrinsicSize::default();
    let _depth = LayoutDepth::default();
    let _context = CachedUiContext::default();
    let _picking = UiPickingContext::default();
    let _stack = ResolvedRootStack::default();
    let _root = ResolvedRootUi::default();
    let _ = (_size, _intrinsic, _depth, _context, _picking, _stack, _root);
}
'

run_success_case "engine_schedule_surface" '
use univis_ui_engine::schedule::{
    UiPickingRuntimeState, UiSettlementRuntimeState, UnivisPostUpdateSet,
};

fn main() {
    let _prepare = UnivisPostUpdateSet::ExternalPrepare;
    let _post_solve = UnivisPostUpdateSet::ExternalPostSolve;
    let _picking_runtime = UiPickingRuntimeState::default();
    let _settlement_runtime = UiSettlementRuntimeState::default();
    let _ = (_prepare, _post_solve, _picking_runtime, _settlement_runtime);
}
'

run_success_case "interaction_picking_surface" '
use univis_ui_interaction::interaction::picking::{PickingSyncState, PickingValidationState};

fn main() {
    let _sync = PickingSyncState::default();
    let _validation = PickingValidationState::default();
    let _ = (_sync, _validation);
}
'

echo "== Public API example guard: canonical roots only =="
if find_example_matches 'UScreenRoot|UWorldRoot'; then
  echo "Examples should use canonical `URootUi` roots instead of deprecated wrappers." >&2
  exit 1
fi

echo
echo "Public API surface checks completed successfully."
