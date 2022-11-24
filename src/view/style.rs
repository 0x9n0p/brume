use crate::view::{size, view};
use crate::view::color;

pub trait Style {
    fn name(&self) -> &'static str where Self: Sized;
    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error>;
}

pub struct Width {
    name: &'static str,
    val: size::Size,
}

impl Width {
    pub fn new(size: size::Size) -> Width {
        Width { name: "width", val: size }
    }

    pub fn max(size: size::Size) -> Width {
        Width { name: "max-width", val: size }
    }
}

impl Style for Width {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val.to_string());
        return Ok(());
    }
}

pub struct Height {
    name: &'static str,
    val: size::Size,
}

impl Height {
    pub fn new(size: size::Size) -> Height {
        Height { name: "height", val: size }
    }

    pub fn max(size: size::Size) -> Height {
        Height { name: "max-height", val: size }
    }
}

impl Style for Height {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val.to_string());
        return Ok(());
    }
}

pub struct Color {
    color: color::Colors,
}

impl Color {
    pub fn new(color: color::Colors) -> Color {
        Color {
            color,
        }
    }
}

impl Style for Color {
    fn name(&self) -> &'static str {
        return "COLOR";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("color", &self.color.to_string());
        return Ok(());
    }
}

pub struct BackgroundColor {
    color: color::Colors,
}

impl BackgroundColor {
    pub fn new(color: color::Colors) -> BackgroundColor {
        BackgroundColor {
            color,
        }
    }
}

impl Style for BackgroundColor {
    fn name(&self) -> &'static str {
        return "BACKGROUND_COLOR";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("background-color", &self.color.to_string());
        return Ok(());
    }
}

pub struct BorderRadius {
    size: size::Size,
}

impl BorderRadius {
    pub fn new(size: size::Size) -> BorderRadius {
        BorderRadius {
            size
        }
    }
}

impl Style for BorderRadius {
    fn name(&self) -> &'static str {
        return "BORDER_RADIUS";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("border-radius", &self.size.to_string());
        return Ok(());
    }
}

// TextTransform specifies how to capitalize an element's text.
pub struct TextTransform {
    val: &'static str,
}

impl TextTransform {
    // No capitalization. The text renders as it is. This is default
    pub fn none() -> TextTransform {
        TextTransform {
            val: "none"
        }
    }

    // Transforms the first character of each word to uppercase
    pub fn capitalize() -> TextTransform {
        TextTransform {
            val: "capitalize"
        }
    }

    // Transforms all characters to uppercase
    pub fn uppercase() -> TextTransform {
        TextTransform {
            val: "uppercase"
        }
    }

    // Transforms all characters to lowercase
    pub fn lowercase() -> TextTransform {
        TextTransform {
            val: "lowercase"
        }
    }

    // Sets this property to its default value
    pub fn initial() -> TextTransform {
        TextTransform {
            val: "initial"
        }
    }

    // Inherits this property from its parent element
    pub fn inherit() -> TextTransform {
        TextTransform {
            val: "inherit"
        }
    }
}

impl Style for TextTransform {
    fn name(&self) -> &'static str {
        return "TEXT_TRANSFORM";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("text-transform", &self.val);
        return Ok(());
    }
}

pub struct Border {
    val: &'static str,
}

impl Border {
    pub fn none() -> Border {
        Border {
            val: "none"
        }
    }

    pub fn dotted() -> Border {
        Border {
            val: "dotted"
        }
    }

    pub fn dashed() -> Border {
        Border {
            val: "dashed"
        }
    }
    pub fn solid() -> Border {
        Border {
            val: "solid"
        }
    }
    pub fn double() -> Border {
        Border {
            val: "double"
        }
    }
    pub fn groove() -> Border {
        Border {
            val: "groove"
        }
    }
    pub fn ridge() -> Border {
        Border {
            val: "ridge"
        }
    }
    pub fn inset() -> Border {
        Border {
            val: "inset"
        }
    }

    pub fn outset() -> Border {
        Border {
            val: "outset"
        }
    }

    pub fn hidden() -> Border {
        Border {
            val: "hidden"
        }
    }
}

impl Style for Border {
    fn name(&self) -> &'static str {
        return "BORDER";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("border", &self.val);
        return Ok(());
    }
}

pub struct Outline {
    val: &'static str,
}

impl Outline {
    pub fn none() -> Outline {
        Outline {
            val: "none"
        }
    }

    pub fn dotted() -> Outline {
        Outline {
            val: "dotted"
        }
    }

    pub fn dashed() -> Outline {
        Outline {
            val: "dashed"
        }
    }
    pub fn solid() -> Outline {
        Outline {
            val: "solid"
        }
    }
    pub fn double() -> Outline {
        Outline {
            val: "double"
        }
    }
    pub fn groove() -> Outline {
        Outline {
            val: "groove"
        }
    }
    pub fn ridge() -> Outline {
        Outline {
            val: "ridge"
        }
    }
    pub fn inset() -> Outline {
        Outline {
            val: "inset"
        }
    }

    pub fn outset() -> Outline {
        Outline {
            val: "outset"
        }
    }

    pub fn hidden() -> Outline {
        Outline {
            val: "hidden"
        }
    }
}

impl Style for Outline {
    fn name(&self) -> &'static str {
        return "OUTLINE";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("outline", &self.val);
        return Ok(());
    }
}

pub struct Padding {
    name: &'static str,
    val: size::Size,
}

impl Padding {
    pub fn new(size: size::Size) -> Padding {
        Padding { name: "padding", val: size }
    }

    pub fn block(size: size::Size) -> Padding {
        Padding { name: "padding-block", val: size }
    }
}

impl Style for Padding {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val.to_string());
        return Ok(());
    }
}

pub struct FontWeight {
    name: &'static str,
    val: &'static str,
}

impl FontWeight {
    pub fn new(size: &'static str) -> FontWeight {
        FontWeight { name: "font-weight", val: size }
    }

    pub fn normal() -> FontWeight {
        FontWeight { name: "font-weight", val: "normal" }
    }

    pub fn bold() -> FontWeight {
        FontWeight { name: "font-weight", val: "bold" }
    }

    pub fn bolder() -> FontWeight {
        FontWeight { name: "font-weight", val: "bolder" }
    }

    pub fn lighter() -> FontWeight {
        FontWeight { name: "font-weight", val: "lighter" }
    }
}

impl Style for FontWeight {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val);
        return Ok(());
    }
}

pub struct LetterSpacing {
    name: &'static str,
    val: String,
}

impl LetterSpacing {
    pub fn new(size: size::Size) -> LetterSpacing {
        LetterSpacing { name: "letter-spacing", val: size.to_string() }
    }

    pub fn normal() -> LetterSpacing {
        LetterSpacing { name: "letter-spacing", val: "normal".to_string() }
    }
}

impl Style for LetterSpacing {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val);
        return Ok(());
    }
}

pub struct Cursor {
    name: &'static str,
    val: &'static str,
}

impl Cursor {
    pub fn pointer() -> Cursor {
        Cursor { name: "cursor", val: "pointer" }
    }
}

impl Style for Cursor {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val);
        return Ok(());
    }
}

pub(crate) struct ContainerDirection {
    name: &'static str,
    val: &'static str,
}

impl ContainerDirection {
    pub fn column() -> Cursor {
        Cursor { name: "flex-direction", val: "column" }
    }

    pub fn row() -> Cursor {
        Cursor { name: "flex-direction", val: "row" }
    }
}

impl Style for ContainerDirection {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val);
        return Ok(());
    }
}

pub struct Justify {
    name: &'static str,
    val: &'static str,
}

impl Justify {
    // Items are positioned in the center of the container
    pub fn center() -> Justify {
        Justify { name: "justify-content", val: "center" }
    }

    // Default value. Items are positioned at the beginning of the container
    pub fn start() -> Justify {
        Justify { name: "justify-content", val: "flex-start" }
    }

    // Items are positioned at the end of the container
    pub fn end() -> Justify {
        Justify { name: "justify-content", val: "flex-end" }
    }

    // Items will have space between them
    pub fn space_between() -> Justify {
        Justify { name: "justify-content", val: "space-between" }
    }

    // Items will have space before, between, and after them
    pub fn space_around() -> Justify {
        Justify { name: "justify-content", val: "space-around" }
    }

    // Items will have equal space around them
    pub fn space_evenly() -> Justify {
        Justify { name: "justify-content", val: "space-evenly" }
    }
}

impl Style for Justify {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val);
        return Ok(());
    }
}

pub struct Align {
    name: &'static str,
    val: &'static str,
}

impl Align {
    pub fn center() -> Align {
        Align { name: "align-items", val: "center" }
    }

    pub fn start() -> Align {
        Align { name: "align-items", val: "flex-start" }
    }

    pub fn end() -> Align {
        Align { name: "align-items", val: "flex-end" }
    }

    pub fn stretch() -> Align {
        Align { name: "align-items", val: "stretch" }
    }

    pub fn baseline() -> Align {
        Align { name: "align-items", val: "baseline" }
    }
}

impl Style for Align {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val);
        return Ok(());
    }
}