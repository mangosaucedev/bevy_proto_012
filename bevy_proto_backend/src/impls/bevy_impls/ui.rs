use bevy::app::App;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{BackgroundColor, Button, Label};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::ui::{
    AlignContent, AlignItems, AlignSelf, BorderColor, CalculatedClip, ContentSize, Direction,
    Display, FlexDirection, FlexWrap, FocusPolicy, GridAutoFlow, GridPlacement, GridTrack,
    Interaction, JustifyContent, JustifyItems, JustifySelf, Node, Overflow, PositionType,
    RelativeCursorPosition, RepeatedGridTrack, Style, UiImage, UiRect, Val, ZIndex,
};

use crate::impls::macros::{from_to, from_to_default, register_schematic};
use crate::proto::{ProtoAsset, ProtoColor};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(
        app,
        BackgroundColor,
        Button,
        CalculatedClip,
        ContentSize,
        FocusPolicy,
        Interaction,
        Label,
        Node,
        RelativeCursorPosition,
        Style,
        UiImage,
    );

    // Can be removed if https://github.com/bevyengine/bevy/pull/5781 is ever merged
    app.register_type::<AlignContentInput>()
        .register_type::<AlignItemsInput>()
        .register_type::<AlignSelfInput>()
        .register_type::<DirectionInput>()
        .register_type::<DisplayInput>()
        .register_type::<FlexDirectionInput>()
        .register_type::<FlexWrapInput>()
        .register_type::<JustifyContentInput>()
        .register_type::<OverflowInput>()
        .register_type::<PositionTypeInput>()
        .register_type::<UiRectInput>()
        .register_type::<ValInput>();
}

impl_external_schematic! {
    #[schematic(from = BackgroundColorInput)]
    struct BackgroundColor();
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct BackgroundColorInput(pub ProtoColor);
    from_to_default! {
        BackgroundColor,
        BackgroundColorInput,
        |value: Input| Self(value.0.into())
    }
}

impl_external_schematic! {
    #[schematic(from = BorderColorInput)]
    struct BorderColor();
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct BorderColorInput(pub ProtoColor);
    from_to_default! {
        BorderColor,
        BorderColorInput,
        |value: Input| Self(value.0.into())
    }
}

impl_external_schematic! {
    #[schematic(from = ButtonInput)]
    struct Button;
    // ---
    #[derive(Reflect)]
    pub struct ButtonInput;
    from_to_default!(
        Button,
        ButtonInput,
        |_: Input| Self
    );
}

impl_external_schematic! {
    #[schematic(from = CalculatedClipInput)]
    struct CalculatedClip {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct CalculatedClipInput {
        pub clip: Rect,
    }
    from_to_default! {
        CalculatedClip,
        CalculatedClipInput,
        |value: Input| Self {
            clip: value.clip,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = ContentSizeInput)]
    struct ContentSize {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct ContentSizeInput {
        pub size: Vec2,
        pub preserve_aspect_ratio: bool,
    }
    from_to_default! {
        ContentSize,
        ContentSizeInput,
        |value: Input| Self {
            size: value.size,
            preserve_aspect_ratio: value.preserve_aspect_ratio,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = FocusPolicyInput)]
    enum FocusPolicy {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum FocusPolicyInput {
        Block,
        Pass,
    }
    from_to_default! {
        FocusPolicy,
        FocusPolicyInput,
        |value: Input| match value {
            Input::Block => Self::Block,
            Input::Pass => Self::Pass,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = InteractionInput)]
    enum Interaction {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum InteractionInput {
        Pressed,
        Hovered,
        None,
    }
    from_to_default! {
        Interaction,
        InteractionInput,
        |value: Input| match value {
            Input::Pressed => Self::Pressed,
            Input::Hovered => Self::Hovered,
            Input::None => Self::None,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = LabelInput)]
    struct Label;
    // ---
    #[derive(Reflect)]
    pub struct LabelInput;
    impl From<LabelInput> for Label {
        fn from(_: LabelInput) -> Self {
            Self
        }
    }
}

impl_external_schematic! {
    #[schematic(from = NodeInput)]
    struct Node {}
    // ---
    #[derive(Reflect)]
    pub struct NodeInput;
    from_to!(
        Node,
        NodeInput,
        |_: Input| Self::default()
    );
    impl Default for NodeInput {
        fn default() -> Self {
            Self
        }
    }
}

impl_external_schematic! {
    #[schematic(from = RelativeCursorPositionInput)]
    struct RelativeCursorPosition {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct RelativeCursorPositionInput {
        pub normalized: Option<Vec2>,
    }
    from_to_default! {
        RelativeCursorPosition,
        RelativeCursorPositionInput,
        |value: Input| Self {
            normalized: value.normalized,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = StyleInput)]
    struct Style {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct StyleInput {
        pub display: DisplayInput,
        pub position_type: PositionTypeInput,
        pub overflow: OverflowInput,
        pub direction: DirectionInput,
        pub flex_direction: FlexDirectionInput,
        pub flex_wrap: FlexWrapInput,
        pub align_items: AlignItemsInput,
        pub align_self: AlignSelfInput,
        pub align_content: AlignContentInput,
        pub justify_content: JustifyContentInput,
        pub justify_self: JustifySelf,
        pub justify_items: JustifyItems,
        pub margin: UiRectInput,
        pub padding: UiRectInput,
        pub border: UiRectInput,
        pub flex_grow: f32,
        pub flex_shrink: f32,
        pub flex_basis: ValInput,
        pub aspect_ratio: Option<f32>,
        pub left: Val,
        pub right: Val,
        pub top: Val,
        pub bottom: Val,
        pub width: Val,
        pub min_width: Val,
        pub max_width: Val,
        pub height: Val,
        pub min_height: Val,
        pub max_height: Val,
        pub row_gap: Val,
        pub column_gap: Val,
        pub grid_auto_flow: GridAutoFlow,
        pub grid_template_rows: Vec<RepeatedGridTrack>,
        pub grid_template_columns: Vec<RepeatedGridTrack>,
        pub grid_auto_rows: Vec<GridTrack>,
        pub grid_auto_columns: Vec<GridTrack>,
        pub grid_row: GridPlacement,
        pub grid_column: GridPlacement,
    }
    from_to_default! {
        Style,
        StyleInput,
        |value: Input| Self {
            display: value.display.into(),
            position_type: value.position_type.into(),
            overflow: value.overflow.into(),
            direction: value.direction.into(),
            flex_direction: value.flex_direction.into(),
            flex_wrap: value.flex_wrap.into(),
            align_items: value.align_items.into(),
            align_self: value.align_self.into(),
            align_content: value.align_content.into(),
            justify_content: value.justify_content.into(),
            justify_self: value.justify_self.into(),
            justify_items: value.justify_items.into(),
            margin: value.margin.into(),
            padding: value.padding.into(),
            border: value.border.into(),
            flex_grow: value.flex_grow,
            flex_shrink: value.flex_shrink,
            flex_basis: value.flex_basis.into(),
            aspect_ratio: value.aspect_ratio,
            left: value.left.into(),
            right: value.right.into(),
            top: value.top.into(),
            bottom: value.bottom.into(),
            width: value.width.into(),
            min_width: value.min_width.into(),
            max_width: value.max_width.into(),
            height: value.height.into(),
            min_height: value.min_height.into(),
            max_height: value.max_height.into(),
            row_gap: value.row_gap.into(),
            column_gap: value.column_gap.into(),
            grid_auto_flow: value.grid_auto_flow.into(),
            grid_template_rows: value.grid_template_rows.into(),
            grid_template_columns: value.grid_template_columns.into(),
            grid_auto_rows: value.grid_auto_rows.into(),
            grid_auto_columns: value.grid_auto_columns.into(),
            grid_row: value.grid_row.into(),
            grid_column: value.grid_column.into(),
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum AlignContentInput {
        Start,
        End,
        FlexStart,
        FlexEnd,
        Center,
        Stretch,
        SpaceBetween,
        SpaceEvenly,
        SpaceAround,
    }
    from_to_default! {
        AlignContent,
        AlignContentInput,
        |value: Input| match value {
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::Stretch => Self::Stretch,
            Input::SpaceBetween => Self::SpaceBetween,
            Input::SpaceEvenly => Self::SpaceEvenly,
            Input::SpaceAround => Self::SpaceAround,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum AlignItemsInput {
        Start,
        End,
        FlexStart,
        FlexEnd,
        Center,
        Baseline,
        Stretch,
    }
    from_to_default! {
        AlignItems,
        AlignItemsInput,
        |value: Input| match value {
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::Baseline => Self::Baseline,
            Input::Stretch => Self::Stretch,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum AlignSelfInput {
        Auto,
        Start,
        End,
        FlexStart,
        FlexEnd,
        Center,
        Baseline,
        Stretch,
    }
    from_to_default! {
        AlignSelf,
        AlignSelfInput,
        |value: Input| match value {
            Input::Auto => Self::Auto,
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::Baseline => Self::Baseline,
            Input::Stretch => Self::Stretch,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum DirectionInput {
        Inherit,
        LeftToRight,
        RightToLeft,
    }
    from_to_default! {
        Direction,
        DirectionInput,
        |value: Input| match value {
            Input::Inherit => Self::Inherit,
            Input::LeftToRight => Self::LeftToRight,
            Input::RightToLeft => Self::RightToLeft,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum DisplayInput {
        None,
        Flex,
    }
    from_to_default! {
        Display,
        DisplayInput,
        |value: Input| match value {
            Input::None => Self::None,
            Input::Flex => Self::Flex,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum FlexWrapInput {
        NoWrap,
        Wrap,
        WrapReverse,
    }
    from_to_default! {
        FlexWrap,
        FlexWrapInput,
        |value: Input| match value {
            Input::NoWrap => Self::NoWrap,
            Input::Wrap => Self::Wrap,
            Input::WrapReverse => Self::WrapReverse,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum FlexDirectionInput {
        Row,
        Column,
        RowReverse,
        ColumnReverse,
    }
    from_to_default! {
        FlexDirection,
        FlexDirectionInput,
        |value: Input| match value {
            Input::Row => Self::Row,
            Input::Column => Self::Column,
            Input::RowReverse => Self::RowReverse,
            Input::ColumnReverse => Self::ColumnReverse,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum JustifyContentInput {
        Start,
        End,
        FlexStart,
        FlexEnd,
        Center,
        SpaceBetween,
        SpaceAround,
        SpaceEvenly,
    }
    from_to_default! {
        JustifyContent,
        JustifyContentInput,
        |value: Input| match value {
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::SpaceBetween => Self::SpaceBetween,
            Input::SpaceEvenly => Self::SpaceEvenly,
            Input::SpaceAround => Self::SpaceAround,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum OverflowInput {
        Visible,
        Hidden,
    }
    from_to_default! {
        Overflow,
        OverflowInput,
        |value: Input| match value {
            Input::Visible => Self::Visible,
            Input::Hidden => Self::Hidden,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum PositionTypeInput {
        Relative,
        Absolute,
    }
    from_to_default! {
        PositionType,
        PositionTypeInput,
        |value: Input| match value {
            Input::Relative => Self::Relative,
            Input::Absolute => Self::Absolute,
        }
    }


    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct UiRectInput {
        pub left: ValInput,
        pub right: ValInput,
        pub top: ValInput,
        pub bottom: ValInput,
    }
    from_to_default! {
        UiRect,
        UiRectInput,
        |value: Input| Self {
            left: value.left.into(),
            right: value.right.into(),
            top: value.top.into(),
            bottom: value.bottom.into(),
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum ValInput {
        Undefined,
        Auto,
        Px(f32),
        Percent(f32),
    }
    from_to_default! {
        Val,
        ValInput,
        |value: Input| match value {
            Input::Undefined => Self::Undefined,
            Input::Auto => Self::Auto,
            Input::Px(value) => Self::Px(value),
            Input::Percent(value) => Self::Percent(value),
        }
    }
}

impl_external_schematic! {
    #[schematic(input(vis = pub))]
    pub struct UiImage {
        #[schematic(asset(lazy))]
        pub texture: Handle<Image>,
        #[reflect(default)]
        pub flip_x: bool,
        #[reflect(default)]
        pub flip_y: bool,
    }

    impl Default for UiImageInput {
        fn default() -> Self {
            let base = UiImage::default();
            Self {
                texture: ProtoAsset::HandleId(base.texture.id()),
                flip_x: base.flip_x,
                flip_y: base.flip_y,
            }
        }
    }
}

impl_external_schematic! {
    #[schematic(from = ZIndexInput)]
    enum ZIndex {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum ZIndexInput {
        Local(i32),
        Global(i32),
    }
    from_to_default! {
        ZIndex,
        ZIndexInput,
        |value: Input| match value {
            Input::Local(z) => Self::Local(z),
            Input::Global(z) => Self::Global(z),
        }
    }
}
