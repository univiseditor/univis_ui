use bevy::prelude::*;
use crate::internal_prelude::*;

pub struct UnivisSelectPlugin;

impl Plugin for UnivisSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<USelect>()
            .register_type::<USelectOption>()
            .add_message::<SelectChangedEvent>()
            .add_message::<SelectOpenStateChangedEvent>()
            .init_resource::<ActiveSelect>()
            .add_systems(Update, (
                init_select_visuals,
                enforce_select_invariants,
                handle_select_trigger_interaction,
                handle_select_option_interaction,
                handle_select_keyboard,
                close_select_on_outside_click,
                sync_select_dropdown_tree,
                update_select_visuals,
                emit_select_events,
            ).chain());
    }
}

#[derive(Clone, Debug, Reflect)]
pub struct USelectOption {
    pub label: String,
    pub value: String,
    pub disabled: bool,
}

impl USelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
#[require(UNode, ULayout)]
pub struct USelect {
    pub options: Vec<USelectOption>,
    pub selected_index: Option<usize>,
    previous_selected_index: Option<usize>,
    pub highlighted_index: Option<usize>,
    pub placeholder: String,
    pub is_open: bool,
    previous_open: bool,
    pub width: f32,
    pub trigger_height: f32,
    pub max_visible_options: usize,
    pub disabled: bool,
    pub font_size: f32,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub background: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub dropdown_background: Color,
    pub border_color: Color,
    pub option_hover_color: Color,
    pub option_selected_color: Color,
    pub option_disabled_text_color: Color,
    pub padding: USides,
}

impl Default for USelect {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected_index: None,
            previous_selected_index: None,
            highlighted_index: None,
            placeholder: "Select an option".to_string(),
            is_open: false,
            previous_open: false,
            width: 220.0,
            trigger_height: 38.0,
            max_visible_options: 6,
            disabled: false,
            font_size: 15.0,
            text_color: Color::WHITE,
            placeholder_color: Color::srgb(0.62, 0.67, 0.76),
            background: Color::srgb(0.14, 0.16, 0.22),
            hover_color: Color::srgb(0.18, 0.2, 0.28),
            pressed_color: Color::srgb(0.12, 0.14, 0.2),
            dropdown_background: Color::srgb(0.12, 0.14, 0.2),
            border_color: Color::srgb(0.36, 0.42, 0.52),
            option_hover_color: Color::srgb(0.2, 0.28, 0.42),
            option_selected_color: Color::srgb(0.2, 0.34, 0.54),
            option_disabled_text_color: Color::srgb(0.45, 0.48, 0.56),
            padding: USides::axes(12.0, 8.0),
        }
    }
}

impl USelect {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(mut self, options: Vec<USelectOption>) -> Self {
        self.options = options;
        sanitize_select(&mut self);
        self.previous_selected_index = self.selected_index;
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_selected_index(mut self, index: usize) -> Self {
        if index < self.options.len() && !self.options[index].disabled {
            self.selected_index = Some(index);
        } else {
            self.selected_index = None;
        }
        self.highlighted_index = self.selected_index;
        self.previous_selected_index = self.selected_index;
        self
    }

    pub fn with_selected_value(mut self, value: impl AsRef<str>) -> Self {
        let value = value.as_ref();
        self.selected_index = self
            .options
            .iter()
            .position(|opt| !opt.disabled && opt.value == value);
        self.highlighted_index = self.selected_index;
        self.previous_selected_index = self.selected_index;
        self
    }

    pub fn with_size(mut self, width: f32, trigger_height: f32) -> Self {
        self.width = width.max(1.0);
        self.trigger_height = trigger_height.max(1.0);
        self
    }

    pub fn with_max_visible_options(mut self, max_visible_options: usize) -> Self {
        self.max_visible_options = max_visible_options.max(1);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self.is_open = false;
        self
    }
}

#[derive(Component)]
struct SelectRuntime {
    trigger_entity: Entity,
    value_label_entity: Entity,
    chevron_entity: Entity,
    dropdown_entity: Option<Entity>,
}

#[derive(Component, Clone, Copy)]
struct SelectTrigger {
    select: Entity,
}

#[derive(Component)]
struct SelectValueLabel;

#[derive(Component)]
struct SelectChevronLabel;

#[derive(Component, Clone, Copy)]
struct SelectDropdown {
    select: Entity,
}

#[derive(Component, Clone, Copy)]
struct SelectOptionRow {
    select: Entity,
    index: usize,
}

#[derive(Component)]
struct SelectOptionLabel;

#[derive(Resource, Default)]
struct ActiveSelect {
    entity: Option<Entity>,
}

fn init_select_visuals(
    mut commands: Commands,
    theme: Res<Theme>,
    mut query: Query<(Entity, &mut USelect), Added<USelect>>,
) {
    for (entity, mut select) in query.iter_mut() {
        sanitize_select(&mut select);

        let mut trigger_entity = None;
        let mut value_label_entity = None;
        let mut chevron_entity = None;

        commands.entity(entity).insert((
            UNode {
                width: UVal::Px(select.width),
                height: UVal::Content,
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
                ..default()
            },
        ));

        commands.entity(entity).with_children(|parent| {
            let trigger = parent
                .spawn((
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(select.trigger_height),
                        padding: select.padding,
                        background_color: select.background,
                        border_radius: UCornerRadius::all(8.0),
                        ..default()
                    },
                    UBorder {
                        color: select.border_color,
                        width: 1.0,
                        radius: UCornerRadius::all(8.0),
                        offset: 0.0,
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                    Pickable::default(),
                    UInteraction::default(),
                    UInteractionColors {
                        normal: select.background,
                        hovered: select.hover_color,
                        pressed: select.pressed_color,
                    },
                    SelectTrigger { select: entity },
                ))
                .with_children(|trigger_parent| {
                    let text = selected_option(&select)
                        .map(|opt| opt.label.clone())
                        .unwrap_or_else(|| select.placeholder.clone());
                    let color = if select.selected_index.is_some() {
                        select.text_color
                    } else {
                        select.placeholder_color
                    };

                    let value_label = trigger_parent
                        .spawn((
                            UTextLabel {
                                text,
                                font_size: select.font_size,
                                color,
                                autosize: false,
                                ..default()
                            },
                            SelectValueLabel,
                            Pickable::IGNORE,
                        ))
                        .id();

                    let chevron = trigger_parent
                        .spawn((
                            UTextLabel {
                                text: Icon::CHEVRON_DOWN.to_string(),
                                font_size: select.font_size,
                                color: select.text_color,
                                font: theme.icon.font.clone(),
                                autosize: true,
                                ..default()
                            },
                            SelectChevronLabel,
                            Pickable::IGNORE,
                        ))
                        .id();

                    value_label_entity = Some(value_label);
                    chevron_entity = Some(chevron);
                })
                .id();

            trigger_entity = Some(trigger);
        });

        commands.entity(entity).insert(SelectRuntime {
            trigger_entity: trigger_entity.expect("select trigger must exist"),
            value_label_entity: value_label_entity.expect("select value label must exist"),
            chevron_entity: chevron_entity.expect("select chevron must exist"),
            dropdown_entity: None,
        });
    }
}

fn enforce_select_invariants(
    mut query: Query<(Entity, &mut USelect)>,
    mut active: ResMut<ActiveSelect>,
) {
    let mut first_open = None;
    let mut active_is_valid = false;

    for (entity, mut select) in query.iter_mut() {
        sanitize_select(&mut select);

        if select.disabled {
            select.is_open = false;
        }

        if select.is_open {
            if first_open.is_none() {
                first_open = Some(entity);
            }
            if active.entity == Some(entity) {
                active_is_valid = true;
            }
        }
    }

    if active.entity.is_none() {
        active.entity = first_open;
    } else if !active_is_valid {
        active.entity = first_open;
    }
}

fn handle_select_trigger_interaction(
    mut select_query: Query<&mut USelect>,
    trigger_query: Query<(&SelectTrigger, &UInteraction)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut active: ResMut<ActiveSelect>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let mut pressed_select = None;
    for (marker, interaction) in trigger_query.iter() {
        if *interaction == UInteraction::Pressed {
            pressed_select = Some(marker.select);
            break;
        }
    }

    let Some(select_entity) = pressed_select else {
        return;
    };

    let should_open = {
        let Ok(mut select) = select_query.get_mut(select_entity) else {
            return;
        };

        if select.disabled {
            return;
        }

        let should_open = !select.is_open;
        select.is_open = should_open;

        if should_open {
            if !is_enabled_index(&select.options, select.highlighted_index) {
                select.highlighted_index = select
                    .selected_index
                    .filter(|idx| is_enabled_index(&select.options, Some(*idx)))
                    .or_else(|| first_enabled_index(&select.options));
            }
        }
        should_open
    };

    if should_open {
        if let Some(other_entity) = active.entity {
            if other_entity != select_entity {
                if let Ok(mut other_select) = select_query.get_mut(other_entity) {
                    other_select.is_open = false;
                }
            }
        }
        active.entity = Some(select_entity);
    } else if active.entity == Some(select_entity) {
        active.entity = None;
    }
}

fn handle_select_option_interaction(
    mut select_query: Query<&mut USelect>,
    option_query: Query<(&SelectOptionRow, &UInteraction)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut active: ResMut<ActiveSelect>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let mut pressed_row = None;
    for (row, interaction) in option_query.iter() {
        if *interaction == UInteraction::Pressed {
            pressed_row = Some(*row);
            break;
        }
    }

    let Some(row) = pressed_row else {
        return;
    };

    let Ok(mut select) = select_query.get_mut(row.select) else {
        return;
    };

    if !select.is_open || select.disabled {
        return;
    }

    let Some(option) = select.options.get(row.index) else {
        return;
    };

    if option.disabled {
        return;
    }

    select.selected_index = Some(row.index);
    select.highlighted_index = Some(row.index);
    select.is_open = false;

    if active.entity == Some(row.select) {
        active.entity = None;
    }
}

fn handle_select_keyboard(
    mut select_query: Query<&mut USelect>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active: ResMut<ActiveSelect>,
) {
    let has_input = keyboard.just_pressed(KeyCode::ArrowDown)
        || keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::Escape);

    if !has_input {
        return;
    }

    let Some(active_entity) = active.entity else {
        return;
    };

    let Ok(mut select) = select_query.get_mut(active_entity) else {
        active.entity = None;
        return;
    };

    if !select.is_open || select.disabled {
        active.entity = None;
        return;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        select.is_open = false;
        active.entity = None;
        return;
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) {
        select.highlighted_index =
            next_enabled_index(&select.options, select.highlighted_index, 1);
    }

    if keyboard.just_pressed(KeyCode::ArrowUp) {
        select.highlighted_index =
            next_enabled_index(&select.options, select.highlighted_index, -1);
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        if let Some(highlighted) = select.highlighted_index {
            if is_enabled_index(&select.options, Some(highlighted)) {
                select.selected_index = Some(highlighted);
                select.is_open = false;
                active.entity = None;
            }
        }
    }
}

fn close_select_on_outside_click(
    mut select_query: Query<&mut USelect>,
    trigger_query: Query<&UInteraction, With<SelectTrigger>>,
    option_query: Query<&UInteraction, With<SelectOptionRow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut active: ResMut<ActiveSelect>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let clicked_on_select = trigger_query
        .iter()
        .any(|state| *state == UInteraction::Pressed)
        || option_query
            .iter()
            .any(|state| *state == UInteraction::Pressed);

    if clicked_on_select {
        return;
    }

    let mut closed_any = false;
    for mut select in select_query.iter_mut() {
        if select.is_open {
            select.is_open = false;
            closed_any = true;
        }
    }

    if closed_any {
        active.entity = None;
    }
}

fn sync_select_dropdown_tree(
    mut commands: Commands,
    mut query: Query<(Entity, &USelect, &mut SelectRuntime)>,
    children_query: Query<(&Children, &SelectDropdown)>,
    option_row_query: Query<&SelectOptionRow>,
) {
    for (entity, select, mut runtime) in query.iter_mut() {
        if !select.is_open {
            if let Some(dropdown_entity) = runtime.dropdown_entity.take() {
                commands.entity(dropdown_entity).despawn();
            }
            continue;
        }

        let mut needs_rebuild = runtime.dropdown_entity.is_none();
        if let Some(dropdown_entity) = runtime.dropdown_entity {
            if let Ok((children, marker)) = children_query.get(dropdown_entity) {
                if marker.select != entity {
                    needs_rebuild = true;
                }
                let row_count = children
                    .iter()
                    .filter(|child| option_row_query.get(*child).is_ok())
                    .count();
                if row_count != select.options.len() {
                    needs_rebuild = true;
                }
            } else {
                needs_rebuild = true;
            }
        }

        if needs_rebuild {
            if let Some(dropdown_entity) = runtime.dropdown_entity.take() {
                commands.entity(dropdown_entity).despawn();
            }
            runtime.dropdown_entity = Some(spawn_dropdown(&mut commands, entity, select));
        }
    }
}

fn update_select_visuals(
    mut root_query: Query<
        (Entity, &USelect, &SelectRuntime, &mut UNode),
        (
            With<USelect>,
            Without<SelectTrigger>,
            Without<SelectDropdown>,
            Without<SelectOptionRow>,
        ),
    >,
    mut trigger_query: Query<
        (&UInteraction, &mut UNode, &mut UBorder, &mut UInteractionColors),
        (With<SelectTrigger>, Without<SelectOptionRow>, Without<SelectDropdown>),
    >,
    mut value_label_query: Query<
        &mut UTextLabel,
        (
            With<SelectValueLabel>,
            Without<SelectChevronLabel>,
            Without<SelectOptionLabel>,
        ),
    >,
    mut chevron_label_query: Query<
        &mut UTextLabel,
        (
            With<SelectChevronLabel>,
            Without<SelectValueLabel>,
            Without<SelectOptionLabel>,
        ),
    >,
    mut dropdown_query: Query<
        (&SelectDropdown, &mut UNode, &mut UBorder, &mut UClip, &Children),
        (With<SelectDropdown>, Without<SelectTrigger>, Without<SelectOptionRow>),
    >,
    mut row_query: Query<
        (
            &SelectOptionRow,
            &UInteraction,
            &mut UNode,
            &mut UInteractionColors,
            &Children,
        ),
        (With<SelectOptionRow>, Without<SelectTrigger>, Without<SelectDropdown>),
    >,
    mut option_label_query: Query<
        &mut UTextLabel,
        (
            With<SelectOptionLabel>,
            Without<SelectValueLabel>,
            Without<SelectChevronLabel>,
        ),
    >,
) {
    for (_entity, select, runtime, mut root_node) in root_query.iter_mut() {
        root_node.width = UVal::Px(select.width.max(1.0));
        root_node.height = UVal::Content;
        root_node.background_color = Color::NONE;

        if let Ok((interaction, mut trigger_node, mut trigger_border, mut trigger_colors)) =
            trigger_query.get_mut(runtime.trigger_entity)
        {
            trigger_node.width = UVal::Percent(1.0);
            trigger_node.height = UVal::Px(select.trigger_height.max(1.0));
            trigger_node.padding = select.padding;
            trigger_node.border_radius = UCornerRadius::all(8.0);

            trigger_colors.normal = select.background;
            trigger_colors.hovered = if select.disabled {
                select.background
            } else {
                select.hover_color
            };
            trigger_colors.pressed = if select.disabled {
                select.background
            } else {
                select.pressed_color
            };

            trigger_border.color = select.border_color;
            trigger_border.width = 1.0;
            trigger_border.radius = UCornerRadius::all(8.0);

            trigger_node.background_color = if select.disabled {
                select.background
            } else {
                match *interaction {
                    UInteraction::Pressed | UInteraction::Clicked => select.pressed_color,
                    UInteraction::Hovered | UInteraction::Released => select.hover_color,
                    UInteraction::Normal => select.background,
                }
            };
        }

        if let Ok(mut label) = value_label_query.get_mut(runtime.value_label_entity) {
            if let Some(option) = selected_option(select) {
                label.text = option.label.clone();
                label.color = select.text_color;
            } else {
                label.text = select.placeholder.clone();
                label.color = select.placeholder_color;
            }
            label.font_size = select.font_size;
        }

        if let Ok(mut chevron) = chevron_label_query.get_mut(runtime.chevron_entity) {
            chevron.text = if select.is_open {
                Icon::CHEVRONS_UP.to_string()
            } else {
                Icon::CHEVRON_DOWN.to_string()
            };
            chevron.font_size = select.font_size;
            chevron.color = select.text_color;
        }

        if let Some(dropdown_entity) = runtime.dropdown_entity {
            if let Ok((dropdown, mut dropdown_node, mut dropdown_border, mut clip, dropdown_children)) =
                dropdown_query.get_mut(dropdown_entity)
            {
                if dropdown.select != _entity {
                    continue;
                }

                let max_visible = select.max_visible_options.max(1);
                let should_clip = select.options.len() > max_visible;
                dropdown_node.width = UVal::Px(select.width.max(1.0));
                dropdown_node.height = if should_clip {
                    UVal::Px(select.trigger_height.max(1.0) * max_visible as f32)
                } else {
                    UVal::Content
                };
                dropdown_node.background_color = select.dropdown_background;
                dropdown_node.border_radius = UCornerRadius::all(8.0);
                dropdown_border.color = select.border_color;
                dropdown_border.width = 1.0;
                dropdown_border.radius = UCornerRadius::all(8.0);
                clip.enabled = should_clip;

                for child in dropdown_children.iter() {
                    if let Ok((row, interaction, mut row_node, mut row_colors, row_children)) =
                        row_query.get_mut(child)
                    {
                        let Some(option) = select.options.get(row.index) else {
                            continue;
                        };

                        let is_selected = select.selected_index == Some(row.index);
                        let is_highlighted = select.highlighted_index == Some(row.index);

                        row_node.width = UVal::Percent(1.0);
                        row_node.height = UVal::Px(select.trigger_height.max(1.0));
                        row_node.padding = select.padding;

                        row_colors.normal = Color::NONE;
                        row_colors.hovered = if option.disabled {
                            Color::NONE
                        } else {
                            select.option_hover_color
                        };
                        row_colors.pressed = if option.disabled {
                            Color::NONE
                        } else {
                            select.option_hover_color
                        };

                        row_node.background_color = if option.disabled {
                            Color::NONE
                        } else if is_selected {
                            select.option_selected_color
                        } else if is_highlighted {
                            select.option_hover_color
                        } else {
                            match *interaction {
                                UInteraction::Hovered
                                | UInteraction::Pressed
                                | UInteraction::Clicked
                                | UInteraction::Released => select.option_hover_color,
                                UInteraction::Normal => Color::NONE,
                            }
                        };

                        for row_child in row_children.iter() {
                            if let Ok(mut label) = option_label_query.get_mut(row_child) {
                                label.text = option.label.clone();
                                label.font_size = select.font_size;
                                label.color = if option.disabled {
                                    select.option_disabled_text_color
                                } else {
                                    select.text_color
                                };
                            }
                        }
                    }
                }
            }
        }
    }
}

fn emit_select_events(
    mut changed_events: MessageWriter<SelectChangedEvent>,
    mut open_events: MessageWriter<SelectOpenStateChangedEvent>,
    mut query: Query<(Entity, &mut USelect)>,
) {
    for (entity, mut select) in query.iter_mut() {
        if select.selected_index != select.previous_selected_index {
            if let Some(index) = select.selected_index {
                if let Some(option) = select.options.get(index) {
                    if !option.disabled {
                        changed_events.write(SelectChangedEvent {
                            entity,
                            selected_index: index,
                            value: option.value.clone(),
                            label: option.label.clone(),
                        });
                    }
                }
            }
            select.previous_selected_index = select.selected_index;
        }

        if select.is_open != select.previous_open {
            open_events.write(SelectOpenStateChangedEvent {
                entity,
                is_open: select.is_open,
            });
            select.previous_open = select.is_open;
        }
    }
}

fn spawn_dropdown(commands: &mut Commands, select_entity: Entity, select: &USelect) -> Entity {
    let mut dropdown_entity = None;
    let max_visible = select.max_visible_options.max(1);
    let should_clip = select.options.len() > max_visible;
    let row_height = select.trigger_height.max(1.0);

    commands.entity(select_entity).with_children(|parent| {
        let dropdown = parent
            .spawn((
                UNode {
                    width: UVal::Px(select.width.max(1.0)),
                    height: if should_clip {
                        UVal::Px(row_height * max_visible as f32)
                    } else {
                        UVal::Content
                    },
                    background_color: select.dropdown_background,
                    border_radius: UCornerRadius::all(8.0),
                    ..default()
                },
                UBorder {
                    color: select.border_color,
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                    offset: 0.0,
                },
                UClip { enabled: should_clip },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    ..default()
                },
                SelectDropdown {
                    select: select_entity,
                },
            ))
            .with_children(|list| {
                for (index, option) in select.options.iter().enumerate() {
                    let is_selected = select.selected_index == Some(index);
                    let is_highlighted = select.highlighted_index == Some(index);

                    let base_bg = if option.disabled {
                        Color::NONE
                    } else if is_selected {
                        select.option_selected_color
                    } else if is_highlighted {
                        select.option_hover_color
                    } else {
                        Color::NONE
                    };

                    list.spawn((
                        UNode {
                            width: UVal::Percent(1.0),
                            height: UVal::Px(row_height),
                            padding: select.padding,
                            background_color: base_bg,
                            ..default()
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            justify_content: UJustifyContent::Start,
                            ..default()
                        },
                        Pickable::default(),
                        UInteraction::default(),
                        UInteractionColors {
                            normal: Color::NONE,
                            hovered: if option.disabled {
                                Color::NONE
                            } else {
                                select.option_hover_color
                            },
                            pressed: if option.disabled {
                                Color::NONE
                            } else {
                                select.option_hover_color
                            },
                        },
                        SelectOptionRow {
                            select: select_entity,
                            index,
                        },
                    ))
                    .with_children(|row| {
                        row.spawn((
                            UTextLabel {
                                text: option.label.clone(),
                                font_size: select.font_size,
                                color: if option.disabled {
                                    select.option_disabled_text_color
                                } else {
                                    select.text_color
                                },
                                autosize: false,
                                ..default()
                            },
                            SelectOptionLabel,
                            Pickable::IGNORE,
                        ));
                    });
                }
            })
            .id();

        dropdown_entity = Some(dropdown);
    });

    dropdown_entity.expect("dropdown should be spawned")
}

fn sanitize_select(select: &mut USelect) {
    select.width = select.width.max(1.0);
    select.trigger_height = select.trigger_height.max(1.0);
    select.max_visible_options = select.max_visible_options.max(1);

    if select.options.is_empty() {
        select.selected_index = None;
        select.highlighted_index = None;
        select.is_open = false;
        return;
    }

    if !is_enabled_index(&select.options, select.selected_index) {
        select.selected_index = None;
    }

    if !is_enabled_index(&select.options, select.highlighted_index) {
        select.highlighted_index = None;
    }

    if select.is_open && select.highlighted_index.is_none() {
        select.highlighted_index = select
            .selected_index
            .filter(|idx| is_enabled_index(&select.options, Some(*idx)))
            .or_else(|| first_enabled_index(&select.options));
    }
}

fn selected_option(select: &USelect) -> Option<&USelectOption> {
    select
        .selected_index
        .and_then(|index| select.options.get(index))
        .filter(|option| !option.disabled)
}

fn is_enabled_index(options: &[USelectOption], index: Option<usize>) -> bool {
    index
        .and_then(|idx| options.get(idx))
        .map(|opt| !opt.disabled)
        .unwrap_or(false)
}

fn first_enabled_index(options: &[USelectOption]) -> Option<usize> {
    options.iter().position(|opt| !opt.disabled)
}

fn next_enabled_index(
    options: &[USelectOption],
    current: Option<usize>,
    direction: i32,
) -> Option<usize> {
    if options.is_empty() {
        return None;
    }

    if !options.iter().any(|opt| !opt.disabled) {
        return None;
    }

    let len = options.len() as i32;
    let step = if direction >= 0 { 1 } else { -1 };
    let mut index = match current {
        Some(i) if i < options.len() => i as i32,
        _ if step > 0 => -1,
        _ => 0,
    };

    for _ in 0..options.len() {
        index += step;
        if index < 0 {
            index = len - 1;
        }
        if index >= len {
            index = 0;
        }

        let idx = index as usize;
        if !options[idx].disabled {
            return Some(idx);
        }
    }

    None
}

#[derive(Message)]
pub struct SelectChangedEvent {
    pub entity: Entity,
    pub selected_index: usize,
    pub value: String,
    pub label: String,
}

#[derive(Message)]
pub struct SelectOpenStateChangedEvent {
    pub entity: Entity,
    pub is_open: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_options() -> Vec<USelectOption> {
        vec![
            USelectOption::new("One", "one"),
            USelectOption::new("Two", "two").disabled(),
            USelectOption::new("Three", "three"),
        ]
    }

    #[test]
    fn first_enabled_index_returns_first_enabled() {
        let options = sample_options();
        assert_eq!(first_enabled_index(&options), Some(0));
    }

    #[test]
    fn next_enabled_index_skips_disabled_and_wraps() {
        let options = sample_options();
        assert_eq!(next_enabled_index(&options, Some(0), 1), Some(2));
        assert_eq!(next_enabled_index(&options, Some(2), 1), Some(0));
        assert_eq!(next_enabled_index(&options, Some(0), -1), Some(2));
    }

    #[test]
    fn with_selected_value_matches_existing_option() {
        let select = USelect::new()
            .with_options(sample_options())
            .with_selected_value("three");
        assert_eq!(select.selected_index, Some(2));
    }

    #[test]
    fn enforce_select_invariants_resets_out_of_range_index() {
        let mut select = USelect::new().with_options(sample_options());
        select.selected_index = Some(99);
        sanitize_select(&mut select);
        assert_eq!(select.selected_index, None);
    }

    #[test]
    fn keyboard_navigation_never_targets_disabled_option() {
        let options = sample_options();
        let next = next_enabled_index(&options, Some(0), 1);
        assert_eq!(next, Some(2));
        assert!(!is_enabled_index(&options, Some(1)));
    }
}
