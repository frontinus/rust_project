use crate::custom_widget::screenshot_image::UPDATE_SCREENSHOT;
use crate::custom_widget::{ResizableBox, UPDATE_ORIGIN, verify_exists_dir};
use druid::kurbo::common::FloatExt;
use druid::piet::ImageFormat;
use druid::widget::Image;
use druid::{
    commands, BoxConstraints, Color, Data, Env, Event, EventCtx, ImageBuf, InternalEvent,
    LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, Selector, Size, Target, UnitPoint,
    UpdateCtx, Vec2, Widget, WidgetExt, WidgetId, WidgetPod,
};
use image::imageops::FilterType;
use image::io::Reader;
use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use image::{ImageFormat as imgFormat, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::sync::Arc;
use crate::BASE_PATH_SCREENSHOT;

pub enum OverImages {
    Circles,
    Triangle,
    Arrow,
    Highlighter,
    Remove,
    Text,
}
pub const UPDATE_BACK_IMG: Selector<Arc<DynamicImage>> = Selector::new("Update the back image");
pub const UPDATE_COLOR: Selector<(Option<Color>, Option<f64>)> =
    Selector::new("Update the over-img color");
pub const SHOW_OVER_IMG: Selector<(OverImages, Option<String>)> =
    Selector::new("Tell the ZStack to show the over_img, params: over_img path");
pub const SAVE_OVER_IMG: Selector<(Box<str>, Box<str>, image::ImageFormat)> = Selector::new("Tell the ZStack to save the modified screenshot, params: (Screenshot original img's path, Folder Path Where To Save, New File Name, Image Format)");
pub const CREATE_ZSTACK: Selector<Vec<&'static str>> = Selector::new("Initialized the over-images");
/// A container that stacks its children on top of each other.
///
/// The container has a baselayer which has the lowest z-index and determines the size of the
/// container.
pub struct CustomZStack<T> {
    layers: Vec<ZChild<T>>,
    back_img: Option<DynamicImage>,
    back_img_origin: Option<Point>,
    screenshot_id: WidgetId,
    color: (Option<Color>, f64),
    over_images: Option<Vec<DynamicImage>>,
    showing_over_img: Option<usize>,
    text_field: Option<String>,
}

struct ZChild<T> {
    child: WidgetPod<T, Box<dyn Widget<T>>>,
    relative_size: Vec2,
    absolute_size: Vec2,
    position: UnitPoint,
    offset: Vec2,
}

impl<T: Data> CustomZStack<T> {
    /// Creates a new ZStack with a background Image.
    ///
    /// The Image is used by the ZStack to determine its own size.
    pub fn new(base_layer: impl Widget<T> + 'static, screenshot_id: WidgetId) -> Self {
        Self {
            layers: vec![ZChild {
                child: WidgetPod::new(base_layer.boxed()),
                relative_size: Vec2::new(1.0, 1.0),
                absolute_size: Vec2::ZERO,
                position: UnitPoint::CENTER,
                offset: Vec2::ZERO,
            }],
            back_img: None,
            back_img_origin: None,
            screenshot_id,
            color: (None, 100.),
            over_images: None,
            showing_over_img: None,
            text_field: None,
        }
    }

    fn rm_over_img(&mut self) {
        while self.layers.len() > 1 {
            self.rm_child();
        }
        self.showing_over_img = None;
        self.back_img_origin = None;
    }

    /// Builder-style method to add a new child to the Z-Stack.
    ///
    /// The child is added directly above the base layer.
    ///
    /// `relative_size` is the space the child is allowed to take up relative to its parent. The
    ///                 values are between 0 and 1.
    /// `absolute_size` is a fixed amount of pixels added to `relative_size`.
    ///
    /// `position`      is the alignment of the child inside the remaining space of its parent.
    ///
    /// `offset`        is a fixed amount of pixels added to `position`.
    fn with_child(
        self: &mut Self,
        child: impl Widget<T> + 'static,
        relative_size: Vec2,
        absolute_size: Vec2,
        position: UnitPoint,
        offset: Vec2,
    ) -> &mut Self {
        if self.layers.len() as i32 - 1 < 0 {
            self.layers = vec![ZChild {
                child: WidgetPod::new(child.boxed()),
                relative_size: Vec2::new(1.0, 1.0),
                absolute_size: Vec2::ZERO,
                position: UnitPoint::CENTER,
                offset: Vec2::ZERO,
            }]
        } else {
            let next_index = self.layers.len() - 1;
            self.layers.insert(
                next_index,
                ZChild {
                    child: WidgetPod::new(child.boxed()),
                    relative_size,
                    absolute_size,
                    position,
                    offset,
                },
            );
        }
        self
    }

    fn rm_child(self: &mut Self) -> ZChild<T> {
        self.layers.remove(0)
    }

    pub fn show_over_img(self: &mut Self, over_img_index: usize, id: WidgetId) {
        if self.showing_over_img.is_none() {
            let mut image = None;
            if image == None {} // for the warning

            if self.text_field != None && over_img_index == 4 {
                let image_modified = text_to_image(self.text_field.as_mut().unwrap().as_str(), self.color.0);
                let over_images_cloned = self.over_images.as_mut().unwrap();

                if over_images_cloned.len() > over_img_index {
                    over_images_cloned[over_img_index] = image_modified.clone();
                } else {
                    over_images_cloned.push(image_modified.clone());
                }

                image = Some(image_modified);
            } else {
                image = Some(
                    self.over_images
                        .as_mut()
                        .unwrap()
                        .get_mut(over_img_index)
                        .unwrap()
                        .clone(),
                );
            }

            let img = image.unwrap();

            let over_image = ResizableBox::new(
                Image::new(ImageBuf::from_raw(
                    Arc::<[u8]>::from(img.as_bytes()),
                    ImageFormat::RgbaSeparate,
                    img.width() as usize,
                    img.height() as usize,
                )),
                id
            )
            .height(50.)
            .width(50.);
            self.with_child(
                over_image,
                Vec2::new(1., 1.),
                Vec2::ZERO,
                UnitPoint::CENTER,
                Vec2::new(5., 5.),
            )
            .showing_over_img = Some(over_img_index);
        } else {
            self.rm_over_img();
        }
    }

    pub fn save_new_img(
        self: &mut Self,
        new_img_path: &String,
        img_format: imgFormat,
    ) -> Option<DynamicImage> {
        if self.layers.len() > 1 && self.showing_over_img.is_some() {
            let back_img = self.back_img.as_mut().unwrap();
            let back_img_resolution = Size::new(back_img.width() as f64, back_img.height() as f64);
            let mut back_img_rect: Rect = self.layers.get(1).unwrap().child.layout_rect();
            let scale_factor_x = (back_img_resolution.width / back_img_rect.x1).expand();
            let scale_factor_y = (back_img_resolution.height / back_img_rect.y1).expand();

            back_img_rect.x0 = (back_img_rect.x0).floor();
            back_img_rect.y0 = (back_img_rect.y0).floor();
            back_img_rect.x1 = (back_img_rect.x1 * scale_factor_x).expand();
            back_img_rect.y1 = (back_img_rect.y1 * scale_factor_y).expand();
            let back_img = back_img.resize(
                back_img_rect.width() as u32,
                back_img_rect.height() as u32,
                FilterType::Lanczos3,
            );

            let mut over_img_rect: Rect = self.layers.get(0).unwrap().child.layout_rect();
            over_img_rect.x0 = (over_img_rect.x0 * scale_factor_x).floor();
            over_img_rect.y0 = (over_img_rect.y0 * scale_factor_y).floor();
            over_img_rect.x1 = (over_img_rect.x1 * scale_factor_x).expand();
            over_img_rect.y1 = (over_img_rect.y1 * scale_factor_y).expand();
            let over_img = self.over_images.as_mut().unwrap()[self.showing_over_img.unwrap()]
                .resize(
                    over_img_rect.width() as u32,
                    over_img_rect.height() as u32,
                    FilterType::Nearest,
                );

            let mut out = back_img;
            let mut i2: u32 = 0;
            let mut j2: u32 = 0;
            for j1 in over_img_rect.y0 as u32..(over_img_rect.y1) as u32 {
                for i1 in over_img_rect.x0 as u32..(over_img_rect.x1) as u32 {
                    if over_img.in_bounds(i2, j2) {
                        let over_px = over_img.get_pixel(i2, j2);
                        if out.in_bounds(i1, j1) && over_px.channels()[3] == u8::MAX {
                            out.put_pixel(i1, j1, over_px);
                        } else if out.in_bounds(i1, j1) && over_px.channels()[3] != 0 {
                            let mut new_px = out.get_pixel(i1, j1);
                            new_px.blend(&over_px);
                            out.put_pixel(i1, j1, new_px);
                        }
                    }
                    i2 += 1;
                }
                i2 = 0;
                j2 += 1;
            }

            self.rm_over_img();

            out.save_with_format(new_img_path, img_format).unwrap();
            self.back_img = Some(out.clone());
            Some(out)
        } else {
            None
        }
    }
}

fn calculate_text_width(font: Font, scale: Scale, text: &str) -> u32 {
    let mut width = 0;

    for c in text.chars() {
        let glyph = font.glyph(c).scaled(scale);
        width += glyph.h_metrics().advance_width.ceil() as u32;
    }

    width
}

fn text_to_image(text: &str, color: Option<Color>) -> DynamicImage {
    let font_data: &[u8] = include_bytes!("../images/icons/DejaVuSans.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();

    let scale = Scale::uniform(30.0);

    let width = calculate_text_width(font.clone(), scale, text);
    let height = 25;
    let mut image = DynamicImage::new_rgba8(width, height);

    let default_color = Color::rgba8(255, 255, 255, 255);
    let colors = color.unwrap_or(default_color).as_rgba8();
    draw_text_mut(
        &mut image,
        Rgba([colors.0, colors.1, colors.2, colors.3]),
        0,
        0,
        scale,
        &font,
        text,
    );

    image
}

impl<T: Data> Widget<T> for CustomZStack<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) => {
                if cmd.is(commands::COPY) {
                    if let Some(image) = self.back_img.clone() {
                        let image_buffer = image.into_rgba8();
                        let mut clipboard = clippers::Clipboard::get();
                        clipboard.write_image(image_buffer.width(), image_buffer.height(), image_buffer.as_raw()).unwrap();
                    }
                }
                if cmd.is(SHOW_OVER_IMG) {
                    let (path, text_field) = cmd.get_unchecked(SHOW_OVER_IMG);
                    match path {
                        OverImages::Circles => {
                            self.show_over_img(0, ctx.widget_id());
                        }
                        OverImages::Triangle => {
                            self.show_over_img(1, ctx.widget_id());
                        }
                        OverImages::Arrow => {
                            self.show_over_img(2, ctx.widget_id());
                        }
                        OverImages::Highlighter => {
                            self.show_over_img(3, ctx.widget_id());
                        }
                        OverImages::Text => {
                            if let Some(text) = text_field {
                                if text.len() > 0 {
                                    self.text_field = Some((*text).clone());
                                    self.show_over_img(4, ctx.widget_id());
                                }
                            }
                        }
                        OverImages::Remove => {
                            if self.showing_over_img.is_some() {
                                self.show_over_img(0, ctx.widget_id());
                            }
                        }
                    }
                } else if cmd.is(SAVE_OVER_IMG) {
                    let (path, file_name, file_format) = cmd.get_unchecked(SAVE_OVER_IMG);
                    let new_img_path = format!(
                        "{}{}.{}",
                        path,
                        file_name,
                        file_format.extensions_str().first().unwrap()
                    );

                    // it verify if exists the dir before saving the image
                    verify_exists_dir(BASE_PATH_SCREENSHOT);

                    let new_img = self.save_new_img(&new_img_path, *file_format);
                    if new_img.is_some() {
                        ctx.submit_command(
                            UPDATE_SCREENSHOT
                                .with(Arc::new(new_img.unwrap()))
                                .to(Target::Widget(self.screenshot_id)),
                        );
                    }
                } else if cmd.is(UPDATE_ORIGIN) {
                    let new_origin = cmd.get_unchecked(UPDATE_ORIGIN);
                    self.back_img_origin = Some(*new_origin);
                } else if cmd.is(UPDATE_COLOR) {
                    let (color, alpha) = cmd.get_unchecked(UPDATE_COLOR);
                    if color.is_some() {
                        self.color.0 = Some(color.unwrap());
                    }
                    if alpha.is_some() {
                        self.color.1 = alpha.unwrap();
                    }

                    //update images color
                    if self.over_images.is_some() {
                        self.over_images
                            .as_mut()
                            .unwrap()
                            .iter_mut()
                            .for_each(|img| {
                                let (color, alpha) = self.color;
                                for j in 0..img.height() {
                                    for i in 0..img.width() {
                                        let mut cur_px = img.get_pixel(i, j);
                                        let ch = cur_px.channels_mut();
                                        if ch[3] > 0 {
                                            if color.is_some() {
                                                let color = color.unwrap().as_rgba8();
                                                ch[0] = color.0;
                                                ch[1] = color.1;
                                                ch[2] = color.2;
                                            }
                                            ch[3] = ((alpha / 100.) * u8::MAX as f64) as u8;
                                            img.put_pixel(i, j, cur_px);
                                        }
                                    }
                                }
                            });
                    }
                    if self.showing_over_img.is_some() {
                        self.rm_over_img();
                    }
                } else if cmd.is(CREATE_ZSTACK) {
                    let paths = cmd.get_unchecked(CREATE_ZSTACK);
                    let mut over_images = Vec::<DynamicImage>::new();
                    paths.iter().for_each(|path| {
                        let over_img = Reader::open(path)
                            .expect("Can't open the screenshot!")
                            .decode()
                            .expect("Can't decode the screenshot");
                        over_images.push(over_img);
                    });
                    self.over_images = Some(over_images);
                } else if cmd.is(UPDATE_BACK_IMG) {
                    let back_img = cmd.get_unchecked(UPDATE_BACK_IMG);
                    self.back_img = Some((**back_img).clone());
                }
                ctx.children_changed();
                ctx.request_paint();
            }
            _ => {
                let mut previous_hot = false;
                for layer in self.layers.iter_mut() {
                    if event.is_pointer_event() && previous_hot {
                        if layer.child.is_active() {
                            ctx.set_handled();
                            layer.child.event(ctx, event, data, env);
                        } else {
                            layer.child.event(
                                ctx,
                                &Event::Internal(InternalEvent::MouseLeave),
                                data,
                                env,
                            );
                        }
                    } else {
                        layer.child.event(ctx, event, data, env);
                    }

                    previous_hot |= layer.child.is_hot();
                }
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        let mut previous_hot = false;
        for layer in self.layers.iter_mut() {
            let inner_event = event.ignore_hot(previous_hot);
            layer.child.lifecycle(ctx, &inner_event, data, env);
            previous_hot |= layer.child.is_hot();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &T, data: &T, env: &Env) {
        for layer in self.layers.iter_mut().rev() {
            layer.child.update(ctx, data, env);
        }
        ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        //Layout base layer

        let base_layer = self.layers.last_mut().unwrap();
        let base_size = base_layer.child.layout(ctx, bc, data, env);

        //Layout other layers
        let other_layers = self.layers.len() - 1;

        for layer in self.layers.iter_mut().take(other_layers) {
            let max_size = layer.resolve_max_size(base_size);
            layer
                .child
                .layout(ctx, &BoxConstraints::new(Size::ZERO, max_size), data, env);
        }

        //Set origin for all Layers and calculate paint insets
        let mut paint_rect = Rect::ZERO;

        let len = self.layers.len();
        for (i, layer) in self.layers.iter_mut().enumerate() {
            let remaining = base_size - layer.child.layout_rect().size();
            let mut origin = layer.resolve_point(remaining);
            if self.back_img_origin.is_some() && i == 0 && len == 2 {
                let dif_point = self.back_img_origin.unwrap();
                origin.x += dif_point.x;
                origin.y += dif_point.y;
            }

            layer.child.set_origin(ctx, origin);

            paint_rect = paint_rect.union(layer.child.paint_rect());
        }

        ctx.set_paint_insets(paint_rect - base_size.to_rect());
        ctx.set_baseline_offset(self.layers.last().unwrap().child.baseline_offset());

        base_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        //Painters algorithm (Painting back to front)
        for layer in self.layers.iter_mut().rev() {
            layer.child.paint(ctx, data, env);
        }
    }
}

impl<T: Data> ZChild<T> {
    fn resolve_max_size(&self, availible: Size) -> Size {
        self.absolute_size.to_size()
            + Size::new(
                availible.width * self.relative_size.x,
                availible.height * self.relative_size.y,
            )
    }

    fn resolve_point(&self, remaining_space: Size) -> Point {
        (self.position.resolve(remaining_space.to_rect()).to_vec2() + self.offset).to_point()
    }
}
