use std::borrow::BorrowMut;
use crate::view::{font, size, view};
use crate::view::color;

pub trait Style {
    fn name(&self) -> &'static str where Self: Sized;
    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error>;
}

pub struct Custom {
    name: &'static str,
    val: &'static str,
}

impl Custom {
    pub fn new(name: &'static str, val: &'static str) -> Custom {
        Custom { name, val }
    }
}

impl Style for Custom {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val.to_string());
        return Ok(());
    }
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

pub struct Background {
    color: color::Colors,
}

impl Background {
    pub fn color(color: color::Colors) -> Background {
        Background {
            color,
        }
    }
}

impl Style for Background {
    fn name(&self) -> &'static str {
        return "BACKGROUND";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("background", &self.color.to_string());
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

pub struct TextDecoration {
    val: &'static str,
}

impl TextDecoration {
    pub fn new(val: &'static str) -> TextDecoration {
        TextDecoration {
            val,
        }
    }

    pub fn none() -> TextDecoration {
        TextDecoration {
            val: "none",
        }
    }
}

impl Style for TextDecoration {
    fn name(&self) -> &'static str {
        return "TEXT_DECORATION";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("text-decoration", &self.val);
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

pub struct BorderStyle {
    name: &'static str,
    val: &'static str,
}

impl BorderStyle {
    pub fn none() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "none",
        }
    }

    pub fn dotted() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "dotted",
        }
    }

    pub fn dashed() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "dashed",
        }
    }
    pub fn solid() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "solid",
        }
    }
    pub fn double() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "double",
        }
    }
    pub fn groove() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "groove",
        }
    }
    pub fn ridge() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "ridge",
        }
    }
    pub fn inset() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "inset",
        }
    }

    pub fn outset() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "outset",
        }
    }

    pub fn hidden() -> BorderStyle {
        BorderStyle {
            name: "border-style",
            val: "hidden",
        }
    }
}

impl Style for BorderStyle {
    fn name(&self) -> &'static str {
        return "BORDER_STYLE";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val);
        return Ok(());
    }
}

pub struct BorderWidth {
    name: &'static str,
    size: size::Size,
}

impl BorderWidth {
    pub fn new(size: size::Size) -> BorderWidth {
        BorderWidth {
            name: "border-width",
            size,
        }
    }
}

impl Style for BorderWidth {
    fn name(&self) -> &'static str {
        return "BORDER_WIDTH";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("border-width", &self.size.to_string());
        return Ok(());
    }
}

pub struct BorderColor {
    name: &'static str,
    color: color::Colors,
}

impl BorderColor {
    pub fn new(color: color::Colors) -> BorderColor {
        BorderColor {
            name: "border-color",
            color,
        }
    }
}

impl Style for BorderColor {
    fn name(&self) -> &'static str {
        return "BORDER_COLOR";
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property("border-color", &self.color.to_string());
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

    pub fn inline(size: size::Size) -> Padding {
        Padding { name: "padding-inline", val: size }
    }

    pub fn left(size: size::Size) -> Padding {
        Padding { name: "padding-left", val: size }
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

pub struct FontSize {
    name: &'static str,
    val: String,
}

impl FontSize {
    pub fn new(size: size::Size) -> FontSize {
        FontSize { name: "font-weight", val: size.to_string() }
    }
}

impl Style for FontSize {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn build(&self, element: &web_sys::HtmlElement) -> Result<(), view::Error> {
        element.style().set_property(self.name, &self.val);
        return Ok(());
    }
}

pub struct FontFamily {
    name: &'static str,
    val: String,
}

impl FontFamily {
    pub fn new(font: font::Font) -> FontFamily {
        FontFamily { name: "font-family", val: font.to_string() }
    }
}

impl Style for FontFamily {
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
    pub fn default() -> Cursor {
        Cursor { name: "cursor", val: "default" }
    }

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