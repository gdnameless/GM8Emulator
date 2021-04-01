// This file was auto-generated based on a function table dump

#![allow(unused_macros)]

use crate::{
    action, asset,
    game::{
        draw, external, gm_save::GMSave, model, particle, pathfinding, replay, string::RCStr, surface::Surface,
        transition::UserTransition, view::View, Game, GetAsset, PlayType, SceneChange, Version,
    },
    gml::{
        self,
        datetime::{self, DateTime},
        ds, file, mappings, network, Context, Value,
    },
    handleman::HandleManager,
    instance::{DummyFieldHolder, Field, Instance, InstanceState},
    math::Real,
    tile::Tile,
};
use gmio::{
    render::{BlendType, Fog, Light, Renderer, RendererOptions, Scaling},
    window,
    window::Cursor,
};
use image::RgbaImage;
use shared::{input::MouseButton, types::Colour};
use std::{
    io::{BufReader, Read, Write},
    process::Command,
};

macro_rules! _arg_into {
    (any, $v: expr) => {{ Ok($v.clone()) }};
    (bool, $v: expr) => {{ Ok($v.is_truthy()) }};
    (int, $v: expr) => {{ Ok(<Value as Into<i32>>::into($v.clone())) }};
    (real, $v: expr) => {{ Ok(<Value as Into<Real>>::into($v.clone())) }};
    (string, $v: expr) => {{ Ok(String::from_utf8_lossy(<&Value as Into<&[u8]>>::into($v))) }};
    (bytes, $v: expr) => {{ Ok(<Value as Into<RCStr>>::into($v.clone())) }};
}

macro_rules! _count_rep {
    () => { 0usize };
    ($x: ident $($ys: ident)*) => { 1usize + _count_rep!($($ys)*) };
}

include!(concat!(env!("OUT_DIR"), "/_apply_args.macro.rs"));

/// Helper macro to validate input arguments from a GML function.
macro_rules! expect_args {
    ($args: expr, [$($x: ident),*]) => {{
        (|| -> gml::Result<_> {
            let argc = _count_rep!($($x)*);
            if $args.len() != argc {
                Err(gml::runtime::Error::WrongArgumentCount(argc, $args.len()))
            } else {
                _apply_args!($args, $($x)*)
            }
        })()
    }};
    ($args: expr, [$($x: ident,)*]) => { expect_args!($args, $($x),*) };
}

pub fn rgb_to_hsv(colour: i32) -> (i32, i32, i32) {
    let (r, g, b) = (Real::from(colour & 0xFF), Real::from((colour >> 8) & 0xFF), Real::from((colour >> 16) & 0xFF));

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let diff = max - min;

    let mut h = (max + min) / Real::from(2.0);
    let s;
    let v = max;

    // Achromatic
    if max == min {
        h = Real::from(0.0);
        s = Real::from(0.0);
    } else {
        s = diff / max;

        if max == r {
            h = (Real::from(60) * ((g - b) / diff) + Real::from(360)) % Real::from(360)
        }
        if max == g {
            h = (Real::from(60) * ((b - r) / diff) + Real::from(120)) % Real::from(360)
        }
        if max == b {
            h = (Real::from(60) * ((r - g) / diff) + Real::from(240)) % Real::from(360)
        }
    }
    (((h / Real::from(360)) * Real::from(255)).round(), (s * Real::from(255)).round(), v.round())
}

impl Game {
    pub fn display_get_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.display_width().into())
    }

    pub fn display_get_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.display_height().into())
    }

    pub fn display_get_colordepth(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.display_colour().into())
    }

    pub fn display_get_frequency(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.display_frequency().into())
    }

    pub fn display_set_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function display_set_size");
        Ok(Default::default())
    }

    pub fn display_set_colordepth(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function display_set_colordepth");
        Ok(Default::default())
    }

    pub fn display_set_frequency(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function display_set_frequency");
        Ok(Default::default())
    }

    pub fn display_set_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function display_set_all");
        Ok(Default::default())
    }

    pub fn display_test_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function display_test_all");
        Ok(Default::default())
    }

    pub fn display_reset(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        println!("Called unimplemented kernel function display_reset");
        Ok(Default::default())
    }

    pub fn display_mouse_get_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok((self.input_manager.mouse_get_location().0 + f64::from(self.window.get_pos().0)).into())
    }

    pub fn display_mouse_get_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok((self.input_manager.mouse_get_location().1 + f64::from(self.window.get_pos().1)).into())
    }

    pub fn display_mouse_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function display_mouse_set");
        Ok(Default::default())
    }

    pub fn window_set_visible(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let visible = expect_args!(args, [bool])?;
        self.window.set_visible(visible);
        Ok(Default::default())
    }

    pub fn window_get_visible(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.get_visible().into())
    }

    pub fn window_set_fullscreen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function window_set_fullscreen");
        Ok(Default::default())
    }

    pub fn window_get_fullscreen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function window_get_fullscreen");
        Ok(Default::default())
    }

    pub fn window_set_showborder(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let show_border = expect_args!(args, [bool])?;
        if show_border != self.window_border {
            self.window_border = show_border;
            if self.play_type != PlayType::Record {
                self.window.set_style(match (show_border, self.window_icons) {
                    (true, true) => window::Style::Regular,
                    (true, false) => window::Style::Undecorated,
                    (false, _) => window::Style::Borderless,
                });
            }
        }
        Ok(Default::default())
    }

    pub fn window_get_showborder(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window_border.into())
    }

    pub fn window_set_showicons(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let show_icons = expect_args!(args, [bool])?;
        if show_icons != self.window_icons {
            self.window_icons = show_icons;
            if self.play_type != PlayType::Record {
                self.window.set_style(match (self.window_border, show_icons) {
                    (true, true) => window::Style::Regular,
                    (true, false) => window::Style::Undecorated,
                    (false, _) => window::Style::Borderless,
                });
            }
        }
        Ok(Default::default())
    }

    pub fn window_get_showicons(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window_icons.into())
    }

    pub fn window_set_stayontop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function window_set_stayontop");
        Ok(Default::default())
    }

    pub fn window_get_stayontop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function window_get_stayontop");
        Ok(Default::default())
    }

    pub fn window_set_sizeable(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function window_set_sizeable");
        Ok(Default::default())
    }

    pub fn window_get_sizeable(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function window_get_sizeable");
        Ok(Default::default())
    }

    pub fn window_set_caption(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let caption = expect_args!(args, [string])?;
        self.window.set_title(caption.as_ref());
        Ok(Default::default())
    }

    pub fn window_get_caption(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.get_title().to_owned().into())
    }

    pub fn window_set_cursor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function window_set_cursor");
        Ok(Default::default())
    }

    pub fn window_get_cursor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function window_get_cursor");
        Ok(Default::default())
    }

    pub fn window_set_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let col = expect_args!(args, [int])?;
        self.background_colour = (col as u32).into();
        Ok(Default::default())
    }

    pub fn window_get_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(u32::from(self.background_colour).into())
    }

    pub fn window_set_position(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [int, int])?;
        self.window.set_pos(x, y);
        Ok(Default::default())
    }

    pub fn window_set_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (width, height) = expect_args!(args, [int, int])?;
        self.window.resize(width as _, height as _);
        Ok(Default::default())
    }

    pub fn window_set_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function window_set_rectangle");
        Ok(Default::default())
    }

    pub fn window_center(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        self.window.center();
        Ok(Default::default())
    }

    pub fn window_default(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function window_default");
        Ok(Default::default())
    }

    pub fn window_get_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.get_pos().0.into())
    }

    pub fn window_get_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.get_pos().1.into())
    }

    pub fn window_get_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.get_inner_size().0.into())
    }

    pub fn window_get_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.window.get_inner_size().1.into())
    }

    pub fn window_set_region_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        // unscaled_width and unscaled_height will need to be separated into framebuffer size
        // and window region size for this to work
        // probably keep the framebuffer size on the renderer and make a getter?
        println!("Called unimplemented kernel function window_set_region_size");
        Ok(Default::default())
    }

    pub fn window_get_region_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.unscaled_width.into())
    }

    pub fn window_get_region_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.unscaled_height.into())
    }

    pub fn window_set_region_scale(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (scaling, shrink_window) = expect_args!(args, [real, bool])?;
        let scaling = match scaling {
            n if n == 0.into() => Scaling::Full,
            n if n < 0.into() => Scaling::Aspect(n.into_inner()),
            n => Scaling::Fixed(n.into_inner()),
        };
        self.scaling = scaling;
        if let Scaling::Fixed(n) = scaling {
            let (region_w, region_h) =
                ((self.unscaled_width as f64 * n) as u32, (self.unscaled_height as f64 * n) as u32);
            let (width, height) = if shrink_window {
                let (window_w, window_h) = self.window.get_inner_size();
                (region_w.max(window_w), region_h.max(window_h))
            } else {
                (region_w, region_h)
            };
            self.window.resize(width, height);
        }
        Ok(Default::default())
    }

    pub fn window_get_region_scale(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(match self.scaling {
            Scaling::Fixed(n) => n,
            Scaling::Aspect(n) => n,
            Scaling::Full => 0.0,
        }
        .into())
    }

    pub fn window_mouse_get_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.input_manager.mouse_get_location().0.into())
    }

    pub fn window_mouse_get_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.input_manager.mouse_get_location().1.into())
    }

    pub fn window_mouse_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function window_mouse_set");
        Ok(Default::default())
    }

    pub fn window_view_mouse_get_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function window_view_mouse_get_x");
        Ok(Default::default())
    }

    pub fn window_view_mouse_get_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function window_view_mouse_get_y");
        Ok(Default::default())
    }

    pub fn window_view_mouse_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function window_view_mouse_set");
        Ok(Default::default())
    }

    pub fn window_views_mouse_get_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function window_views_mouse_get_x");
        Ok(Default::default())
    }

    pub fn window_views_mouse_get_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function window_views_mouse_get_y");
        Ok(Default::default())
    }

    pub fn window_views_mouse_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function window_views_mouse_set");
        Ok(Default::default())
    }

    pub fn set_synchronization(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let synchro = expect_args!(args, [bool])?;
        self.renderer.set_vsync(synchro);
        Ok(Default::default())
    }

    pub fn set_automatic_draw(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let auto_draw = expect_args!(args, [bool])?;
        self.auto_draw = auto_draw;
        Ok(Default::default())
    }

    pub fn screen_redraw(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.draw()?;
        Ok(Default::default())
    }

    pub fn screen_refresh(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let (width, height) = self.window.get_inner_size();
        self.renderer.present(width, height, self.scaling);
        Ok(Default::default())
    }

    pub fn screen_wait_vsync(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.renderer.wait_vsync();
        Ok(Default::default())
    }

    pub fn screen_save(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let fname = expect_args!(args, [string])?;
        self.renderer.flush_queue();
        let (width, height) = (self.unscaled_width, self.unscaled_height);
        let rgba = self.renderer.get_pixels(0, 0, width as _, height as _);
        let mut image = RgbaImage::from_vec(width, height, rgba.into()).unwrap();
        asset::sprite::process_image(&mut image, false, false, true);
        match file::save_image(fname.as_ref(), image) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("screen_save".into(), e.to_string())),
        }
    }

    pub fn screen_save_part(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (fname, x, y, w, h) = expect_args!(args, [string, int, int, int, int])?;
        let x = x.max(0);
        let y = y.max(0);
        let w = w.min(self.unscaled_width as i32 - x);
        let h = h.min(self.unscaled_height as i32 - y);
        self.renderer.flush_queue();
        let rgba = self.renderer.get_pixels(x, y, w, h);
        let mut image = RgbaImage::from_vec(w as _, h as _, rgba.into()).unwrap();
        asset::sprite::process_image(&mut image, false, false, true);
        match file::save_image(fname.as_ref(), image) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("screen_save_part".into(), e.to_string())),
        }
    }

    pub fn draw_getpixel(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [int, int])?;
        self.renderer.flush_queue();
        let data = self.renderer.get_pixels(x, y, 1, 1);
        Ok(u32::from_le_bytes([data[0], data[1], data[2], 0]).into())
    }

    pub fn draw_set_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let col = expect_args!(args, [int])?;
        self.draw_colour = (col as u32).into();
        Ok(Default::default())
    }

    pub fn draw_set_alpha(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.draw_alpha = expect_args!(args, [real])?;
        Ok(Default::default())
    }

    pub fn draw_get_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(u32::from(self.draw_colour).into())
    }

    pub fn draw_get_alpha(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.draw_alpha.into())
    }

    pub fn make_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int, int, int]).map(|(r, g, b)| r + (g * 256) + (b * 256 * 256)).map(Value::from)
    }

    pub fn make_color_rgb(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int, int, int]).map(|(r, g, b)| r + (g * 256) + (b * 256 * 256)).map(Value::from)
    }

    pub fn make_color_hsv(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (h, s, v) = expect_args!(args, [real, real, real])?;
        let h = h * Real::from(360.0) / Real::from(255.0);
        let s = s / Real::from(255.0);
        let v = v / Real::from(255.0);
        let chroma = v * s;
        let hprime = (h / Real::from(60.0)) % Real::from(6.0);
        let x = chroma * (Real::from(1.0) - ((hprime % Real::from(2.0)) - Real::from(1.0)).abs());
        let m = v - chroma;

        let (r, g, b) = match hprime.floor().into_inner() as i32 {
            0 => (chroma, x, Real::from(0.0)),
            1 => (x, chroma, Real::from(0.0)),
            2 => (Real::from(0.0), chroma, x),
            3 => (Real::from(0.0), x, chroma),
            4 => (x, Real::from(0.0), chroma),
            5 => (chroma, Real::from(0.0), x),
            _ => (Real::from(0.0), Real::from(0.0), Real::from(0.0)),
        };

        let out_r = ((r + m) * Real::from(255.0)).round();
        let out_g = ((g + m) * Real::from(255.0)).round();
        let out_b = ((b + m) * Real::from(255.0)).round();
        Ok((out_r | (out_g << 8) | (out_b << 16)).into())
    }

    pub fn color_get_red(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int]).map(|c| c % 256).map(Value::from)
    }

    pub fn color_get_green(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int]).map(|c| (c / 256) % 256).map(Value::from)
    }

    pub fn color_get_blue(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int]).map(|c| (c / 256 / 256) % 256).map(Value::from)
    }

    pub fn color_get_hue(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let c = expect_args!(args, [int])?;
        let (h, _, _) = rgb_to_hsv(c);
        Ok(h.into())
    }

    pub fn color_get_saturation(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let c = expect_args!(args, [int])?;
        let (_, s, _) = rgb_to_hsv(c);
        Ok(s.into())
    }

    pub fn color_get_value(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let c = expect_args!(args, [int])?;
        let (_, _, v) = rgb_to_hsv(c);
        Ok(v.into())
    }

    pub fn merge_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (c1, c2, amount) = expect_args!(args, [int, int, real])?;
        let r = Real::from(c1 & 255) * (Real::from(1) - amount) + Real::from(c2 & 255) * amount;
        let g = Real::from((c1 >> 8) & 255) * (Real::from(1) - amount) + Real::from((c2 >> 8) & 255) * amount;
        let b = Real::from((c1 >> 16) & 255) * (Real::from(1) - amount) + Real::from((c2 >> 16) & 255) * amount;
        Ok(Value::from((r.round() & 255) + ((g.round() & 255) << 8) + ((b.round() & 255) << 16)))
    }

    pub fn draw_set_blend_mode(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let mode = expect_args!(args, [int])?;
        let (src, dest) = match mode {
            1 => (BlendType::SrcAlpha, BlendType::One),          // bm_add
            2 => (BlendType::SrcAlpha, BlendType::InvSrcColour), // bm_subtract
            3 => (BlendType::Zero, BlendType::InvSrcColour),     // bm_max
            _ => (BlendType::SrcAlpha, BlendType::InvSrcAlpha),  // bm_normal
        };
        self.renderer.set_blend_mode(src, dest);
        Ok(Default::default())
    }

    pub fn draw_set_blend_mode_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (src, dest) = expect_args!(args, [int, int])?;
        let int_to_blend_type = |i| match i {
            2 => BlendType::One,
            3 => BlendType::SrcColour,
            4 => BlendType::InvSrcColour,
            5 => BlendType::SrcAlpha,
            6 => BlendType::InvSrcAlpha,
            7 => BlendType::DestAlpha,
            8 => BlendType::InvDestAlpha,
            9 => BlendType::DestColour,
            10 => BlendType::InvDestColour,
            11 => BlendType::SrcAlphaSaturate,
            _ => BlendType::Zero,
        };
        let src = int_to_blend_type(src);
        let dest = int_to_blend_type(dest);
        self.renderer.set_blend_mode(src, dest);
        Ok(Default::default())
    }

    pub fn draw_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let col = expect_args!(args, [int])?;
        self.renderer.clear_view((col as u32).into(), 1.0);
        Ok(Default::default())
    }

    pub fn draw_clear_alpha(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (col, alpha) = expect_args!(args, [int, real])?;
        self.renderer.clear_view((col as u32).into(), alpha.into());
        Ok(Default::default())
    }

    pub fn draw_point(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [real, real])?;
        self.renderer.draw_point(x.into(), y.into(), u32::from(self.draw_colour) as _, self.draw_alpha.into());
        Ok(Default::default())
    }

    pub fn draw_line(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2) = expect_args!(args, [real, real, real, real])?;
        self.renderer.draw_line(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            None,
            u32::from(self.draw_colour) as _,
            u32::from(self.draw_colour) as _,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn draw_line_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, w) = expect_args!(args, [real, real, real, real, real])?;
        self.renderer.draw_line(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            Some(w.into()),
            u32::from(self.draw_colour) as _,
            u32::from(self.draw_colour) as _,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn draw_rectangle(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, outline) = expect_args!(args, [real, real, real, real, bool])?;
        if outline {
            self.renderer.draw_rectangle_outline(
                x1.into(),
                y1.into(),
                x2.into(),
                y2.into(),
                u32::from(self.draw_colour) as _,
                self.draw_alpha.into(),
            );
        } else {
            self.renderer.draw_rectangle(
                x1.into(),
                y1.into(),
                x2.into(),
                y2.into(),
                u32::from(self.draw_colour) as _,
                self.draw_alpha.into(),
            );
        }
        Ok(Default::default())
    }

    pub fn draw_roundrect(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, outline) = expect_args!(args, [real, real, real, real, bool])?;
        self.renderer.draw_roundrect(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            u32::from(self.draw_colour) as _,
            u32::from(self.draw_colour) as _,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_triangle(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, x3, y3, outline) = expect_args!(args, [real, real, real, real, real, real, bool])?;
        self.renderer.draw_triangle(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            x3.into(),
            y3.into(),
            u32::from(self.draw_colour) as _,
            u32::from(self.draw_colour) as _,
            u32::from(self.draw_colour) as _,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_circle(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, radius, outline) = expect_args!(args, [real, real, real, bool])?;
        self.renderer.draw_ellipse(
            x.into(),
            y.into(),
            radius.into(),
            radius.into(),
            u32::from(self.draw_colour) as _,
            u32::from(self.draw_colour) as _,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_ellipse(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, outline) = expect_args!(args, [real, real, real, real, bool])?;
        let xcenter = (x1 + x2) / 2.into();
        let ycenter = (y1 + y2) / 2.into();
        let rad_x = (xcenter - x1).abs();
        let rad_y = (ycenter - y1).abs();
        self.renderer.draw_ellipse(
            xcenter.into(),
            ycenter.into(),
            rad_x.into(),
            rad_y.into(),
            u32::from(self.draw_colour) as _,
            u32::from(self.draw_colour) as _,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_arrow(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, size) = expect_args!(args, [real, real, real, real, real])?;
        let (x1, y1, x2, y2) = (x1.into_inner(), y1.into_inner(), x2.into_inner(), y2.into_inner());
        let length = (x2 - x1).hypot(y2 - y1);
        if length != 0.0 {
            self.renderer.draw_line(
                x1,
                y1,
                x2,
                y2,
                None,
                u32::from(self.draw_colour) as _,
                u32::from(self.draw_colour) as _,
                self.draw_alpha.into(),
            );
            let size = size.into_inner().min(length);
            let x_offset = (x2 - x1) * size / length;
            let y_offset = (y2 - y1) * size / length;
            self.renderer.draw_triangle(
                (x2 - x_offset) - y_offset / 3.0,
                (y2 - y_offset) + x_offset / 3.0,
                x2,
                y2,
                (x2 - x_offset) + y_offset / 3.0,
                (y2 - y_offset) - x_offset / 3.0,
                u32::from(self.draw_colour) as _,
                u32::from(self.draw_colour) as _,
                u32::from(self.draw_colour) as _,
                self.draw_alpha.into(),
                false,
            );
        }
        Ok(Default::default())
    }

    pub fn draw_button(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function draw_button");
        Ok(Default::default())
    }

    pub fn draw_healthbar(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, amount, backcol, mincol, maxcol, direction, showback, showborder) =
            expect_args!(args, [real, real, real, real, real, int, int, int, int, bool, bool])?;
        let health_ratio = f64::from(amount / Real::from(100.0));
        let lerp = |min, max| (health_ratio * f64::from(max) + (1.0 - health_ratio) * f64::from(min)) as u8 as i32;
        let bar_colour = lerp(mincol & 0xff, maxcol & 0xff)
            | (lerp((mincol >> 8) & 0xff, (maxcol >> 8) & 0xff) << 8)
            | (lerp((mincol >> 16) & 0xff, (maxcol >> 16) & 0xff) << 16);
        let (x1, y1, x2, y2) = (x1.into_inner(), y1.into_inner(), x2.into_inner(), y2.into_inner());
        if showback {
            self.renderer.draw_rectangle(x1, y1, x2, y2, backcol, self.draw_alpha.into());
            if showborder {
                self.renderer.draw_rectangle_outline(x1, y1, x2, y2, 0, self.draw_alpha.into());
            }
        }
        let (x1, y1, x2, y2) = match direction {
            1 => (x2 - (x2 - x1) * health_ratio, y1, x2, y2),
            2 => (x1, y1, x2, y1 + (y2 - y1) * health_ratio),
            3 => (x1, y2 - (y2 - y1) * health_ratio, x2, y2),
            _ => (x1, y1, x1 + (x2 - x1) * health_ratio, y2),
        };
        self.renderer.draw_rectangle(x1, y1, x2, y2, bar_colour, self.draw_alpha.into());
        if showborder {
            self.renderer.draw_rectangle_outline(x1, y1, x2, y2, 0, self.draw_alpha.into());
        }
        Ok(Default::default())
    }

    pub fn draw_path(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path, x, y, absolute) = expect_args!(args, [int, real, real, bool])?;
        let path = self.assets.paths.get_asset(path).ok_or(gml::Error::NonexistentAsset(asset::Type::Path, path))?;
        let (x_offset, y_offset) = if absolute { (0.into(), 0.into()) } else { (x - path.start.x, y - path.start.y) };

        for (node1, node2) in path.control_nodes.windows(2).map(|x| (x[0], x[1])) {
            self.renderer.draw_line(
                f64::from(node1.point.x + x_offset),
                f64::from(node1.point.y + y_offset),
                f64::from(node2.point.x + x_offset),
                f64::from(node2.point.y + y_offset),
                None,
                u32::from(self.draw_colour) as _,
                u32::from(self.draw_colour) as _,
                self.draw_alpha.into(),
            );
        }

        Ok(Default::default())
    }

    pub fn draw_point_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, col) = expect_args!(args, [real, real, int])?;
        self.renderer.draw_point(x.into(), y.into(), col, self.draw_alpha.into());
        Ok(Default::default())
    }

    pub fn draw_line_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, c1, c2) = expect_args!(args, [real, real, real, real, int, int])?;
        self.renderer.draw_line(x1.into(), y1.into(), x2.into(), y2.into(), None, c1, c2, self.draw_alpha.into());
        Ok(Default::default())
    }

    pub fn draw_line_width_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, w, c1, c2) = expect_args!(args, [real, real, real, real, real, int, int])?;
        self.renderer.draw_line(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            Some(w.into()),
            c1,
            c2,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn draw_rectangle_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, c1, c2, c3, c4, outline) =
            expect_args!(args, [real, real, real, real, int, int, int, int, bool])?;
        self.renderer.draw_rectangle_gradient(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            c1,
            c2,
            c3,
            c4,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_roundrect_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, col1, col2, outline) = expect_args!(args, [real, real, real, real, int, int, bool])?;
        self.renderer.draw_roundrect(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            col1,
            col2,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_triangle_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, x3, y3, c1, c2, c3, outline) =
            expect_args!(args, [real, real, real, real, real, real, int, int, int, bool])?;
        self.renderer.draw_triangle(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            x3.into(),
            y3.into(),
            c1,
            c2,
            c3,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_circle_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, radius, col1, col2, outline) = expect_args!(args, [real, real, real, int, int, bool])?;
        self.renderer.draw_ellipse(
            x.into(),
            y.into(),
            radius.into(),
            radius.into(),
            col1,
            col2,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_ellipse_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, col1, col2, outline) = expect_args!(args, [real, real, real, real, int, int, bool])?;
        let xcenter = (x1 + x2) / 2.into();
        let ycenter = (y1 + y2) / 2.into();
        let rad_x = (xcenter - x1).abs();
        let rad_y = (ycenter - y1).abs();
        self.renderer.draw_ellipse(
            xcenter.into(),
            ycenter.into(),
            rad_x.into(),
            rad_y.into(),
            col1,
            col2,
            self.draw_alpha.into(),
            outline,
        );
        Ok(Default::default())
    }

    pub fn draw_set_circle_precision(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let prec = expect_args!(args, [int])?;
        self.renderer.set_circle_precision(prec);
        Ok(Default::default())
    }

    pub fn draw_primitive_begin(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let kind = expect_args!(args, [int])?;
        self.renderer.reset_primitive_2d(kind.into(), None);
        Ok(Default::default())
    }

    pub fn draw_primitive_begin_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (kind, texture) = expect_args!(args, [int, int])?;
        self.renderer.reset_primitive_2d(kind.into(), self.renderer.get_texture_from_id(texture as _).copied());
        Ok(Default::default())
    }

    pub fn draw_primitive_end(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.renderer.draw_primitive_2d();
        Ok(Default::default())
    }

    pub fn draw_vertex(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [real, real])?;
        self.renderer.vertex_2d(x.into(), y.into(), 0.0, 0.0, u32::from(self.draw_colour) as _, self.draw_alpha.into());
        Ok(Default::default())
    }

    pub fn draw_vertex_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, col, alpha) = expect_args!(args, [real, real, int, real])?;
        self.renderer.vertex_2d(x.into(), y.into(), 0.0, 0.0, col, alpha.into());
        Ok(Default::default())
    }

    pub fn draw_vertex_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, xtex, ytex) = expect_args!(args, [real, real, real, real])?;
        self.renderer.vertex_2d(
            x.into(),
            y.into(),
            xtex.into(),
            ytex.into(),
            u32::from(self.draw_colour) as _,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn draw_vertex_texture_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, xtex, ytex, col, alpha) = expect_args!(args, [real, real, real, real, int, real])?;
        self.renderer.vertex_2d(x.into(), y.into(), xtex.into(), ytex.into(), col, alpha.into());
        Ok(Default::default())
    }

    pub fn sprite_get_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index) = expect_args!(args, [int, int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            if let Some(atlas_ref) = sprite.get_atlas_ref(Real::from(image_index)) {
                return Ok(self.renderer.get_texture_id(atlas_ref).into())
            }
            Ok((-1).into())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn background_get_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let bg_index = expect_args!(args, [int])?;
        if let Some(background) = self.assets.backgrounds.get_asset(bg_index) {
            if let Some(atlas_ref) = &background.atlas_ref {
                Ok(self.renderer.get_texture_id(atlas_ref).into())
            } else {
                Ok((-1).into())
            }
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Background, bg_index))
        }
    }

    pub fn texture_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function texture_exists");
        Ok(Default::default())
    }

    pub fn texture_set_interpolation(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let lerping = expect_args!(args, [bool])?;
        self.renderer.set_pixel_interpolation(lerping);
        Ok(Default::default())
    }

    pub fn texture_set_blending(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function texture_set_blending");
        Ok(Default::default())
    }

    pub fn texture_set_repeat(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let repeat = expect_args!(args, [bool])?;
        self.renderer.set_texture_repeat(repeat);
        Ok(Default::default())
    }

    pub fn texture_get_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let _texid = expect_args!(args, [int])?;
        Ok(1.into()) // we don't pad textures to power-of-2
    }

    pub fn texture_get_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let _texid = expect_args!(args, [int])?;
        Ok(1.into()) // see texture_get_width
    }

    pub fn texture_preload(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function texture_preload");
        Ok(Default::default())
    }

    pub fn texture_set_priority(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function texture_set_priority");
        Ok(Default::default())
    }

    pub fn draw_set_font(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let font_id = expect_args!(args, [int])?;
        if self.assets.fonts.get_asset(font_id).is_some() {
            self.draw_font_id = font_id;
        } else {
            self.draw_font_id = -1;
        }
        Ok(Default::default())
    }

    pub fn draw_set_halign(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.draw_halign = match expect_args!(args, [int])? {
            1 => draw::Halign::Middle,
            2 => draw::Halign::Right,
            0 | _ => draw::Halign::Left,
        };
        Ok(Default::default())
    }

    pub fn draw_set_valign(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.draw_valign = match expect_args!(args, [int])? {
            0 => draw::Valign::Top,
            1 => draw::Valign::Middle,
            2 | _ => draw::Valign::Bottom,
        };
        Ok(Default::default())
    }

    pub fn string_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let string = expect_args!(args, [bytes])?;
        let (width, _) = self.get_string_size(string, None, None);
        Ok(width.into())
    }

    pub fn string_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let string = expect_args!(args, [bytes])?;
        let (_, height) = self.get_string_size(string, None, None);
        Ok(height.into())
    }

    pub fn string_width_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (string, line_height, max_width) = expect_args!(args, [bytes, int, int])?;
        let (width, _) = self.get_string_size(
            string,
            if line_height < 0 { None } else { Some(line_height as _) },
            if max_width < 0 { None } else { Some(max_width as _) },
        );
        Ok(width.into())
    }

    pub fn string_height_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (string, line_height, max_width) = expect_args!(args, [bytes, int, int])?;
        let (_, height) = self.get_string_size(
            string,
            if line_height < 0 { None } else { Some(line_height as _) },
            if max_width < 0 { None } else { Some(max_width as _) },
        );
        Ok(height.into())
    }

    pub fn draw_text(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text) = expect_args!(args, [real, real, any])?;
        self.draw_string(x, y, text.repr(), None, None, 1.into(), 1.into(), 0.into(), None, self.draw_alpha.into());
        Ok(Default::default())
    }

    pub fn draw_text_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text, line_height, max_width) = expect_args!(args, [real, real, any, int, int])?;
        let line_height = if line_height < 0 { None } else { Some(line_height as _) };
        let max_width = if max_width < 0 { None } else { Some(max_width as _) };

        self.draw_string(
            x,
            y,
            text.repr(),
            line_height,
            max_width,
            1.into(),
            1.into(),
            0.into(),
            None,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn draw_text_transformed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text, xscale, yscale, angle) = expect_args!(args, [real, real, any, real, real, real])?;
        self.draw_string(x, y, text.repr(), None, None, xscale, yscale, angle, None, self.draw_alpha.into());
        Ok(Default::default())
    }

    pub fn draw_text_ext_transformed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text, line_height, max_width, xscale, yscale, angle) =
            expect_args!(args, [real, real, any, int, int, real, real, real])?;
        let line_height = if line_height < 0 { None } else { Some(line_height as _) };
        let max_width = if max_width < 0 { None } else { Some(max_width as _) };

        self.draw_string(
            x,
            y,
            text.repr(),
            line_height,
            max_width,
            xscale,
            yscale,
            angle,
            None,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn draw_text_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text, col1, col2, col3, col4, alpha) =
            expect_args!(args, [real, real, any, int, int, int, int, real])?;
        self.draw_string(
            x,
            y,
            text.repr(),
            None,
            None,
            1.into(),
            1.into(),
            0.into(),
            Some((col1, col2, col3, col4)),
            alpha,
        );
        Ok(Default::default())
    }

    pub fn draw_text_transformed_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text, xscale, yscale, angle, col1, col2, col3, col4, alpha) =
            expect_args!(args, [real, real, any, real, real, real, int, int, int, int, real])?;
        self.draw_string(x, y, text.repr(), None, None, xscale, yscale, angle, Some((col1, col2, col3, col4)), alpha);
        Ok(Default::default())
    }

    pub fn draw_text_ext_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text, line_height, max_width, col1, col2, col3, col4, alpha) =
            expect_args!(args, [real, real, any, int, int, int, int, int, int, real])?;
        let line_height = if line_height < 0 { None } else { Some(line_height as _) };
        let max_width = if max_width < 0 { None } else { Some(max_width as _) };

        self.draw_string(
            x,
            y,
            text.repr(),
            line_height,
            max_width,
            1.into(),
            1.into(),
            0.into(),
            Some((col1, col2, col3, col4)),
            alpha,
        );
        Ok(Default::default())
    }

    pub fn draw_text_ext_transformed_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, text, line_height, max_width, xscale, yscale, angle, col1, col2, col3, col4, alpha) =
            expect_args!(args, [real, real, any, int, int, real, real, real, int, int, int, int, real])?;
        let line_height = if line_height < 0 { None } else { Some(line_height as _) };
        let max_width = if max_width < 0 { None } else { Some(max_width as _) };

        self.draw_string(
            x,
            y,
            text.repr(),
            line_height,
            max_width,
            xscale,
            yscale,
            angle,
            Some((col1, col2, col3, col4)),
            alpha,
        );
        Ok(Default::default())
    }

    pub fn draw_self(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        if let Some(sprite) = self.assets.sprites.get_asset(instance.sprite_index.get()) {
            if let Some(atlas_ref) = sprite.get_atlas_ref(Real::from(instance.image_index.get())) {
                self.renderer.draw_sprite(
                    atlas_ref,
                    instance.x.get().into(),
                    instance.y.get().into(),
                    instance.image_xscale.get().into(),
                    instance.image_yscale.get().into(),
                    instance.image_angle.get().into(),
                    instance.image_blend.get(),
                    instance.image_alpha.get().into(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, instance.sprite_index.get()))
        }
    }

    pub fn draw_sprite(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, x, y) = expect_args!(args, [int, real, real, real])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            if let Some(atlas_ref) = sprite.get_atlas_ref(image_index) {
                self.renderer.draw_sprite(atlas_ref, x.into(), y.into(), 1.0, 1.0, 0.0, 0xffffff, 1.0);
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn draw_sprite_pos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 11
        println!("Called unimplemented kernel function draw_sprite_pos");
        Ok(Default::default())
    }

    pub fn draw_sprite_ext(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, x, y, xscale, yscale, angle, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, real, real, int, real])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            let image_index = if image_index < Real::from(0.0) {
                self.instance_list.get(context.this).image_index.get()
            } else {
                image_index
            };
            if let Some(atlas_ref) = sprite.get_atlas_ref(image_index) {
                self.renderer.draw_sprite(
                    atlas_ref,
                    x.into(),
                    y.into(),
                    xscale.into(),
                    yscale.into(),
                    angle.into(),
                    colour,
                    alpha.into(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn draw_sprite_stretched(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, x, y, w, h) = expect_args!(args, [any, any, any, any, any, any])?;
        let instance = self.instance_list.get(context.this);
        let args = [
            sprite_index,
            image_index,
            x,
            y,
            w,
            h,
            instance.image_blend.get().into(),
            instance.image_alpha.get().into(),
        ];
        self.draw_sprite_stretched_ext(context, &args)
    }

    pub fn draw_sprite_stretched_ext(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, x, y, w, h, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, real, int, real])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            let image_index = if image_index < Real::from(0.0) {
                self.instance_list.get(context.this).image_index.get()
            } else {
                image_index
            };
            if let Some(atlas_ref) = sprite.get_atlas_ref(image_index) {
                self.renderer.draw_sprite(
                    atlas_ref,
                    x.into(),
                    y.into(),
                    (w / sprite.width.into()).into(),
                    (h / sprite.height.into()).into(),
                    0.0,
                    colour,
                    alpha.into(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn draw_sprite_part(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, left, top, width, height, x, y) =
            expect_args!(args, [any, any, any, any, any, any, any, any])?;
        self.draw_sprite_part_ext(context, &[
            sprite_index,
            image_index,
            left,
            top,
            width,
            height,
            x,
            y,
            1.into(),
            1.into(),
            0xFFFFFF.into(),
            1.into(),
        ])
    }

    pub fn draw_sprite_part_ext(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, left, top, width, height, x, y, xscale, yscale, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, real, int, real])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            let image_index = if image_index < Real::from(0.0) {
                self.instance_list.get(context.this).image_index.get()
            } else {
                image_index
            };
            if let Some(atlas_ref) = sprite.get_atlas_ref(image_index) {
                self.renderer.draw_sprite_partial(
                    atlas_ref,
                    left.into(),
                    top.into(),
                    width.into(),
                    height.into(),
                    x.into(),
                    y.into(),
                    xscale.into(),
                    yscale.into(),
                    0.0,
                    colour,
                    alpha.into(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn draw_sprite_general(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (
            sprite_index,
            image_index,
            left,
            top,
            width,
            height,
            x,
            y,
            xscale,
            yscale,
            angle,
            col1,
            col2,
            col3,
            col4,
            alpha,
        ) = expect_args!(args, [
            int, real, real, real, real, real, real, real, real, real, real, int, int, int, int, real
        ])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            let image_index = if image_index < Real::from(0.0) {
                self.instance_list.get(context.this).image_index.get()
            } else {
                image_index
            };
            if let Some(atlas_ref) = sprite.get_atlas_ref(image_index) {
                self.renderer.draw_sprite_general(
                    atlas_ref,
                    left.into(),
                    top.into(),
                    width.into(),
                    height.into(),
                    x.into(),
                    y.into(),
                    xscale.into(),
                    yscale.into(),
                    angle.into(),
                    col1,
                    col2,
                    col3,
                    col4,
                    alpha.into(),
                    false,
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn draw_sprite_tiled(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, x, y) = expect_args!(args, [any, any, any, any])?;
        self.draw_sprite_tiled_ext(context, &[
            sprite_index,
            image_index,
            x,
            y,
            1.into(),
            1.into(),
            0xffffff.into(),
            1.into(),
        ])
    }

    pub fn draw_sprite_tiled_ext(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_index, image_index, x, y, xscale, yscale, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, real, int, real])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            let image_index = if image_index < Real::from(0.0) {
                self.instance_list.get(context.this).image_index.get()
            } else {
                image_index
            };
            if let Some(atlas_ref) = sprite.get_atlas_ref(image_index) {
                self.renderer.draw_sprite_tiled(
                    atlas_ref,
                    x.into(),
                    y.into(),
                    xscale.into(),
                    yscale.into(),
                    colour,
                    alpha.into(),
                    Some(self.room_width.into()),
                    Some(self.room_height.into()),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_index))
        }
    }

    pub fn draw_background(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, x, y) = expect_args!(args, [any, any, any])?;
        self.draw_background_ext(context, &[bg_index, x, y, 1.into(), 1.into(), 0.into(), 0xFFFFFF.into(), 1.into()])
    }

    pub fn draw_background_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, x, y, xscale, yscale, angle, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, real, int, real])?;
        if let Some(background) = self.assets.backgrounds.get_asset(bg_index) {
            if let Some(atlas_ref) = &background.atlas_ref {
                self.renderer.draw_sprite(
                    atlas_ref,
                    x.into(),
                    y.into(),
                    xscale.into(),
                    yscale.into(),
                    angle.into(),
                    colour,
                    alpha.into(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Background, bg_index))
        }
    }

    pub fn draw_background_stretched(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, x, y, w, h) = expect_args!(args, [any, any, any, any, any])?;
        self.draw_background_stretched_ext(context, &[bg_index, x, y, w, h, 0xffffff.into(), 1.0.into()])
    }

    pub fn draw_background_stretched_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, x, y, w, h, colour, alpha) = expect_args!(args, [int, real, real, real, real, int, real])?;
        if let Some(background) = self.assets.backgrounds.get_asset(bg_index) {
            if let Some(atlas_ref) = &background.atlas_ref {
                self.renderer.draw_sprite(
                    atlas_ref,
                    x.into(),
                    y.into(),
                    (w / background.width.into()).into(),
                    (h / background.height.into()).into(),
                    0.0,
                    colour,
                    alpha.into(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Background, bg_index))
        }
    }

    pub fn draw_background_part(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, left, top, width, height, x, y) = expect_args!(args, [any, any, any, any, any, any, any])?;

        self.draw_background_part_ext(context, &[
            bg_index,
            left,
            top,
            width,
            height,
            x,
            y,
            1.into(),
            1.into(),
            0xFFFFFF.into(),
            1.into(),
        ])
    }

    pub fn draw_background_part_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, left, top, width, height, x, y, xscale, yscale, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, int, real])?;
        if let Some(background) = self.assets.backgrounds.get_asset(bg_index) {
            if let Some(atlas_ref) = &background.atlas_ref {
                self.renderer.draw_sprite_partial(
                    atlas_ref,
                    left.into(),
                    top.into(),
                    width.into(),
                    height.into(),
                    x.into(),
                    y.into(),
                    xscale.into(),
                    yscale.into(),
                    0.0,
                    colour,
                    alpha.into(),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Background, bg_index))
        }
    }

    pub fn draw_background_general(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, left, top, width, height, x, y, xscale, yscale, angle, col1, col2, col3, col4, alpha) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, real, int, int, int, int, real])?;
        if let Some(background) = self.assets.backgrounds.get_asset(bg_index) {
            if let Some(atlas_ref) = &background.atlas_ref {
                self.renderer.draw_sprite_general(
                    atlas_ref,
                    left.into(),
                    top.into(),
                    width.into(),
                    height.into(),
                    x.into(),
                    y.into(),
                    xscale.into(),
                    yscale.into(),
                    angle.into(),
                    col1,
                    col2,
                    col3,
                    col4,
                    alpha.into(),
                    false,
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Background, bg_index))
        }
    }

    pub fn draw_background_tiled(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, x, y) = expect_args!(args, [any, any, any])?;
        self.draw_background_tiled_ext(context, &[bg_index, x, y, 1.into(), 1.into(), 0xFFFFFF.into(), 1.into()])
    }

    pub fn draw_background_tiled_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, x, y, xscale, yscale, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, int, real])?;
        if let Some(background) = self.assets.backgrounds.get_asset(bg_index) {
            if let Some(atlas_ref) = &background.atlas_ref {
                self.renderer.draw_sprite_tiled(
                    atlas_ref,
                    x.into(),
                    y.into(),
                    xscale.into(),
                    yscale.into(),
                    colour,
                    alpha.into(),
                    Some(self.room_width.into()),
                    Some(self.room_height.into()),
                );
            }
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Background, bg_index))
        }
    }

    pub fn tile_get_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).x.get().into())
        } else {
            Err(gml::Error::FunctionError("tile_get_x".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_get_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).y.get().into())
        } else {
            Err(gml::Error::FunctionError("tile_get_y".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_get_left(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).tile_x.get().into())
        } else {
            Err(gml::Error::FunctionError("tile_get_left".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_get_top(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).tile_y.get().into())
        } else {
            Err(gml::Error::FunctionError("tile_get_top".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_get_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).width.get().into())
        } else {
            Err(gml::Error::FunctionError("tile_get_width".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_get_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).height.get().into())
        } else {
            Err(gml::Error::FunctionError(
                "tile_get_height".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_get_depth(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).depth.get().into())
        } else {
            Err(gml::Error::FunctionError("tile_get_depth".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_get_visible(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).visible.get().into())
        } else {
            Err(gml::Error::FunctionError(
                "tile_get_visible".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_get_xscale(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).xscale.get().into())
        } else {
            Err(gml::Error::FunctionError(
                "tile_get_xscale".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_get_yscale(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).yscale.get().into())
        } else {
            Err(gml::Error::FunctionError(
                "tile_get_yscale".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_get_blend(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).blend.get().into())
        } else {
            Err(gml::Error::FunctionError("tile_get_blend".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_get_alpha(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).alpha.get().into())
        } else {
            Err(gml::Error::FunctionError("tile_get_alpha".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_get_background(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            Ok(self.tile_list.get(handle).background_index.get().into())
        } else {
            Err(gml::Error::FunctionError(
                "tile_get_background".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_set_visible(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (tile_id, visible) = expect_args!(args, [int, bool])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            self.tile_list.get(handle).visible.set(visible);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError(
                "tile_set_visible".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_set_background(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (tile_id, bg_index) = expect_args!(args, [int, int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            self.tile_list.get(handle).background_index.set(bg_index);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError(
                "tile_set_background".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_set_region(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (tile_id, left, top, width, height) = expect_args!(args, [int, int, int, int, int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            let tile = self.tile_list.get(handle);
            tile.tile_x.set(left);
            tile.tile_y.set(top);
            tile.width.set(width);
            tile.height.set(height);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError(
                "tile_set_region".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_set_position(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (tile_id, x, y) = expect_args!(args, [int, real, real])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            let tile = self.tile_list.get(handle);
            tile.x.set(x);
            tile.y.set(y);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError(
                "tile_set_position".into(),
                format!("Tile with ID {} does not exist.", tile_id),
            ))
        }
    }

    pub fn tile_set_depth(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (tile_id, depth) = expect_args!(args, [int, real])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            self.tile_list.get(handle).depth.set(depth);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("tile_set_depth".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_set_scale(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (tile_id, xscale, yscale) = expect_args!(args, [int, real, real])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            let tile = self.tile_list.get(handle);
            tile.xscale.set(xscale);
            tile.yscale.set(yscale);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("tile_set_scale".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_set_blend(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (tile_id, blend) = expect_args!(args, [int, int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            self.tile_list.get(handle).blend.set(blend);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("tile_set_blend".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_set_alpha(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (tile_id, alpha) = expect_args!(args, [int, real])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            self.tile_list.get(handle).alpha.set(alpha);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("tile_set_alpha".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (background_index, tile_x, tile_y, width, height, x, y, depth) =
            expect_args!(args, [int, int, int, int, int, real, real, real])?;
        self.last_tile_id += 1;
        self.tile_list.insert(Tile {
            x: x.into(),
            y: y.into(),
            background_index: background_index.into(),
            tile_x: tile_x.into(),
            tile_y: tile_y.into(),
            width: width.into(),
            height: height.into(),
            depth: depth.into(),
            id: self.last_tile_id.into(),
            alpha: Real::from(1.0).into(),
            blend: 0xffffff.into(),
            xscale: Real::from(1.0).into(),
            yscale: Real::from(1.0).into(),
            visible: true.into(),
        });
        Ok(self.last_tile_id.into())
    }

    pub fn tile_find(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function tile_find");
        Ok(Default::default())
    }

    pub fn tile_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        Ok(self.tile_list.get_by_tileid(tile_id).is_some().into())
    }

    pub fn tile_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let tile_id = expect_args!(args, [int])?;
        if let Some(handle) = self.tile_list.get_by_tileid(tile_id) {
            self.tile_list.remove(handle);
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("tile_delete".into(), format!("Tile with ID {} does not exist.", tile_id)))
        }
    }

    pub fn tile_delete_at(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function tile_delete_at");
        Ok(Default::default())
    }

    pub fn tile_layer_hide(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let depth = expect_args!(args, [real])?;
        let mut iter_tile = self.tile_list.iter_by_drawing();
        while let Some(handle) = iter_tile.next(&self.tile_list) {
            let tile = self.tile_list.get(handle);
            if tile.depth.get() == depth {
                tile.visible.set(false);
            }
        }
        Ok(Default::default())
    }

    pub fn tile_layer_show(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let depth = expect_args!(args, [real])?;
        let mut iter_tile = self.tile_list.iter_by_drawing();
        while let Some(handle) = iter_tile.next(&self.tile_list) {
            let tile = self.tile_list.get(handle);
            if tile.depth.get() == depth {
                tile.visible.set(true);
            }
        }
        Ok(Default::default())
    }

    pub fn tile_layer_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let depth = expect_args!(args, [real])?;
        self.tile_list.remove_with(|t| t.depth.get() == depth);
        Ok(Default::default())
    }

    pub fn tile_layer_shift(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (depth, x, y) = expect_args!(args, [real, real, real])?;
        let mut iter_tile = self.tile_list.iter_by_drawing();
        while let Some(handle) = iter_tile.next(&self.tile_list) {
            let tile = self.tile_list.get(handle);
            if tile.depth.get() == depth {
                tile.x.set(tile.x.get() + x);
                tile.y.set(tile.y.get() + y);
            }
        }
        Ok(Default::default())
    }

    pub fn tile_layer_find(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (depth, x, y) = expect_args!(args, [real, real, real])?;
        let use_scaling = self.gm_version == Version::GameMaker8_1; // 8.1 bugfix
        let mut iter_tile = self.tile_list.iter_by_drawing();
        while let Some(handle) = iter_tile.next(&self.tile_list) {
            let tile = self.tile_list.get(handle);
            if tile.depth.get() == depth
                && x >= tile.x.get()
                && x < tile.x.get() + if use_scaling { tile.xscale.get() } else { 0.into() } * tile.width.get().into()
                && y >= tile.y.get()
                && y < tile.y.get() + if use_scaling { tile.yscale.get() } else { 0.into() } * tile.height.get().into()
            {
                return Ok(tile.id.get().into())
            }
        }
        Ok((-1).into())
    }

    pub fn tile_layer_delete_at(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (depth, x, y) = expect_args!(args, [real, real, real])?;
        let use_scaling = self.gm_version == Version::GameMaker8_1; // 8.1 bugfix
        self.tile_list.remove_with(|tile| {
            tile.depth.get() == depth
                && x >= tile.x.get()
                && x < tile.x.get() + if use_scaling { tile.xscale.get() } else { 0.into() } * tile.width.get().into()
                && y >= tile.y.get()
                && y < tile.y.get() + if use_scaling { tile.yscale.get() } else { 0.into() } * tile.height.get().into()
        });
        Ok(Default::default())
    }

    pub fn tile_layer_depth(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (old_depth, new_depth) = expect_args!(args, [real, real])?;
        let mut iter_tile = self.tile_list.iter_by_drawing();
        while let Some(handle) = iter_tile.next(&self.tile_list) {
            let tile = self.tile_list.get(handle);
            if tile.depth.get() == old_depth {
                tile.depth.set(new_depth);
            }
        }
        Ok(Default::default())
    }

    pub fn surface_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (w, h) = expect_args!(args, [int, int])?;
        let surf = Surface {
            width: w as _,
            height: h as _,
            atlas_ref: match self.renderer.create_surface(w, h, true) {
                Ok(atl_ref) => atl_ref,
                Err(e) => return Err(gml::Error::FunctionError("surface_create".into(), e.into())),
            },
        };
        if let Some(id) = self.surfaces.iter().position(|x| x.is_none()) {
            self.surfaces[id] = Some(surf);
            Ok(id.into())
        } else {
            self.surfaces.push(Some(surf));
            Ok((self.surfaces.len() - 1).into())
        }
    }

    pub fn surface_create_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function surface_create_ext");
        Ok(Default::default())
    }

    pub fn surface_free(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let surf_id = expect_args!(args, [int])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            self.renderer.delete_sprite(surf.atlas_ref);
            self.surfaces[surf_id as usize] = None;
        }
        Ok(Default::default())
    }

    pub fn surface_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let surf_id = expect_args!(args, [int])?;
        Ok(self.surfaces.get_asset(surf_id).is_some().into())
    }

    pub fn surface_get_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let surf_id = expect_args!(args, [int])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) { Ok(surf.width.into()) } else { Ok((-1).into()) }
    }

    pub fn surface_get_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let surf_id = expect_args!(args, [int])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) { Ok(surf.height.into()) } else { Ok((-1).into()) }
    }

    pub fn surface_get_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let surf_id = expect_args!(args, [int])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            Ok(self.renderer.get_texture_id(&surf.atlas_ref).into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn surface_set_target(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let surf_id = expect_args!(args, [int])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            self.renderer.set_target(&surf.atlas_ref);
            self.surface_target = Some(surf_id);
        }
        Ok(Default::default())
    }

    pub fn surface_reset_target(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        // reset viewport to top left of room because lol
        self.renderer.reset_target();
        self.surface_target = None;
        Ok(Default::default())
    }

    pub fn draw_surface(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, x, y) = expect_args!(args, [any, any, any])?;
        self.draw_surface_ext(context, &[surf_id, x, y, 1.into(), 1.into(), 0.into(), 0xffffff.into(), 1.into()])
    }

    pub fn draw_surface_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, x, y, xscale, yscale, rot, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, real, int, real])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            self.renderer.draw_sprite(
                &surf.atlas_ref,
                x.into(),
                y.into(),
                xscale.into(),
                yscale.into(),
                rot.into(),
                colour,
                alpha.into(),
            );
        }
        Ok(Default::default())
    }

    pub fn draw_surface_stretched(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, x, y, w, h) = expect_args!(args, [int, any, any, real, real])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            let xscale = w / surf.width.into();
            let yscale = h / surf.height.into();
            self.draw_surface_ext(context, &[
                surf_id.into(),
                x,
                y,
                xscale.into(),
                yscale.into(),
                0.into(),
                0xffffff.into(),
                1.into(),
            ])
        } else {
            Ok(Default::default())
        }
    }

    pub fn draw_surface_stretched_ext(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, x, y, w, h, colour, alpha) = expect_args!(args, [int, any, any, real, real, any, any])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            let xscale = w / surf.width.into();
            let yscale = h / surf.height.into();
            self.draw_surface_ext(context, &[
                surf_id.into(),
                x,
                y,
                xscale.into(),
                yscale.into(),
                0.into(),
                colour,
                alpha,
            ])
        } else {
            Ok(Default::default())
        }
    }

    pub fn draw_surface_part(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, l, t, w, h, x, y) = expect_args!(args, [any, any, any, any, any, any, any])?;
        self.draw_surface_part_ext(context, &[surf_id, l, t, w, h, x, y, 1.into(), 1.into(), 0xffffff.into(), 1.into()])
    }

    pub fn draw_surface_part_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, l, t, w, h, x, y, xscale, yscale, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, int, real])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            self.renderer.draw_sprite_partial(
                &surf.atlas_ref,
                l.into(),
                t.into(),
                w.into(),
                h.into(),
                x.into(),
                y.into(),
                xscale.into(),
                yscale.into(),
                0.0,
                colour,
                alpha.into(),
            );
        }
        Ok(Default::default())
    }

    pub fn draw_surface_general(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, l, t, w, h, x, y, xscale, yscale, angle, col1, col2, col3, col4, alpha) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, real, int, int, int, int, real])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            self.renderer.draw_sprite_general(
                &surf.atlas_ref,
                l.into(),
                t.into(),
                w.into(),
                h.into(),
                x.into(),
                y.into(),
                xscale.into(),
                yscale.into(),
                angle.into(),
                col1,
                col2,
                col3,
                col4,
                alpha.into(),
                false,
            );
        }
        Ok(Default::default())
    }

    pub fn draw_surface_tiled(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, x, y) = expect_args!(args, [any, any, any])?;
        self.draw_surface_tiled_ext(context, &[surf_id, x, y, 1.into(), 1.into(), 0xFFFFFF.into(), 1.into()])
    }

    pub fn draw_surface_tiled_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, x, y, xscale, yscale, colour, alpha) =
            expect_args!(args, [int, real, real, real, real, int, real])?;
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            self.renderer.draw_sprite_tiled(
                &surf.atlas_ref,
                x.into(),
                y.into(),
                xscale.into(),
                yscale.into(),
                colour,
                alpha.into(),
                Some(self.room_width.into()),
                Some(self.room_height.into()),
            );
        }
        Ok(Default::default())
    }

    pub fn surface_save(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, fname) = expect_args!(args, [int, string])?;
        if Some(surf_id) == self.surface_target {
            self.renderer.flush_queue();
        }
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            let mut image =
                RgbaImage::from_vec(surf.width, surf.height, self.renderer.dump_sprite(&surf.atlas_ref).into())
                    .unwrap();
            asset::sprite::process_image(&mut image, false, false, true);
            match file::save_image(fname.as_ref(), image) {
                Ok(()) => Ok(Default::default()),
                Err(e) => Err(gml::Error::FunctionError("surface_save".into(), e.to_string())),
            }
        } else {
            Ok(Default::default())
        }
    }

    pub fn surface_save_part(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, fname, x, y, w, h) = expect_args!(args, [int, string, int, int, int, int])?;
        if Some(surf_id) == self.surface_target {
            self.renderer.flush_queue();
        }
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            let x = x.max(0);
            let y = y.max(0);
            let w = w.min(surf.width as i32 - x);
            let h = h.min(surf.height as i32 - y);
            let mut image =
                RgbaImage::from_vec(w as _, h as _, self.renderer.dump_sprite_part(&surf.atlas_ref, x, y, w, h).into())
                    .unwrap();
            asset::sprite::process_image(&mut image, false, false, true);
            match file::save_image(fname.as_ref(), image) {
                Ok(()) => Ok(Default::default()),
                Err(e) => Err(gml::Error::FunctionError("surface_save_part".into(), e.to_string())),
            }
        } else {
            Ok(Default::default())
        }
    }

    pub fn surface_getpixel(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function surface_getpixel");
        Ok(Default::default())
    }

    pub fn surface_copy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function surface_copy");
        Ok(Default::default())
    }

    pub fn surface_copy_part(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        println!("Called unimplemented kernel function surface_copy_part");
        Ok(Default::default())
    }

    pub fn action_path_old(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function action_path_old");
        Ok(Default::default())
    }

    pub fn action_set_sprite(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite, scale) = expect_args!(args, [int, real])?;
        let instance = self.instance_list.get(context.this);
        instance.sprite_index.set(sprite);
        instance.image_xscale.set(scale);
        instance.image_yscale.set(scale);
        Ok(Default::default())
    }

    pub fn action_draw_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_draw_font");
        Ok(Default::default())
    }

    pub fn action_draw_font_old(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function action_draw_font_old");
        Ok(Default::default())
    }

    pub fn action_fill_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_fill_color");
        Ok(Default::default())
    }

    pub fn action_line_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_line_color");
        Ok(Default::default())
    }

    pub fn action_highscore(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function action_highscore");
        Ok(Default::default())
    }

    pub fn action_move(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (dir_string, speed) = expect_args!(args, [bytes, real])?;
        let instance = self.instance_list.get(context.this);
        // dir_string is typically something like "000000100" indicating which of the 9 direction buttons were pressed.
        let bytes = dir_string.as_ref();
        if bytes.len() != 9 {
            return Err(gml::Error::FunctionError(
                "action_move".into(),
                format!("Invalid argument to action_move: {}", dir_string),
            ))
        }

        // Only invoke RNG if at least one of the options is checked, otherwise don't do anything
        if bytes.contains(&49) {
            // Call irandom until it lands on a byte that's '1' rather than '0'
            let offset = loop {
                let index = self.rand.next_int(8) as usize;
                if bytes[index] == 49 {
                    // '1'
                    break index
                }
            };

            // Handle each case separately
            let (speed, direction): (Real, Real) = match offset {
                0 => (speed, Real::from(225.0)),
                1 => (speed, Real::from(270.0)),
                2 => (speed, Real::from(315.0)),
                3 => (speed, Real::from(180.0)),
                4 => (Real::from(0.0), Real::from(0.0)),
                5 => (speed, Real::from(0.0)),
                6 => (speed, Real::from(135.0)),
                7 => (speed, Real::from(90.0)),
                8 => (speed, Real::from(45.0)),
                _ => unreachable!(),
            };
            if context.relative {
                instance.set_hspeed(direction.to_radians().cos() * speed + instance.hspeed.get());
                instance.set_vspeed(-direction.to_radians().sin() * speed + instance.vspeed.get());
            } else {
                instance.set_speed_direction(speed, direction);
            }
        }

        Ok(Default::default())
    }

    pub fn action_set_motion(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, speed) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.set_hspeed(direction.to_radians().cos() * speed + instance.hspeed.get());
            instance.set_vspeed(-direction.to_radians().sin() * speed + instance.vspeed.get());
        } else {
            instance.set_speed_direction(speed, direction);
        }
        Ok(Default::default())
    }

    pub fn action_set_hspeed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| {
            let instance = self.instance_list.get(context.this);
            if context.relative {
                instance.set_hspeed(x + instance.hspeed.get());
            } else {
                instance.set_hspeed(x);
            }
            Ok(Default::default())
        })?
    }

    pub fn action_set_vspeed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| {
            let instance = self.instance_list.get(context.this);
            if context.relative {
                instance.set_vspeed(x + instance.vspeed.get());
            } else {
                instance.set_vspeed(x);
            }
            Ok(Default::default())
        })?
    }

    pub fn action_set_gravity(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real]).map(|(direction, gravity)| {
            let instance = self.instance_list.get(context.this);
            instance.gravity.set(gravity);
            instance.gravity_direction.set(direction);
        })?;
        Ok(Default::default())
    }

    pub fn action_set_friction(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| {
            self.instance_list.get(context.this).friction.set(x);
            Ok(Default::default())
        })?
    }

    pub fn action_move_point(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, speed) = expect_args!(args, [real, real, real])?;
        let instance = self.instance_list.get(context.this);
        let speed = if context.relative { instance.speed.get() + speed } else { speed };
        let direction = (instance.y.get() - y).arctan2(x - instance.x.get()).to_degrees();
        instance.set_speed_direction(speed, direction);
        Ok(Default::default())
    }

    pub fn action_move_to(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        let (x, y) = if context.relative { (instance.x.get() + x, instance.y.get() + y) } else { (x, y) };
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);
        Ok(Default::default())
    }

    pub fn action_move_start(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        instance.x.set(instance.xstart.get());
        instance.y.set(instance.ystart.get());
        instance.bbox_is_stale.set(true);
        Ok(Default::default())
    }

    pub fn action_move_random(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (hsnap, vsnap) = expect_args!(args, [int, int])?;
        let inst = self.instance_list.get(context.this);
        let (mut left, mut right, mut top, mut bottom) = (0, self.room_width, 0, self.room_height);
        if let Some(sprite) = self
            .assets
            .sprites
            .get_asset(inst.sprite_index.get())
            .or(self.assets.sprites.get_asset(inst.mask_index.get()))
        {
            inst.update_bbox(Some(sprite));
            left = (inst.x.get() - inst.bbox_left.get().into()).round();
            right = (inst.x.get() + right.into() - inst.bbox_right.get().into()).round();
            top = (inst.y.get() - inst.bbox_top.get().into()).round();
            bottom = (inst.y.get() + bottom.into() - inst.bbox_bottom.get().into()).round();
        };
        drop(inst); // le borrow
        let (mut x, mut y) = Default::default();
        for _ in 0..100 {
            x = Real::from(self.rand.next_int((right - left - 1) as u32) + left);
            if hsnap > 0 {
                x = (x / hsnap.into()).floor() * hsnap.into();
            }
            y = Real::from(self.rand.next_int((bottom - top - 1) as u32) + top);
            if vsnap > 0 {
                y = (y / vsnap.into()).floor() * vsnap.into();
            }
            if self.place_free(context, &[x.into(), y.into()])?.is_truthy() {
                break
            }
        }
        let inst = self.instance_list.get(context.this);
        inst.x.set(x);
        inst.y.set(y);
        inst.bbox_is_stale.set(true);
        Ok(Default::default())
    }

    pub fn action_snap(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.move_snap(context, args)
    }

    pub fn action_wrap(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (horizontal, vertical) = match expect_args!(args, [int])? {
            0 => (true, false),
            1 => (false, true),
            _ => (true, true),
        };

        let instance = self.instance_list.get(context.this);
        // Get sprite width/height, as these are used to decide how far to wrap
        let (w, h) = if let Some(Some(sprite)) = self.assets.sprites.get(instance.sprite_index.get() as usize) {
            (
                Real::from(sprite.width) * instance.image_xscale.get(),
                Real::from(sprite.height) * instance.image_yscale.get(),
            )
        } else {
            (Real::from(0.0), Real::from(0.0))
        };

        if horizontal {
            let room_width = Real::from(self.room_width);
            if instance.hspeed.get() > Real::from(0.0) && instance.x.get() > room_width {
                // Wrap x right-to-left
                instance.x.set(instance.x.get() - (room_width + w));
            }
            if instance.hspeed.get() < Real::from(0.0) && instance.x.get() < Real::from(0.0) {
                // Wrap x left-to-right
                instance.x.set(instance.x.get() + (room_width + w));
            }
        }
        if vertical {
            let room_height = Real::from(self.room_height);
            if instance.vspeed.get() > Real::from(0.0) && instance.y.get() > room_height {
                // Wrap y bottom-to-top
                instance.y.set(instance.y.get() - (room_height + h));
            }
            if instance.vspeed.get() < Real::from(0.0) && instance.y.get() < Real::from(0.0) {
                // Wrap y top-to-bottom
                instance.y.set(instance.y.get() + (room_height + h));
            }
        }
        Ok(Default::default())
    }

    pub fn action_reverse_xdir(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        instance.set_hspeed(-instance.hspeed.get());
        Ok(Default::default())
    }

    pub fn action_reverse_ydir(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        instance.set_vspeed(-instance.vspeed.get());
        Ok(Default::default())
    }

    pub fn action_move_contact(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, max_distance, kind) = expect_args!(args, [any, any, int])?;
        if kind == 0 {
            self.move_contact_solid(context, &[direction, max_distance])
        } else {
            self.move_contact_all(context, &[direction, max_distance])
        }
    }

    pub fn action_bounce(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (advanced, kind) = expect_args!(args, [any, int])?;
        if kind == 0 {
            self.move_bounce_solid(context, &[advanced])
        } else {
            self.move_bounce_all(context, &[advanced])
        }
    }

    pub fn action_path(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.path_start(context, args)
    }

    pub fn action_path_end(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.path_end(context, args)
    }

    pub fn action_path_position(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let position = expect_args!(args, [real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.path_position.set(position + instance.path_position.get());
        } else {
            instance.path_position.set(position);
        }
        Ok(Default::default())
    }

    pub fn action_path_speed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let speed = expect_args!(args, [real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.path_speed.set(speed + instance.path_speed.get());
        } else {
            instance.path_speed.set(speed);
        }
        Ok(Default::default())
    }

    pub fn action_linear_step(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, step_size, checkall) = expect_args!(args, [real, real, real, bool])?;
        let instance = self.instance_list.get(context.this);
        let (x, y) = if context.relative { (instance.x.get() + x, instance.y.get() + y) } else { (x, y) };
        Ok(pathfinding::linear_step(x, y, step_size, instance, || {
            if checkall {
                self.check_collision_any(context.this).is_some()
            } else {
                self.check_collision_solid(context.this).is_some()
            }
        })
        .into())
    }

    pub fn action_potential_step(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, step_size, checkall) = expect_args!(args, [real, real, real, bool])?;
        let instance = self.instance_list.get(context.this);
        let (x, y) = if context.relative { (instance.x.get() + x, instance.y.get() + y) } else { (x, y) };
        Ok(pathfinding::potential_step(x, y, step_size, &self.potential_step_settings, instance, || {
            if checkall {
                self.check_collision_any(context.this).is_some()
            } else {
                self.check_collision_solid(context.this).is_some()
            }
        })
        .into())
    }

    pub fn action_kill_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.run_instance_event(gml::ev::DESTROY, 0, context.this, context.this, None)?;
        self.instance_list.mark_deleted(context.this);
        Ok(Default::default())
    }

    pub fn action_create_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, x, y) = expect_args!(args, [int, real, real])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            let (relative_x, relative_y) = if context.relative {
                let instance = self.instance_list.get(context.this);
                (instance.x.get(), instance.y.get())
            } else {
                (Real::from(0.0), Real::from(0.0))
            };
            self.last_instance_id += 1;
            let instance = self.instance_list.insert(Instance::new(
                self.last_instance_id,
                x + relative_x,
                y + relative_y,
                object_id,
                object,
            ));
            self.run_instance_event(gml::ev::CREATE, 0, instance, instance, None)?;
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("action_create_object".into(), format!("Invalid object ID: {}", object_id)))
        }
    }

    pub fn action_create_object_motion(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, x, y, speed, direction) = expect_args!(args, [int, real, real, real, real])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            let (relative_x, relative_y) = if context.relative {
                let instance = self.instance_list.get(context.this);
                (instance.x.get(), instance.y.get())
            } else {
                (Real::from(0.0), Real::from(0.0))
            };
            self.last_instance_id += 1;
            let instance = self.instance_list.insert(Instance::new(
                self.last_instance_id,
                x + relative_x,
                y + relative_y,
                object_id,
                object,
            ));
            self.instance_list.get(instance).set_speed_direction(speed, direction);
            self.run_instance_event(gml::ev::CREATE, 0, instance, instance, None)?;
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError(
                "action_create_object_motion".into(),
                format!("Invalid object ID: {}", object_id),
            ))
        }
    }

    pub fn action_create_object_random(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (obj1, obj2, obj3, obj4, x, y) = expect_args!(args, [int, int, int, int, real, real])?;
        let (x, y) = if context.relative {
            let instance = self.instance_list.get(context.this);
            ((instance.x.get() + x.into()).into(), (instance.y.get() + y.into()).into())
        } else {
            (x, y)
        };
        let object_ids = [obj1, obj2, obj3, obj4];
        if object_ids.iter().any(|&id| self.assets.objects.get_asset(id).is_some()) {
            let (object_id, object) = loop {
                let i = self.rand.next_int(3) as usize;
                if let Some(object) = self.assets.objects.get_asset(object_ids[i]) {
                    break (object_ids[i], object)
                }
            };
            self.last_instance_id += 1;
            let id = self.last_instance_id;
            let instance = self.instance_list.insert(Instance::new(id, x, y, object_id, object));
            self.run_instance_event(gml::ev::CREATE, 0, instance, instance, None)?;
        }
        Ok(Default::default())
    }

    pub fn action_change_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.instance_change(context, args)
    }

    pub fn action_kill_position(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [any, any])?;
        let (x, y) = if context.relative {
            let instance = self.instance_list.get(context.this);
            ((instance.x.get() + x.into()).into(), (instance.y.get() + y.into()).into())
        } else {
            (x, y)
        };
        self.position_destroy(context, &[x, y])
    }

    pub fn action_sprite_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, image_index, image_speed) = expect_args!(args, [int, real, real])?;
        let instance = self.instance_list.get(context.this);
        instance.bbox_is_stale.set(true);
        instance.sprite_index.set(sprite_id);
        instance.image_index.set(image_index);
        instance.image_speed.set(image_speed);
        Ok(Default::default())
    }

    pub fn action_sprite_transform(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (mut xsc, mut ysc, ang, mirroring) = expect_args!(args, [real, real, real, int])?;
        let instance = self.instance_list.get(context.this);
        let (hmirr, vmirr) = match mirroring {
            1 => (true, false),
            2 => (false, true),
            3 => (true, true),
            0 | _ => (false, false),
        };
        if hmirr {
            xsc = -xsc;
        }
        if vmirr {
            ysc = -ysc;
        }
        instance.image_xscale.set(xsc);
        instance.image_yscale.set(ysc);
        instance.image_angle.set(ang);
        Ok(Default::default())
    }

    pub fn action_sprite_color(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (col, alpha) = expect_args!(args, [int, real])?;
        let instance = self.instance_list.get(context.this);
        instance.image_blend.set(col);
        instance.image_alpha.set(alpha);
        Ok(Default::default())
    }

    pub fn action_sound(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sound, repeat) = expect_args!(args, [any, any])?;
        if repeat.is_truthy() { self.sound_loop(context, &[sound]) } else { self.sound_play(context, &[sound]) }
    }

    pub fn action_end_sound(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.sound_stop(context, args)
    }

    pub fn action_if_sound(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.sound_isplaying(context, args)
    }

    pub fn action_another_room(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (room_id, transition) = expect_args!(args, [int, int])?;
        self.scene_change = Some(SceneChange::Room(room_id));
        self.transition_kind = transition;
        Ok(Default::default())
    }

    pub fn action_current_room(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let transition = expect_args!(args, [int])?;
        self.scene_change = Some(SceneChange::Room(self.room_id));
        self.transition_kind = transition;
        Ok(Default::default())
    }

    pub fn action_previous_room(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let transition = expect_args!(args, [int])?;
        self.transition_kind = transition;
        self.room_goto_previous(context, &[])
    }

    pub fn action_next_room(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let transition = expect_args!(args, [int])?;
        self.transition_kind = transition;
        self.room_goto_next(context, &[])
    }

    pub fn action_if_previous_room(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.room_order.first() {
            Some(&room_id) => Ok((room_id != self.room_id).into()),
            None => Err(gml::Error::EndOfRoomOrder),
        }
    }

    pub fn action_if_next_room(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.room_order.last() {
            Some(&room_id) => Ok((room_id != self.room_id).into()),
            None => Err(gml::Error::EndOfRoomOrder),
        }
    }

    pub fn action_set_alarm(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (time, alarm) = expect_args!(args, [int, int])?;
        self.instance_list.get(context.this).alarms.borrow_mut().insert(alarm as u32, time);
        Ok(Default::default())
    }

    pub fn action_sleep(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (millis, redraw) = expect_args!(args, [any, bool])?;
        if redraw {
            self.screen_redraw(context, &[])?;
        }
        self.sleep(context, &[millis])
    }

    pub fn action_set_timeline(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (index, position) = expect_args!(args, [int, real])?;
        let instance = self.instance_list.get(context.this);
        instance.timeline_index.set(index);
        instance.timeline_position.set(position);
        instance.timeline_running.set(true);
        Ok(Default::default())
    }

    pub fn action_timeline_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (index, position, start_option, loop_option) = expect_args!(args, [int, real, int, int])?;
        let instance = self.instance_list.get(context.this);
        instance.timeline_index.set(index);
        instance.timeline_position.set(position);
        instance.timeline_running.set(start_option == 0);
        instance.timeline_loop.set(loop_option == 1);
        Ok(Default::default())
    }

    pub fn action_timeline_start(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.instance_list.get(context.this).timeline_running.set(true);
        Ok(Default::default())
    }

    pub fn action_timeline_pause(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.instance_list.get(context.this).timeline_running.set(false);
        Ok(Default::default())
    }

    pub fn action_timeline_stop(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let instance = self.instance_list.get(context.this);
        instance.timeline_position.set(Real::from(0.0));
        instance.timeline_running.set(false);
        Ok(Default::default())
    }

    pub fn action_set_timeline_position(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let position = expect_args!(args, [real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.timeline_position.set(instance.timeline_position.get() + position);
        } else {
            instance.timeline_position.set(position);
        }
        Ok(Default::default())
    }

    pub fn action_set_timeline_speed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let speed = expect_args!(args, [real])?;
        let instance = self.instance_list.get(context.this);
        if context.relative {
            instance.timeline_speed.set(instance.timeline_speed.get() + speed);
        } else {
            instance.timeline_speed.set(speed);
        }
        Ok(Default::default())
    }

    pub fn action_message(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.show_message(context, args)
    }

    pub fn action_show_info(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function action_show_info");
        Ok(Default::default())
    }

    pub fn action_show_video(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function action_show_video");
        Ok(Default::default())
    }

    pub fn action_splash_video(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function action_splash_video");
        Ok(Default::default())
    }

    pub fn action_splash_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_splash_text");
        Ok(Default::default())
    }

    pub fn action_splash_image(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_splash_image");
        Ok(Default::default())
    }

    pub fn action_splash_web(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function action_splash_web");
        Ok(Default::default())
    }

    pub fn action_splash_settings(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function action_splash_settings");
        Ok(Default::default())
    }

    pub fn action_end_game(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.game_end(context, args)
    }

    pub fn action_restart_game(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.game_restart(context, args)
    }

    pub fn action_save_game(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_save_game");
        Ok(Default::default())
    }

    pub fn action_load_game(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_load_game");
        Ok(Default::default())
    }

    pub fn action_replace_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function action_replace_sprite");
        Ok(Default::default())
    }

    pub fn action_replace_sound(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function action_replace_sound");
        Ok(Default::default())
    }

    pub fn action_replace_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function action_replace_background");
        Ok(Default::default())
    }

    pub fn action_if_empty(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        Ok((!self.action_if_collision(context, args)?.is_truthy()).into())
    }

    pub fn action_if_collision(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (mut x, mut y, collision) = expect_args!(args, [real, real, bool])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            x += instance.x.get();
            y += instance.y.get();
        }
        Ok((!if collision {
            self.place_empty(context, &[x.into(), y.into()])
        } else {
            self.place_free(context, &[x.into(), y.into()])
        }?
        .is_truthy())
        .into())
    }

    pub fn action_if(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).map(|x| x.clone())
    }

    pub fn action_if_number(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, number, comparator) = expect_args!(args, [int, int, int])?;
        if let Some(ids) = self.assets.objects.get_asset(object_id).map(|x| x.children.clone()) {
            let count = ids.borrow().iter().copied().map(|id| self.instance_list.count(id)).sum::<usize>() as i32;
            let cond = match comparator {
                1 => count < number,
                2 => count > number,
                0 | _ => count == number,
            };
            Ok(cond.into())
        } else {
            Ok(0.into())
        }
    }

    pub fn action_if_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object, mut x, mut y) = expect_args!(args, [any, real, real])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            x += instance.x.get();
            y += instance.y.get();
        }
        self.place_meeting(context, &[x.into(), y.into(), object])
    }

    pub fn action_if_question(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_if_question");
        Ok(Default::default())
    }

    pub fn action_if_dice(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let bound = expect_args!(args, [real])?;
        Ok((self.rand.next(bound.into()) < 1.0).into())
    }

    pub fn action_if_mouse(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let button = expect_args!(args, [int])?;
        let button_enum = match button {
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            _ => return Ok((self.input_manager.mouse_get_button() == 0).into()),
        };
        Ok((self.input_manager.mouse_check(button_enum) || self.input_manager.mouse_check_released(button_enum)).into())
    }

    pub fn action_if_aligned(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (xsnap, ysnap) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        Ok((((xsnap <= 0.into()) || (instance.x.get() % xsnap == 0.into()))
            && ((ysnap <= 0.into()) || (instance.y.get() % ysnap == 0.into())))
        .into())
    }

    pub fn action_execute_script(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (script_id, arg1, arg2, arg3, arg4, arg5) = expect_args!(args, [int, any, any, any, any, any])?;
        if let Some(script) = self.assets.scripts.get_asset(script_id) {
            let instructions = script.compiled.clone();

            let mut new_context = Context {
                this: context.this,
                other: context.other,
                event_action: context.event_action,
                relative: context.relative,
                event_type: context.event_type,
                event_number: context.event_number,
                event_object: context.event_object,
                arguments: [
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ],
                argument_count: 5,
                locals: DummyFieldHolder::new(),
                return_value: Default::default(),
            };
            self.execute(&instructions, &mut new_context)?;
            Ok(new_context.return_value)
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Script, script_id))
        }
    }

    pub fn action_inherited(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.event_inherited(context, args)
    }

    pub fn action_if_variable(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (lhs, rhs, comparator) = expect_args!(args, [any, any, int])?;
        let operator = match comparator {
            1 => Value::gml_lt,
            2 => Value::gml_gt,
            0 | _ => Value::gml_eq,
        };
        operator(lhs, rhs)
    }

    pub fn action_draw_variable(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (variable, mut x, mut y) = expect_args!(args, [any, real, real])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            x += instance.x.get();
            y += instance.y.get();
        }
        self.draw_text(context, &[x.into(), y.into(), variable])
    }

    pub fn action_set_score(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let score = expect_args!(args, [int])?;
        if context.relative {
            self.score += score;
        } else {
            self.score = score;
        }
        Ok(Default::default())
    }

    pub fn action_if_score(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (value, method) = expect_args!(args, [real, int])?;

        Ok(match method {
            1 => Real::from(self.score) < value,
            2 => Real::from(self.score) > value,
            0 | _ => Real::from(self.score) == value,
        }
        .into())
    }

    pub fn action_draw_score(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (mut x, mut y, caption) = expect_args!(args, [real, real, bytes])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            x += instance.x.get();
            y += instance.y.get();
        }
        self.draw_text(context, &[x.into(), y.into(), format!("{}{}", caption, self.score).into()])
    }

    pub fn action_highscore_show(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function action_highscore_show");
        Ok(Default::default())
    }

    pub fn action_highscore_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function action_highscore_clear");
        Ok(Default::default())
    }

    pub fn action_set_life(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let lives = expect_args!(args, [int])?;
        if context.relative {
            self.lives += lives;
        } else {
            self.lives = lives;
        }
        Ok(Default::default())
    }

    pub fn action_if_life(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (value, method) = expect_args!(args, [real, int])?;

        Ok(match method {
            1 => Real::from(self.lives) < value,
            2 => Real::from(self.lives) > value,
            0 | _ => Real::from(self.lives) == value,
        }
        .into())
    }

    pub fn action_draw_life(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (mut x, mut y, caption) = expect_args!(args, [real, real, bytes])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            x += instance.x.get();
            y += instance.y.get();
        }
        self.draw_text(context, &[x.into(), y.into(), format!("{}{}", caption, self.lives).into()])
    }

    pub fn action_draw_life_images(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (mut x, mut y, sprite_index) = expect_args!(args, [real, real, int])?;
        if context.relative {
            let inst = self.instance_list.get(context.this);
            x += inst.x.get();
            y += inst.y.get();
        }
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_index) {
            if let Some(atlas_ref) = sprite.get_atlas_ref(Real::from(0)) {
                for _ in 0..self.lives {
                    self.renderer.draw_sprite(atlas_ref, x.into(), y.into(), 1.0, 1.0, 0.0, 0xFFFFFF, 1.0);
                    x += sprite.width.into();
                }
            }
        }
        Ok(Default::default())
    }

    pub fn action_set_health(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let health = expect_args!(args, [real])?;
        self.health = health;
        Ok(Default::default())
    }

    pub fn action_if_health(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (value, method) = expect_args!(args, [real, int])?;

        Ok(match method {
            1 => self.health < value,
            2 => self.health > value,
            0 | _ => self.health == value,
        }
        .into())
    }

    pub fn action_draw_health(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (mut x1, mut y1, mut x2, mut y2, back_col, col) = expect_args!(args, [real, real, real, real, int, int])?;

        if context.relative {
            let instance = self.instance_list.get(context.this);
            let x = instance.x.get();
            let y = instance.y.get();
            x1 += x;
            x2 += x;
            y1 += y;
            y2 += y;
        }

        let health_ratio = f64::from(self.health / Real::from(100.0));

        use mappings::constants;
        let bar_colour = match col {
            0 => {
                // green to red (actually c_lime to c_red)
                if health_ratio > 0.5 {
                    i32::from_le_bytes([((1.0 - health_ratio) * (2.0 * 255.0)) as u8, 255, 0, 0])
                } else {
                    i32::from_le_bytes([255, (health_ratio * (2.0 * 255.0)) as u8, 0, 0])
                }
            },
            1 => {
                // white to black
                let value = (health_ratio * 255.0) as u8;
                i32::from_le_bytes([value, value, value, 0])
            },
            2 => constants::C_BLACK as i32,
            3 => constants::C_GRAY as i32,
            4 => constants::C_SILVER as i32,
            5 => constants::C_WHITE as i32,
            6 => constants::C_MAROON as i32,
            7 => constants::C_GREEN as i32,
            8 => constants::C_OLIVE as i32,
            9 => constants::C_NAVY as i32,
            10 => constants::C_PURPLE as i32,
            11 => constants::C_TEAL as i32,
            12 => constants::C_RED as i32,
            13 => constants::C_LIME as i32,
            14 => constants::C_YELLOW as i32,
            15 => constants::C_BLUE as i32,
            16 => constants::C_FUCHSIA as i32,
            17 => constants::C_AQUA as i32,
            _ => constants::C_BLACK as i32,
        };
        let back_colour = match back_col {
            0 => None,
            1 => Some(constants::C_BLACK as i32),
            2 => Some(constants::C_GRAY as i32),
            3 => Some(constants::C_SILVER as i32),
            4 => Some(constants::C_WHITE as i32),
            5 => Some(constants::C_MAROON as i32),
            6 => Some(constants::C_GREEN as i32),
            7 => Some(constants::C_OLIVE as i32),
            8 => Some(constants::C_NAVY as i32),
            9 => Some(constants::C_PURPLE as i32),
            10 => Some(constants::C_TEAL as i32),
            11 => Some(constants::C_RED as i32),
            12 => Some(constants::C_LIME as i32),
            13 => Some(constants::C_YELLOW as i32),
            14 => Some(constants::C_BLUE as i32),
            15 => Some(constants::C_FUCHSIA as i32),
            16 => Some(constants::C_AQUA as i32),
            _ => Some(constants::C_PURPLE as i32),
        };

        if let Some(colour) = back_colour {
            self.renderer.draw_rectangle(x1.into(), y1.into(), x2.into(), y2.into(), colour, self.draw_alpha.into());
            self.renderer.draw_rectangle_outline(x1.into(), y1.into(), x2.into(), y2.into(), 0, self.draw_alpha.into());
        }

        x2 = x1 + ((x2 - x1) * health_ratio.into());
        self.renderer.draw_rectangle(x1.into(), y1.into(), x2.into(), y2.into(), bar_colour, self.draw_alpha.into());
        self.renderer.draw_rectangle_outline(x1.into(), y1.into(), x2.into(), y2.into(), 0, self.draw_alpha.into());

        Ok(Default::default())
    }

    pub fn action_set_caption(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sc_show, sc_cap, lv_show, lv_cap, hl_show, hl_cap) =
            expect_args!(args, [bool, bytes, bool, bytes, bool, bytes])?;

        self.has_set_show_score = true;
        self.score_capt_d = sc_show;
        self.lives_capt_d = lv_show;
        self.health_capt_d = hl_show;

        self.score_capt = sc_cap;
        self.lives_capt = lv_cap;
        self.health_capt = hl_cap;

        Ok(Default::default())
    }

    pub fn action_partsyst_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let depth = expect_args!(args, [real])?;
        self.particles.get_dnd_system_mut().depth = depth;
        Ok(Default::default())
    }

    pub fn action_partsyst_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.particles.destroy_dnd_system();
        Ok(Default::default())
    }

    pub fn action_partsyst_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.particles.clear_dnd_system();
        Ok(Default::default())
    }

    pub fn action_parttype_create_old(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, shape, size_min, size_max, col1, col2) = expect_args!(args, [int, int, real, real, int, int])?;
        let pt = self.particles.get_dnd_type_mut(id as usize);
        pt.graphic = particle::ParticleGraphic::Shape(shape);
        pt.size_min = size_min;
        pt.size_max = size_max;
        pt.size_incr = 0.into();
        pt.size_wiggle = 0.into();
        pt.colour = particle::ParticleColour::Two(col1, col2);
        Ok(Default::default())
    }

    pub fn action_parttype_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, shape, sprite, size_min, size_max, size_incr) = expect_args!(args, [int, int, int, real, real, real])?;
        let pt = self.particles.get_dnd_type_mut(id as usize);
        pt.graphic = if self.assets.sprites.get_asset(sprite).is_none() {
            particle::ParticleGraphic::Shape(shape)
        } else {
            particle::ParticleGraphic::Sprite { sprite, animat: true, random: false, stretch: false }
        };
        pt.size_min = size_min;
        pt.size_max = size_max;
        pt.size_incr = size_incr;
        pt.size_wiggle = 0.into();
        Ok(Default::default())
    }

    pub fn action_parttype_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, changing, col1, col2, start_alpha, end_alpha) = expect_args!(args, [int, bool, int, int, real, real])?;
        let pt = self.particles.get_dnd_type_mut(id as usize);
        pt.colour = if changing {
            particle::ParticleColour::Two(col1, col2)
        } else {
            particle::ParticleColour::Mix(col1, col2)
        };
        pt.alpha1 = start_alpha;
        pt.alpha2 = (start_alpha + end_alpha) / Real::from(2.0);
        pt.alpha3 = end_alpha;
        Ok(Default::default())
    }

    pub fn action_parttype_life(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, life_min, life_max) = expect_args!(args, [int, int, int])?;
        let pt = self.particles.get_dnd_type_mut(id as usize);
        pt.life_min = life_min;
        pt.life_max = life_max;
        Ok(Default::default())
    }

    pub fn action_parttype_speed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, speed_min, speed_max, dir_min, dir_max, friction) =
            expect_args!(args, [int, real, real, real, real, real])?;
        let pt = self.particles.get_dnd_type_mut(id as usize);
        pt.speed_min = speed_min;
        pt.speed_max = speed_max;
        pt.dir_min = dir_min;
        pt.dir_max = dir_max;
        pt.speed_incr = -friction;
        Ok(Default::default())
    }

    pub fn action_parttype_gravity(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, grav_amount, grav_dir) = expect_args!(args, [int, real, real])?;
        let pt = self.particles.get_dnd_type_mut(id as usize);
        pt.grav_amount = grav_amount;
        pt.grav_dir = grav_dir;
        Ok(Default::default())
    }

    pub fn action_parttype_secondary(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, step_type, step_number, death_type, death_number) = expect_args!(args, [int, int, int, int, int])?;
        self.particles.dnd_type_secondary(
            id as usize,
            step_type as usize,
            step_number,
            death_type as usize,
            death_number,
        );
        Ok(Default::default())
    }

    pub fn action_partemit_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, shape, xmin, ymin, xmax, ymax) = expect_args!(args, [int, int, real, real, real, real])?;
        let em = self.particles.get_dnd_emitter_mut(id as usize);
        em.shape = match shape {
            1 => particle::Shape::Ellipse,
            2 => particle::Shape::Diamond,
            3 => particle::Shape::Line,
            _ => particle::Shape::Rectangle,
        };
        em.xmin = xmin;
        em.ymin = ymin;
        em.xmax = xmax;
        em.ymax = ymax;
        Ok(Default::default())
    }

    pub fn action_partemit_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        self.particles.destroy_dnd_emitter(id as usize);
        Ok(Default::default())
    }

    pub fn action_partemit_burst(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, parttype, number) = expect_args!(args, [int, int, int])?;
        self.particles.dnd_emitter_burst(id as usize, parttype as usize, number, &mut self.rand);
        Ok(Default::default())
    }

    pub fn action_partemit_stream(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, parttype, number) = expect_args!(args, [int, int, int])?;
        self.particles.dnd_emitter_stream(id as usize, parttype as usize, number);
        Ok(Default::default())
    }

    pub fn action_cd_play(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function action_cd_play");
        Ok(Default::default())
    }

    pub fn action_cd_stop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function action_cd_stop");
        Ok(Default::default())
    }

    pub fn action_cd_pause(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function action_cd_pause");
        Ok(Default::default())
    }

    pub fn action_cd_resume(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function action_cd_resume");
        Ok(Default::default())
    }

    pub fn action_cd_present(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function action_cd_present");
        Ok(Default::default())
    }

    pub fn action_cd_playing(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function action_cd_playing");
        Ok(Default::default())
    }

    pub fn action_set_cursor(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, show_window_cursor) = expect_args!(args, [int, bool])?;
        self.cursor_sprite = sprite_id;
        let cursor = if show_window_cursor {
            Cursor::default() // GM8 seems to always resets to default cursor on call of this function
        } else {
            Cursor::Invisible
        };
        self.window.set_cursor(cursor);
        Ok(Default::default())
    }

    pub fn action_webpage(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_webpage");
        Ok(Default::default())
    }

    pub fn action_draw_sprite(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, x, y, image_index) = expect_args!(args, [any, real, real, any])?;
        let instance = self.instance_list.get(context.this);
        let (x, y) = if context.relative { (x + instance.x.get(), y + instance.y.get()) } else { (x, y) };
        self.draw_sprite(context, &[sprite_id, image_index, x.into(), y.into()])
    }

    pub fn action_draw_background(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (bg_index, x, y, tiled) = expect_args!(args, [any, any, any, bool])?;
        let (x, y) = if context.relative {
            let instance = self.instance_list.get(context.this);
            ((instance.x.get() + x.into()).into(), (instance.y.get() + y.into()).into())
        } else {
            (x, y)
        };
        if tiled {
            self.draw_background_tiled(context, &[bg_index, x, y])
        } else {
            self.draw_background(context, &[bg_index, x, y])
        }
    }

    pub fn action_draw_text(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (text, mut x, mut y) = expect_args!(args, [any, real, real])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            x += instance.x.get();
            y += instance.y.get();
        }
        self.draw_text(context, &[x.into(), y.into(), text])
    }

    pub fn action_draw_text_transformed(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (text, mut x, mut y, xscale, yscale, angle) = expect_args!(args, [any, real, real, any, any, any])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            x += instance.x.get();
            y += instance.y.get();
        }
        self.draw_text_transformed(context, &[x.into(), y.into(), text, xscale, yscale, angle])
    }

    pub fn action_draw_rectangle(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, border) = expect_args!(args, [real, real, real, real, any])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            let x = instance.x.get();
            let y = instance.y.get();
            self.draw_rectangle(context, &[
                Value::from(x1 + x),
                Value::from(y1 + y),
                Value::from(x2 + x),
                Value::from(y2 + y),
                border,
            ])
        } else {
            self.draw_rectangle(context, &[Value::from(x1), Value::from(y1), Value::from(x2), Value::from(y2), border])
        }
    }

    pub fn action_draw_gradient_hor(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, col1, col2) = expect_args!(args, [any, any, any, any, any, any])?;
        let (x1, y1, x2, y2) = if context.relative {
            let instance = self.instance_list.get(context.this);
            (
                (instance.x.get() + x1.into()).into(),
                (instance.y.get() + y1.into()).into(),
                (instance.x.get() + x2.into()).into(),
                (instance.y.get() + y2.into()).into(),
            )
        } else {
            (x1, y1, x2, y2)
        };
        self.draw_rectangle_color(context, &[x1, y1, x2, y2, col1.clone(), col2.clone(), col2, col1, false.into()])
    }

    pub fn action_draw_gradient_vert(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, col1, col2) = expect_args!(args, [any, any, any, any, any, any])?;
        let (x1, y1, x2, y2) = if context.relative {
            let instance = self.instance_list.get(context.this);
            (
                (instance.x.get() + x1.into()).into(),
                (instance.y.get() + y1.into()).into(),
                (instance.x.get() + x2.into()).into(),
                (instance.y.get() + y2.into()).into(),
            )
        } else {
            (x1, y1, x2, y2)
        };
        self.draw_rectangle_color(context, &[x1, y1, x2, y2, col1.clone(), col1, col2.clone(), col2, false.into()])
    }

    pub fn action_draw_ellipse(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, outline) = expect_args!(args, [any, any, any, any, any])?;
        let (x1, y1, x2, y2) = if context.relative {
            let instance = self.instance_list.get(context.this);
            (
                (instance.x.get() + x1.into()).into(),
                (instance.y.get() + y1.into()).into(),
                (instance.x.get() + x2.into()).into(),
                (instance.y.get() + y2.into()).into(),
            )
        } else {
            (x1, y1, x2, y2)
        };
        self.draw_ellipse(context, &[x1, y1, x2, y2, outline])
    }

    pub fn action_draw_ellipse_gradient(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, col1, col2) = expect_args!(args, [any, any, any, any, any, any])?;
        let (x1, y1, x2, y2) = if context.relative {
            let instance = self.instance_list.get(context.this);
            (
                (instance.x.get() + x1.into()).into(),
                (instance.y.get() + y1.into()).into(),
                (instance.x.get() + x2.into()).into(),
                (instance.y.get() + y2.into()).into(),
            )
        } else {
            (x1, y1, x2, y2)
        };
        self.draw_ellipse_color(context, &[x1, y1, x2, y2, col1, col2, false.into()])
    }

    pub fn action_draw_line(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2) = expect_args!(args, [any, any, any, any])?;
        let (x1, y1, x2, y2) = if context.relative {
            let instance = self.instance_list.get(context.this);
            (
                (instance.x.get() + x1.into()).into(),
                (instance.y.get() + y1.into()).into(),
                (instance.x.get() + x2.into()).into(),
                (instance.y.get() + y2.into()).into(),
            )
        } else {
            (x1, y1, x2, y2)
        };
        self.draw_line(context, &[x1, y1, x2, y2])
    }

    pub fn action_draw_arrow(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, size) = expect_args!(args, [any, any, any, any, any])?;
        let (x1, y1, x2, y2) = if context.relative {
            let instance = self.instance_list.get(context.this);
            (
                (instance.x.get() + x1.into()).into(),
                (instance.y.get() + y1.into()).into(),
                (instance.x.get() + x2.into()).into(),
                (instance.y.get() + y2.into()).into(),
            )
        } else {
            (x1, y1, x2, y2)
        };
        self.draw_arrow(context, &[x1, y1, x2, y2, size])
    }

    pub fn action_color(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.draw_set_color(context, args)
    }

    pub fn action_font(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (font_id, align) = expect_args!(args, [int, int])?;
        if self.assets.fonts.get_asset(font_id).is_some() {
            self.draw_font_id = font_id;
        } else {
            self.draw_font_id = -1;
        }
        self.draw_halign = match align {
            1 => draw::Halign::Middle,
            2 => draw::Halign::Right,
            0 | _ => draw::Halign::Left,
        };
        Ok(Default::default())
    }

    pub fn action_fullscreen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function action_fullscreen");
        Ok(Default::default())
    }

    pub fn action_snapshot(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.screen_save(context, args)
    }

    pub fn action_effect(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (kind, mut x, mut y, size, col, below) = expect_args!(args, [int, real, real, int, int, bool])?;
        if context.relative {
            let instance = self.instance_list.get(context.this);
            x += instance.x.get();
            y += instance.y.get();
        }
        let kind = match kind {
            0 => particle::EffectType::Explosion,
            1 => particle::EffectType::Ring,
            2 => particle::EffectType::Ellipse,
            3 => particle::EffectType::Firework,
            4 => particle::EffectType::Smoke,
            5 => particle::EffectType::SmokeUp,
            6 => particle::EffectType::Star,
            7 => particle::EffectType::Spark,
            8 => particle::EffectType::Flare,
            9 => particle::EffectType::Cloud,
            10 => particle::EffectType::Rain,
            11 => particle::EffectType::Snow,
            _ => return Ok(Default::default()),
        };
        let size = match size {
            0 => particle::EffectSize::Small,
            2 => particle::EffectSize::Large,
            _ => particle::EffectSize::Medium,
        };
        self.particles.create_effect(
            kind,
            x,
            y,
            size,
            col,
            below,
            (Real::from(30) / self.room_speed.into()).max(1.into()),
            self.room_width,
            self.room_height,
            &mut self.rand,
        );
        Ok(Default::default())
    }

    pub fn is_real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        match expect_args!(args, [any])? {
            Value::Real(_) => Ok(gml::TRUE.into()),
            _ => Ok(gml::FALSE.into()),
        }
    }

    pub fn is_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        match expect_args!(args, [any])? {
            Value::Str(_) => Ok(gml::TRUE.into()),
            _ => Ok(gml::FALSE.into()),
        }
    }

    pub fn random(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let bound = expect_args!(args, [real])?;
        Ok(self.rand.next(bound.into()).into())
    }

    pub fn random_range(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (lower, upper) = expect_args!(args, [real, real])?;
        Ok((lower.min(upper) + Real::from(self.rand.next((upper - lower).abs().into()))).into())
    }

    pub fn irandom(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let bound = expect_args!(args, [int])?;
        Ok(self.rand.next_int(bound as _).into())
    }

    pub fn irandom_range(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (lower, upper) = expect_args!(args, [int, int])?;
        Ok((lower.min(upper) + self.rand.next_int((upper - lower).abs() as _)).into())
    }

    pub fn random_set_seed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let seed = expect_args!(args, [int])?;
        self.rand.set_seed(seed);
        Ok(Default::default())
    }

    pub fn random_get_seed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.rand.seed().into())
    }

    pub fn randomize(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.play_type {
            PlayType::Normal => self.rand.randomize(),
            PlayType::Record => {
                self.rand.randomize();
                self.stored_events.push_back(replay::Event::Randomize(self.rand.seed()));
            },
            PlayType::Replay => {
                if let Some(replay::Event::Randomize(seed)) = self.stored_events.pop_front() {
                    self.rand.set_seed(seed);
                } else {
                    return Err(gml::Error::ReplayError("randomize".into()))
                }
            },
        }
        Ok(Default::default())
    }

    pub fn abs(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.abs()))
    }

    pub fn round(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| x.round().into())
    }

    pub fn floor(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.floor()))
    }

    pub fn ceil(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.ceil()))
    }

    pub fn sign(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real])
            .map(|x| if x != 0.into() { Value::Real(x.into_inner().signum().into()) } else { 0.into() })
    }

    pub fn frac(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.fract()))
    }

    pub fn sqrt(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).and_then(|x| match x.sqrt() {
            n if !n.as_ref().is_nan() => Ok(Value::Real(n)),
            n => Err(gml::Error::FunctionError("sqrt".into(), format!("can't get square root of {}", n))),
        })
    }

    pub fn sqr(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x * x))
    }

    pub fn exp(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.exp()))
    }

    pub fn ln(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.ln()))
    }

    pub fn log2(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.log2()))
    }

    pub fn log10(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.log10()))
    }

    pub fn sin(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.sin()))
    }

    pub fn cos(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.cos()))
    }

    pub fn tan(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.tan()))
    }

    pub fn arcsin(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.arcsin()))
    }

    pub fn arccos(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.arccos()))
    }

    pub fn arctan(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.arctan()))
    }

    pub fn arctan2(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real]).map(|(y, x)| Value::Real(y.arctan2(x)))
    }

    pub fn degtorad(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.to_radians()))
    }

    pub fn radtodeg(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real]).map(|x| Value::Real(x.to_degrees()))
    }

    pub fn power(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real]).map(|(x, n)| Value::Real(x.into_inner().powf(n.into()).into()))
    }

    pub fn logn(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real]).map(|(n, x)| Value::Real(x.logn(n)))
    }

    pub fn min(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let mut min = match args.first() {
            Some(v) => v.clone(),
            None => return Ok(Default::default()),
        };

        // It works like this: check all the args left to right, buffering whichever is currently lowest.
        // Comparing Reals works as obviously expected, and comparing Strings is lexical.
        // In type mismatch, Real always beats String, however String only beats Real if the Real is above 0.
        for value in args {
            match (value, &min) {
                (Value::Real(v), Value::Real(m)) if m > v => min = Value::Real(*v),
                (Value::Real(v), Value::Str(_)) => min = Value::Real(*v),
                (Value::Str(v), Value::Real(m)) if m.into_inner() > 0.0 => min = Value::Str(v.clone()),
                (Value::Str(v), Value::Str(m)) if m > v => min = Value::Str(v.clone()),
                _ => (),
            }
        }
        Ok(min)
    }

    pub fn max(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let mut max = match args.first() {
            Some(v) => v.clone(),
            None => return Ok(Default::default()),
        };

        // See min() for an explanation.
        for value in args {
            match (value, &max) {
                (Value::Real(v), Value::Real(m)) if m < v => max = Value::Real(*v),
                (Value::Real(v), Value::Str(_)) => max = Value::Real(*v),
                (Value::Str(v), Value::Real(m)) if m.into_inner() < 0.0 => max = Value::Str(v.clone()),
                (Value::Str(v), Value::Str(m)) if m < v => max = Value::Str(v.clone()),
                _ => (),
            }
        }
        Ok(max)
    }

    pub fn min3(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.min(context, args)
    }

    pub fn max3(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.max(context, args)
    }

    pub fn mean(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        if !args.is_empty() {
            Ok(Value::from(args.iter().cloned().map(Real::from).sum::<Real>() / Real::from(args.len() as f64)))
        } else {
            Ok(Default::default())
        }
    }

    pub fn median(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        Ok(args
            .iter()
            .cloned()
            .find(|v| {
                let v = Real::from(v.clone());
                let mut less = 0.0;
                let mut less_eq = 0.0;
                for arg in args.iter().cloned() {
                    let arg = Real::from(arg);
                    if arg <= v {
                        less_eq += 1.0;
                        if arg != v {
                            less += 1.0;
                        }
                    }
                }
                less < args.len() as f64 / 2.0 && less_eq >= args.len() as f64 / 2.0
            })
            .unwrap_or_default())
    }

    pub fn choose(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        match args.len().checked_sub(1) {
            Some(i) => Ok(args[self.rand.next_int(i as _) as usize].clone()),
            None => Ok(Default::default()),
        }
    }

    pub fn clamp(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [real, real, real]).map(|(n, lo, hi)| Value::Real(n.max(lo).min(hi)))
    }

    pub fn lerp(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (low, high, amount) = expect_args!(args, [real, real, real])?;
        Ok(Value::from(((high - low) * amount) + low))
    }

    pub fn real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).and_then(|v| match v {
            r @ Value::Real(_) => Ok(r),
            Value::Str(s) => match self.decode_str(s.as_ref()).trim() {
                x if x.len() == 0 => Ok(Default::default()),
                x => match x.parse::<f64>() {
                    Ok(r) => Ok(r.into()),
                    Err(e) => Err(gml::Error::FunctionError("real".into(), format!("can't convert {} - {}", s, e))),
                },
            },
        })
    }

    pub fn string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).map(|v| v.repr().into())
    }

    pub fn string_format(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (val, mut tot, mut dec) = expect_args!(args, [any, int, int])?;
        match val {
            Value::Str(_) => Ok(val),
            Value::Real(mut x) => {
                dec = dec.min(18);
                tot = tot.max(0);
                if dec < 0 {
                    // Very strange behaviour here but I swear it's accurate
                    let power = Real::from(10f64.powi(-dec));
                    x = Real::from((x / power).round()) * power;
                    dec = 18;
                }
                Ok(format!("{num:>width$.prec$}", num = x, prec = dec as usize, width = tot as usize).into())
            },
        }
    }

    pub fn chr(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // TODO: use font to decode if not sprite font
        self.ansi_char(context, args)
    }

    pub fn ansi_char(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [int]).map(|x| vec![x as u8].into())
    }

    pub fn ord(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [bytes]).map(|s| s.as_ref().get(0).copied().map(f64::from).unwrap_or_default().into())
    }

    pub fn string_length(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let string = expect_args!(args, [bytes])?;
        match self.gm_version {
            Version::GameMaker8_0 => Ok(Value::Real((string.as_ref().len() as f64).into())),
            Version::GameMaker8_1 => Ok(Value::Real((self.decode_str(string.as_ref()).chars().count() as f64).into())),
        }
    }

    pub fn string_byte_length(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [bytes]).map(|s| Value::Real((s.as_ref().len() as f64).into()))
    }

    pub fn string_byte_at(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // NOTE: The gamemaker 8 runner instead of defaulting to 0 just reads any memory address. LOL
        // We don't do this, unsurprisingly.
        expect_args!(args, [bytes, int]).map(|(s, ix)| {
            Value::Real((s.as_ref().get((ix as isize - 1).max(0) as usize).copied().unwrap_or_default() as f64).into())
        })
    }

    pub fn string_pos(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // TODO: bytes-ify
        expect_args!(args, [string, string]).map(|(ss, s)| {
            Value::Real(Real::from(s.as_ref().find(ss.as_ref()).map(|p| p + 1).unwrap_or_default() as f64))
        })
    }

    pub fn string_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (s, start, len) = expect_args!(args, [bytes, int, int])?;
        let start = (start as isize - 1).max(0) as usize;
        let len = len.max(0) as usize;
        let s = s.as_ref();
        let (start, end) = match self.gm_version {
            Version::GameMaker8_0 => {
                let end = (start + len).min(s.len());
                (start, end)
            },
            Version::GameMaker8_1 => {
                let s = self.decode_str(s);
                let start = s.char_indices().nth(start).map_or(0, |(i, _)| i);
                let sub = s.get(start..).unwrap_or("");
                let len = sub.char_indices().nth(len).map_or(sub.len(), |(i, _)| i);
                (start, start + len)
            },
        };
        Ok(Value::from(s.get(start..end).unwrap_or(b"")))
    }

    pub fn string_char_at(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (string, pos) = expect_args!(args, [bytes, int])?;
        match self.gm_version {
            Version::GameMaker8_0 => {
                Ok(string.as_ref().get((pos as isize - 1).max(0) as usize).map_or("".into(), |ch| vec![*ch].into()))
            },
            Version::GameMaker8_1 => Ok(Value::Str(
                self.decode_str(string.as_ref())
                    .chars()
                    .nth((pos as isize - 1).max(0) as usize)
                    .map_or("".to_string().into(), |ch| ch.to_string().into()),
            )),
        }
    }

    pub fn string_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (s, start, len) = expect_args!(args, [bytes, int, int])?;
        let start = (start as isize - 1).max(0) as usize;
        let len = len.max(0) as usize;
        let s = s.as_ref();
        let (start, end) = match self.gm_version {
            Version::GameMaker8_0 => {
                let end = (start + len).min(s.len());
                (start, end)
            },
            Version::GameMaker8_1 => {
                let s = self.decode_str(s);
                let start = s.char_indices().nth(start).map_or(0, |(i, _)| i);
                let sub = s.get(start..).unwrap_or("");
                let len = sub.char_indices().nth(len).map_or(sub.len(), |(i, _)| i);
                (start, start + len)
            },
        };
        Ok(s[..start].iter().chain(&s[end..]).copied().collect::<Vec<_>>().into())
    }

    pub fn string_insert(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // string_insert doesn't care about UTF-8
        expect_args!(args, [bytes, bytes, int]).map(|(ss, s, ix)| {
            let ix = ((ix as isize - 1).max(0) as usize).min(s.as_ref().len());
            s.as_ref()[..ix].iter().chain(ss.as_ref()).chain(&s.as_ref()[ix..]).copied().collect::<Vec<_>>().into()
        })
    }

    pub fn string_lower(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // TODO: bytes-ify
        expect_args!(args, [string])
            .map(|s| Value::Str(s.as_ref().chars().map(|ch| ch.to_ascii_lowercase()).collect::<String>().into()))
    }

    pub fn string_upper(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // TODO: bytes-ify
        expect_args!(args, [string])
            .map(|s| Value::Str(s.as_ref().chars().map(|ch| ch.to_ascii_uppercase()).collect::<String>().into()))
    }

    pub fn string_repeat(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [bytes, real]).map(|(s, n)| Value::Str(s.as_ref().repeat(n.into_inner() as usize).into()))
    }

    pub fn string_letters(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string])
            .map(|s| Value::Str(s.as_ref().chars().filter(|ch| ch.is_ascii_alphabetic()).collect::<String>().into()))
    }

    pub fn string_digits(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string])
            .map(|s| Value::Str(s.as_ref().chars().filter(|ch| ch.is_ascii_digit()).collect::<String>().into()))
    }

    pub fn string_lettersdigits(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string])
            .map(|s| Value::Str(s.as_ref().chars().filter(|ch| ch.is_ascii_alphanumeric()).collect::<String>().into()))
    }

    pub fn string_replace(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // TODO: bytes-ify
        expect_args!(args, [string, string, string])
            .map(|(s, x, y)| Value::Str(s.as_ref().replacen(x.as_ref(), y.as_ref(), 1).into()))
    }

    pub fn string_replace_all(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // TODO: bytes-ify
        expect_args!(args, [string, string, string])
            .map(|(s, x, y)| Value::Str(s.as_ref().replace(x.as_ref(), y.as_ref()).into()))
    }

    pub fn string_count(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [string, string])
            .map(|(ss, s)| Value::Real(Real::from(s.as_ref().matches(ss.as_ref()).count() as f64)))
    }

    pub fn dot_product(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2) = expect_args!(args, [real, real, real, real])?;
        let l1 = Real::from(x1.into_inner().hypot(y1.into_inner()));
        let l2 = Real::from(x2.into_inner().hypot(y2.into_inner()));
        let (x1, y1) = (x1 / l1, y1 / l1);
        let (x2, y2) = (x2 / l2, y2 / l2);
        Ok((x1 * x2 + y1 * y2).into())
    }

    pub fn dot_product_3d(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2) = expect_args!(args, [real, real, real, real, real, real])?;
        let l1 = (x1 * x1 + y1 * y1 + z1 * z1).sqrt();
        let l2 = (x2 * x2 + y2 * y2 + z2 * z2).sqrt();
        let (x1, y1, z1) = (x1 / l1, y1 / l1, z1 / l1);
        let (x2, y2, z2) = (x2 / l2, y2 / l2, z2 / l2);
        Ok((x1 * x2 + y1 * y2 + z1 * z2).into())
    }

    pub fn point_distance_3d(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2) = expect_args!(args, [real, real, real, real, real, real])?;
        let xdist = x2 - x1;
        let ydist = y2 - y1;
        let zdist = z2 - z1;
        Ok((xdist * xdist + ydist * ydist + zdist * zdist).sqrt().into())
    }

    pub fn point_distance(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2) = expect_args!(args, [real, real, real, real])?;
        let xdist = x2 - x1;
        let ydist = y2 - y1;
        Ok((xdist * xdist + ydist * ydist).sqrt().into())
    }

    pub fn point_direction(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2) = expect_args!(args, [real, real, real, real])?;
        Ok((y1 - y2).arctan2(x2 - x1).to_degrees().rem_euclid(360.into()).into())
    }

    pub fn lengthdir_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (len, dir) = expect_args!(args, [real, real])?;
        Ok((dir.to_radians().cos() * len).into())
    }

    pub fn lengthdir_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (len, dir) = expect_args!(args, [real, real])?;
        Ok((dir.to_radians().sin() * -len).into())
    }

    pub fn move_random(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function move_random");
        Ok(Default::default())
    }

    pub fn place_free(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [real, real])?;

        // Set self's position to the new coordinates
        let instance = self.instance_list.get(context.this);
        let old_x = instance.x.get();
        let old_y = instance.y.get();
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);

        // Check collision with any solids
        let free = self.check_collision_solid(context.this).is_none();

        // Move self back to where it was
        instance.x.set(old_x);
        instance.y.set(old_y);
        instance.bbox_is_stale.set(true);

        Ok(free.into())
    }

    pub fn place_empty(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [real, real])?;

        // Set self's position to the new coordinates
        let instance = self.instance_list.get(context.this);
        let old_x = instance.x.get();
        let old_y = instance.y.get();
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);

        // Check collision with any instance
        let empty = self.check_collision_any(context.this).is_none();

        // Move self back to where it was
        instance.x.set(old_x);
        instance.y.set(old_y);
        instance.bbox_is_stale.set(true);

        Ok(empty.into())
    }

    pub fn place_meeting(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, obj) = expect_args!(args, [real, real, int])?;

        // Set self's position to the new coordinates
        let instance = self.instance_list.get(context.this);
        let old_x = instance.x.get();
        let old_y = instance.y.get();
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);

        // Check collision with target
        let collision = match obj {
            gml::SELF => false,
            gml::OTHER => self.check_collision(context.this, context.other),
            obj => self.find_instance_with(obj, |handle| self.check_collision(context.this, handle)).is_some(),
        };

        // Move self back to where it was
        instance.x.set(old_x);
        instance.y.set(old_y);
        instance.bbox_is_stale.set(true);

        Ok(collision.into())
    }

    pub fn place_snapped(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function place_snapped");
        Ok(Default::default())
    }

    pub fn move_snap(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (hsnap, vsnap) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        instance.x.set(Real::from((instance.x.get() / hsnap).round()) * hsnap);
        instance.y.set(Real::from((instance.y.get() / vsnap).round()) * vsnap);
        instance.bbox_is_stale.set(true);
        Ok(Default::default())
    }

    pub fn move_towards_point(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, speed) = expect_args!(args, [real, real, real])?;
        let instance = self.instance_list.get(context.this);
        let direction = (instance.y.get() - y).arctan2(x - instance.x.get()).to_degrees();
        instance.set_speed_direction(speed, direction);
        Ok(Default::default())
    }

    pub fn move_contact(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let direction = expect_args!(args, [any])?;
        self.move_contact_all(context, &[direction, (-1).into()])
    }

    pub fn move_contact_solid(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, max_distance) = expect_args!(args, [real, int])?;
        let max_distance = if max_distance > 0 {
            max_distance
        } else {
            1000 // GML default
        };

        // Figure out how far we're going to step in x and y between each check
        let step_x = direction.to_radians().cos();
        let step_y = -direction.to_radians().sin();

        // Check if we're already colliding with a solid, do nothing if so
        if self.check_collision_solid(context.this).is_none() {
            let instance = self.instance_list.get(context.this);
            for _ in 0..max_distance {
                // Step forward, but back up old coordinates
                let old_x = instance.x.get();
                let old_y = instance.y.get();
                instance.x.set(instance.x.get() + step_x);
                instance.y.set(instance.y.get() + step_y);
                instance.bbox_is_stale.set(true);

                // Check if we're colliding with a solid now
                if self.check_collision_solid(context.this).is_some() {
                    // Move self back to where it was, then exit
                    instance.x.set(old_x);
                    instance.y.set(old_y);
                    instance.bbox_is_stale.set(true);
                    break
                }
            }
        }

        Ok(Default::default())
    }

    pub fn move_contact_all(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, max_distance) = expect_args!(args, [real, int])?;
        let max_distance = if max_distance > 0 {
            max_distance
        } else {
            1000 // GML default
        };

        // Figure out how far we're going to step in x and y between each check
        let step_x = direction.to_radians().cos();
        let step_y = -direction.to_radians().sin();

        // Check if we're already colliding with another instance, do nothing if so
        if self.check_collision_any(context.this).is_none() {
            let instance = self.instance_list.get(context.this);
            for _ in 0..max_distance {
                // Step forward, but back up old coordinates
                let old_x = instance.x.get();
                let old_y = instance.y.get();
                instance.x.set(instance.x.get() + step_x);
                instance.y.set(instance.y.get() + step_y);
                instance.bbox_is_stale.set(true);

                // Check if we're colliding with another instance now
                if self.check_collision_any(context.this).is_some() {
                    // Move self back to where it was, then exit
                    instance.x.set(old_x);
                    instance.y.set(old_y);
                    instance.bbox_is_stale.set(true);
                    break
                }
            }
        }

        Ok(Default::default())
    }

    pub fn move_outside_solid(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, max_distance) = expect_args!(args, [real, int])?;
        let max_distance = if max_distance > 0 {
            max_distance
        } else {
            1000 // GML default
        };

        // Figure out how far we're going to step in x and y between each check
        let step_x = direction.to_radians().cos();
        let step_y = -direction.to_radians().sin();

        // Check if we're already outside all solids, do nothing if so
        if self.check_collision_solid(context.this).is_some() {
            let instance = self.instance_list.get(context.this);
            for _ in 0..max_distance {
                // Step forward
                instance.x.set(instance.x.get() + step_x);
                instance.y.set(instance.y.get() + step_y);
                instance.bbox_is_stale.set(true);

                // Check if we're outside all solids now
                if self.check_collision_solid(context.this).is_none() {
                    // Outside a solid, exit
                    break
                }
            }
        }

        Ok(Default::default())
    }

    pub fn move_outside_all(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, max_distance) = expect_args!(args, [real, int])?;
        let max_distance = if max_distance > 0 {
            max_distance
        } else {
            1000 // GML default
        };

        // Figure out how far we're going to step in x and y between each check
        let step_x = direction.to_radians().cos();
        let step_y = -direction.to_radians().sin();

        // Check if we're already not colliding with anything, do nothing if so
        if self.check_collision_any(context.this).is_some() {
            let instance = self.instance_list.get(context.this);
            for _ in 0..max_distance {
                // Step forward
                instance.x.set(instance.x.get() + step_x);
                instance.y.set(instance.y.get() + step_y);
                instance.bbox_is_stale.set(true);

                // Check if we're not colliding with anything now
                if self.check_collision_any(context.this).is_none() {
                    // Outside a solid, exit
                    break
                }
            }
        }

        Ok(Default::default())
    }

    pub fn move_bounce(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.move_bounce_solid(context, args)
    }

    pub fn move_bounce_solid(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let advanced = expect_args!(args, [int])?;
        if advanced == 1 {
            self.bounce_advanced(context.this, true);
        } else {
            self.bounce(context.this, true);
        }
        Ok(Default::default())
    }

    pub fn move_bounce_all(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let advanced = expect_args!(args, [int])?;
        if advanced == 1 {
            self.bounce_advanced(context.this, false);
        } else {
            self.bounce(context.this, false);
        }
        Ok(Default::default())
    }

    pub fn move_wrap(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (horizontal_wrap, vertical_wrap, margin) = expect_args!(args, [bool, bool, real])?;
        let instance = self.instance_list.get(context.this);

        let mut update_bbox = false;

        if horizontal_wrap {
            let instance_x = instance.x.get();

            if instance_x < -margin {
                instance.x.set(Real::from(self.room_width) + instance_x + Real::from(2) * margin);
                update_bbox = true;
            }
            if instance_x > Real::from(self.room_width) + margin {
                instance.x.set(instance_x - Real::from(self.room_width) - Real::from(2) * margin);
                update_bbox = true;
            }
        }
        if vertical_wrap {
            let instance_y = instance.y.get();
            if instance_y < -margin {
                instance.y.set(Real::from(self.room_height) + instance_y + Real::from(2) * margin);
                update_bbox = true;
            }
            if instance_y > Real::from(self.room_height) + margin {
                instance.y.set(instance_y - Real::from(self.room_height) - Real::from(2) * margin);
                update_bbox = true;
            }
        }
        if update_bbox {
            instance.bbox_is_stale.set(true);
        }
        Ok(Default::default())
    }

    pub fn motion_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, speed) = expect_args!(args, [real, real])?;
        self.instance_list.get(context.this).set_speed_direction(speed, direction);
        Ok(Default::default())
    }

    pub fn motion_add(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (direction, speed) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);
        let hspeed = direction.to_radians().cos() * speed;
        let vspeed = -direction.to_radians().sin() * speed;
        instance.set_hvspeed(instance.hspeed.get() + hspeed, instance.vspeed.get() + vspeed);
        Ok(Default::default())
    }

    pub fn distance_to_point(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [real, real])?;
        let instance = self.instance_list.get(context.this);

        let sprite = self.get_instance_mask_sprite(context.this);
        instance.update_bbox(sprite);

        let distance_x = if x < instance.bbox_left.get().into() {
            x - instance.bbox_left.get().into()
        } else if x > instance.bbox_right.get().into() {
            x - instance.bbox_right.get().into()
        } else {
            0.into()
        };

        let distance_y = if y < instance.bbox_top.get().into() {
            y - instance.bbox_top.get().into()
        } else if y > instance.bbox_bottom.get().into() {
            y - instance.bbox_bottom.get().into()
        } else {
            0.into()
        };

        Ok(distance_x.into_inner().hypot(distance_y.into_inner()).into())
    }

    pub fn distance_to_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;

        // Helper fn: distance between two instances (with a function name that's really hard to say quickly)
        fn instance_distance(inst1: &Instance, inst2: &Instance) -> f64 {
            let distance_x = if inst1.bbox_left.get() > inst2.bbox_right.get() {
                inst1.bbox_left.get() - inst2.bbox_right.get()
            } else if inst2.bbox_left.get() > inst1.bbox_right.get() {
                inst2.bbox_left.get() - inst1.bbox_right.get()
            } else {
                0
            };

            let distance_y = if inst1.bbox_top.get() > inst2.bbox_bottom.get() {
                inst1.bbox_top.get() - inst2.bbox_bottom.get()
            } else if inst2.bbox_top.get() > inst1.bbox_bottom.get() {
                inst2.bbox_top.get() - inst1.bbox_bottom.get()
            } else {
                0
            };

            match (distance_x, distance_y) {
                (0, 0) => 0.0,
                (x, 0) => x.into(),
                (0, y) => y.into(),
                (x, y) => f64::from(x.pow(2) + y.pow(2)).sqrt(),
            }
        }

        let sprite = self.get_instance_mask_sprite(context.this);
        let this = self.instance_list.get(context.this);
        this.update_bbox(sprite);

        Ok(match object_id {
            gml::SELF => 0.0,
            gml::OTHER => {
                let sprite = self.get_instance_mask_sprite(context.other);
                let other = self.instance_list.get(context.other);
                other.update_bbox(sprite);
                instance_distance(this, other)
            },
            gml::ALL => {
                let mut closest = 1000000.0; // GML default
                let this = this;
                let mut iter = self.instance_list.iter_by_insertion();
                while let Some(other) = iter.next(&self.instance_list) {
                    let sprite = self.get_instance_mask_sprite(other);
                    let other = self.instance_list.get(other);
                    other.update_bbox(sprite);
                    let dist = instance_distance(this, other);
                    if dist < closest {
                        closest = dist;
                    }
                }
                closest
            },
            object_id if object_id <= 100000 => {
                if let Some(ids) = self.assets.objects.get_asset(object_id).map(|x| x.children.clone()) {
                    let mut closest = 1000000.0; // GML default
                    let this = this;
                    let mut iter = self.instance_list.iter_by_identity(ids);
                    while let Some(other) = iter.next(&self.instance_list) {
                        let sprite = self.get_instance_mask_sprite(other);
                        let other = self.instance_list.get(other);
                        other.update_bbox(sprite);
                        let dist = instance_distance(this, other);
                        if dist < closest {
                            closest = dist;
                        }
                    }
                    closest
                } else {
                    1000000.0 // GML default
                }
            },
            instance_id => {
                match self.instance_list.get_by_instid(instance_id) {
                    Some(handle) => {
                        let sprite = self.get_instance_mask_sprite(handle);
                        let other = self.instance_list.get(handle);
                        other.update_bbox(sprite);
                        instance_distance(this, other)
                    },
                    None => 1000000.0, // Again, GML default
                }
            },
        }
        .into())
    }

    pub fn path_start(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, speed, end_action, absolute) = expect_args!(args, [int, real, int, bool])?;
        let instance = self.instance_list.get(context.this);
        instance.path_index.set(path_id);
        instance.path_speed.set(speed);
        instance.path_endaction.set(end_action);
        instance.path_position.set(Real::from(0.0));
        if absolute {
            if let Some(path_start) = self.assets.paths.get_asset(path_id).map(|x| x.start) {
                instance.path_xstart.set(path_start.x);
                instance.path_ystart.set(path_start.y);
                instance.path_pointspeed.set(path_start.speed);
            } else {
                return Err(gml::Error::NonexistentAsset(asset::Type::Path, path_id))
            }
        } else {
            instance.path_xstart.set(instance.x.get());
            instance.path_ystart.set(instance.y.get());
        }
        Ok(Default::default())
    }

    pub fn path_end(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.instance_list.get(context.this).path_index.set(-1);
        Ok(Default::default())
    }

    pub fn mp_linear_step(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, step_size, checkall) = expect_args!(args, [real, real, real, bool])?;
        Ok(pathfinding::linear_step(x, y, step_size, self.instance_list.get(context.this), || {
            if checkall {
                self.check_collision_any(context.this).is_some()
            } else {
                self.check_collision_solid(context.this).is_some()
            }
        })
        .into())
    }

    pub fn mp_linear_path(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, xg, yg, step_size, checkall) = expect_args!(args, [int, real, real, real, bool])?;
        // i apologize for this code
        if self.assets.paths.get_asset_mut(path_id).is_some() {
            let mut path = self.assets.paths[path_id as usize].take().unwrap();
            let inst = self.instance_list.get(context.this);
            pathfinding::make_path(inst, &mut path, |inst| {
                let (old_x, old_y) = (inst.x.get(), inst.y.get());
                if pathfinding::linear_step(xg, yg, step_size, inst, || {
                    if checkall {
                        self.check_collision_any(context.this).is_some()
                    } else {
                        self.check_collision_solid(context.this).is_some()
                    }
                }) {
                    pathfinding::PathGenResult::Done
                } else if inst.x.get() == old_x && inst.y.get() == old_y {
                    pathfinding::PathGenResult::Failed
                } else {
                    pathfinding::PathGenResult::NotDone
                }
            });
            self.assets.paths[path_id as usize] = Some(path);
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Path, path_id))
        }
    }

    pub fn mp_linear_step_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, step_size, obj) = expect_args!(args, [real, real, real, int])?;
        Ok(pathfinding::linear_step(x, y, step_size, self.instance_list.get(context.this), || match obj {
            gml::SELF => false,
            gml::OTHER => self.check_collision(context.this, context.other),
            obj => self.find_instance_with(obj, |handle| self.check_collision(context.this, handle)).is_some(),
        })
        .into())
    }

    pub fn mp_linear_path_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function mp_linear_path_object");
        Ok(Default::default())
    }

    pub fn mp_potential_settings(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (max_rotation, rotate_step, check_distance, rotate_on_spot) = expect_args!(args, [real, real, real, bool])?;
        self.potential_step_settings = pathfinding::PotentialStepSettings {
            max_rotation,
            rotate_step,
            check_distance,
            rotate_on_spot: rotate_on_spot,
        };
        Ok(Default::default())
    }

    pub fn mp_potential_step(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, step_size, checkall) = expect_args!(args, [real, real, real, bool])?;
        Ok(pathfinding::potential_step(
            x,
            y,
            step_size,
            &self.potential_step_settings,
            self.instance_list.get(context.this),
            || {
                if checkall {
                    self.check_collision_any(context.this).is_some()
                } else {
                    self.check_collision_solid(context.this).is_some()
                }
            },
        )
        .into())
    }

    pub fn mp_potential_path(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function mp_potential_path");
        Ok(Default::default())
    }

    pub fn mp_potential_step_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, step_size, obj) = expect_args!(args, [real, real, real, int])?;
        Ok(pathfinding::potential_step(
            x,
            y,
            step_size,
            &self.potential_step_settings,
            self.instance_list.get(context.this),
            || match obj {
                gml::SELF => false,
                gml::OTHER => self.check_collision(context.this, context.other),
                obj => self.find_instance_with(obj, |handle| self.check_collision(context.this, handle)).is_some(),
            },
        )
        .into())
    }

    pub fn mp_potential_path_object(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function mp_potential_path_object");
        Ok(Default::default())
    }

    pub fn mp_grid_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function mp_grid_create");
        Ok(Default::default())
    }

    pub fn mp_grid_destroy(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mp_grid_destroy");
        Ok(Default::default())
    }

    pub fn mp_grid_clear_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mp_grid_clear_all");
        Ok(Default::default())
    }

    pub fn mp_grid_clear_cell(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function mp_grid_clear_cell");
        Ok(Default::default())
    }

    pub fn mp_grid_clear_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function mp_grid_clear_rectangle");
        Ok(Default::default())
    }

    pub fn mp_grid_add_cell(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function mp_grid_add_cell");
        Ok(Default::default())
    }

    pub fn mp_grid_add_rectangle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function mp_grid_add_rectangle");
        Ok(Default::default())
    }

    pub fn mp_grid_add_instances(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function mp_grid_add_instances");
        Ok(Default::default())
    }

    pub fn mp_grid_path(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        println!("Called unimplemented kernel function mp_grid_path");
        Ok(Default::default())
    }

    pub fn mp_grid_draw(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mp_grid_draw");
        Ok(Default::default())
    }

    pub fn collision_point(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, object_id, precise, exclude_self) = expect_args!(args, [int, int, int, bool, bool])?;
        match self.find_instance_with(object_id, |handle| {
            (!exclude_self || handle != context.this) && self.check_collision_point(handle, x, y, precise)
        }) {
            Some(handle) => Ok(self.instance_list.get(handle).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn collision_rectangle(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, object_id, precise, exclude_self) =
            expect_args!(args, [int, int, int, int, int, bool, bool])?;
        match self.find_instance_with(object_id, |handle| {
            (!exclude_self || handle != context.this) && self.check_collision_rectangle(handle, x1, y1, x2, y2, precise)
        }) {
            Some(handle) => Ok(self.instance_list.get(handle).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn collision_circle(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, r, object_id, precise, exclude_self) = expect_args!(args, [real, real, real, int, bool, bool])?;
        match self.find_instance_with(object_id, |handle| {
            (!exclude_self || handle != context.this)
                && self.check_collision_ellipse(handle, x - r, y - r, x + r, y + r, precise)
        }) {
            Some(handle) => Ok(self.instance_list.get(handle).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn collision_ellipse(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, object_id, precise, exclude_self) =
            expect_args!(args, [real, real, real, real, int, bool, bool])?;
        match self.find_instance_with(object_id, |handle| {
            (!exclude_self || handle != context.this) && self.check_collision_ellipse(handle, x1, y1, x2, y2, precise)
        }) {
            Some(handle) => Ok(self.instance_list.get(handle).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn collision_line(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, x2, y2, object_id, precise, exclude_self) =
            expect_args!(args, [real, real, real, real, int, bool, bool])?;
        match self.find_instance_with(object_id, |handle| {
            (!exclude_self || handle != context.this) && self.check_collision_line(handle, x1, y1, x2, y2, precise)
        }) {
            Some(handle) => Ok(self.instance_list.get(handle).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn instance_find(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (obj, n) = expect_args!(args, [int, int])?;
        if n < 0 {
            return Ok(gml::NOONE.into())
        }
        let handle = match obj {
            gml::ALL => {
                let mut iter = self.instance_list.iter_by_insertion();
                (0..n + 1).filter_map(|_| iter.next(&self.instance_list)).nth(n as usize)
            },
            _ if obj < 0 => None,
            obj if obj < 100000 => {
                if let Some(ids) = self.assets.objects.get_asset(obj).map(|x| x.children.clone()) {
                    let mut iter = self.instance_list.iter_by_identity(ids);
                    (0..n + 1).filter_map(|_| iter.next(&self.instance_list)).nth(n as usize)
                } else {
                    None
                }
            },
            inst_id => {
                if n != 0 {
                    None
                } else {
                    self.instance_list
                        .get_by_instid(inst_id)
                        .filter(|h| self.instance_list.get(*h).state.get() == InstanceState::Active)
                }
            },
        };
        Ok(handle.map(|h| self.instance_list.get(h).id.get()).unwrap_or(gml::NOONE).into())
    }

    pub fn instance_exists(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let obj = expect_args!(args, [int])?;
        let exists = match obj {
            gml::SELF => self.instance_list.get(context.this).state.get() == InstanceState::Active,
            gml::OTHER => self.instance_list.get(context.other).state.get() == InstanceState::Active,
            gml::ALL => self.instance_list.any_active(),
            obj if obj <= 100000 => {
                if let Some(object) = self.assets.objects.get_asset(obj) {
                    object.children.borrow().iter().any(|&obj| self.instance_list.count(obj) != 0)
                } else {
                    false
                }
            },
            _ => self.instance_list.get_by_instid(obj).is_some(),
        };
        Ok(exists.into())
    }

    pub fn instance_number(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        let number = match object_id {
            gml::SELF => {
                if self.instance_list.get(context.this).state.get() == InstanceState::Active {
                    1
                } else {
                    0
                }
            },
            gml::OTHER => {
                if self.instance_list.get(context.other).state.get() == InstanceState::Active {
                    1
                } else {
                    0
                }
            },
            gml::ALL => self.instance_list.count_all_active(),
            obj if obj <= 100000 => {
                if let Some(object) = self.assets.objects.get_asset(obj) {
                    object.children.borrow().iter().map(|&obj| self.instance_list.count(obj)).sum()
                } else {
                    0
                }
            },
            inst_id => {
                if self.instance_list.get_by_instid(inst_id).is_some() {
                    1
                } else {
                    0
                }
            },
        };
        Ok(number.into())
    }

    pub fn instance_position(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, object_id) = expect_args!(args, [int, int, int])?;
        match self.find_instance_with(object_id, |handle| self.check_collision_point(handle, x, y, true)) {
            Some(handle) => Ok(self.instance_list.get(handle).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn instance_nearest(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, obj) = expect_args!(args, [real, real, int])?;
        // Check collision with target
        let nearest = match obj {
            gml::ALL => {
                // Target is all objects
                let mut iter = self.instance_list.iter_by_insertion();
                let mut maxdist = Real::from(10000000000.0); // GML default
                let mut nearest = None;
                loop {
                    match iter.next(&self.instance_list) {
                        Some(target) => {
                            let ti = self.instance_list.get(target);
                            let xdist = ti.x.get() - x;
                            let ydist = ti.y.get() - y;
                            let dist = (xdist * xdist) + (ydist * ydist);
                            if dist < maxdist {
                                maxdist = dist;
                                nearest = Some(target);
                            }
                        },
                        None => break nearest,
                    }
                }
            },
            obj if obj >= 0 && obj < 100000 => {
                // Target is an object ID
                if let Some(object) = self.assets.objects.get_asset(obj) {
                    let mut iter = self.instance_list.iter_by_identity(object.children.clone());
                    let mut maxdist = Real::from(10000000000.0); // GML default
                    let mut nearest = None;
                    loop {
                        match iter.next(&self.instance_list) {
                            Some(target) => {
                                let ti = self.instance_list.get(target);
                                let xdist = ti.x.get() - x;
                                let ydist = ti.y.get() - y;
                                let dist = (xdist * xdist) + (ydist * ydist);
                                if dist < maxdist {
                                    maxdist = dist;
                                    nearest = Some(target);
                                }
                            },
                            None => break nearest,
                        }
                    }
                } else {
                    None
                }
            },
            // Target is an instance id
            _ => None,
        };

        match nearest {
            Some(t) => Ok(self.instance_list.get(t).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn instance_furthest(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, obj) = expect_args!(args, [real, real, int])?;
        // Check collision with target
        let other: Option<usize> = match obj {
            gml::ALL => {
                // Target is an object ID
                let mut iter = self.instance_list.iter_by_insertion();
                let mut maxdist = Real::from(0.0);
                let mut nearest = None;
                loop {
                    match iter.next(&self.instance_list) {
                        Some(target) => {
                            let ti = self.instance_list.get(target);
                            let xdist = ti.x.get() - x;
                            let ydist = ti.y.get() - y;
                            let dist = (xdist * xdist) + (ydist * ydist);
                            if nearest.is_none() || dist > maxdist {
                                maxdist = dist;
                                nearest = Some(target);
                            }
                        },
                        None => break nearest,
                    }
                }
            },
            obj if obj >= 0 && obj < 100000 => {
                // Target is an object ID
                if let Some(object) = self.assets.objects.get_asset(obj) {
                    let mut iter = self.instance_list.iter_by_identity(object.children.clone());
                    let mut maxdist = Real::from(0.0);
                    let mut nearest = None;
                    loop {
                        match iter.next(&self.instance_list) {
                            Some(target) => {
                                let ti = self.instance_list.get(target);
                                let xdist = ti.x.get() - x;
                                let ydist = ti.y.get() - y;
                                let dist = (xdist * xdist) + (ydist * ydist);
                                if nearest.is_none() || dist > maxdist {
                                    maxdist = dist;
                                    nearest = Some(target);
                                }
                            },
                            None => break nearest,
                        }
                    }
                } else {
                    None
                }
            },
            // Target is an instance ID
            _ => None,
        };

        match other {
            Some(t) => Ok(self.instance_list.get(t).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn instance_place(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, obj) = expect_args!(args, [real, real, int])?;

        // Set self's position to the new coordinates
        let instance = self.instance_list.get(context.this);
        let old_x = instance.x.get();
        let old_y = instance.y.get();
        instance.x.set(x);
        instance.y.set(y);
        instance.bbox_is_stale.set(true);

        // Check collision with target
        let other =
            self.find_instance_with(obj, |handle| handle != context.this && self.check_collision(context.this, handle));

        // Move self back to where it was
        instance.x.set(old_x);
        instance.y.set(old_y);
        instance.bbox_is_stale.set(true);

        match other {
            Some(t) => Ok(self.instance_list.get(t).id.get().into()),
            None => Ok(gml::NOONE.into()),
        }
    }

    pub fn instance_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, object_id) = expect_args!(args, [real, real, int])?;
        let object = self
            .assets
            .objects
            .get_asset(object_id)
            .ok_or(gml::Error::NonexistentAsset(asset::Type::Object, object_id))?;
        self.last_instance_id += 1;
        let id = self.last_instance_id;
        let instance = self.instance_list.insert(Instance::new(id, x, y, object_id, object));
        self.run_instance_event(gml::ev::CREATE, 0, instance, instance, None)?;
        Ok(id.into())
    }

    pub fn instance_copy(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let run_event = expect_args!(args, [bool])?;
        let new_instance = self.instance_list.get(context.this).clone();
        self.last_instance_id += 1;
        let id = self.last_instance_id;
        new_instance.id.set(id);
        let handle = self.instance_list.insert(new_instance);
        if run_event {
            self.run_instance_event(gml::ev::CREATE, 0, handle, handle, None)?;
        }
        Ok(id.into())
    }

    pub fn instance_change(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, run_events) = expect_args!(args, [int, bool])?;

        if run_events {
            self.run_instance_event(gml::ev::DESTROY, 0, context.this, context.this, None)?;
        }

        let object = self
            .assets
            .objects
            .get_asset(object_id)
            .ok_or(gml::Error::NonexistentAsset(asset::Type::Object, object_id))?;
        let new_instance = self.instance_list.get(context.this).clone();
        new_instance.object_index.set(object_id);
        new_instance.sprite_index.set(object.sprite_index);
        new_instance.mask_index.set(object.mask_index);
        new_instance.depth.set(Real::from(object.depth));
        new_instance.solid.set(object.solid);
        new_instance.visible.set(object.visible);
        new_instance.persistent.set(object.persistent);
        self.last_instance_id += 1; // This is incremented by GM8 but not used

        let frame_count = if let Some(sprite) = self.assets.sprites.get_asset(object.sprite_index) {
            sprite.frames.len() as f64
        } else {
            0.0
        };
        if frame_count <= new_instance.image_index.get().floor().into() {
            new_instance.image_index.set(Real::from(0.0));
        }
        new_instance.bbox_is_stale.set(true);

        self.instance_list.mark_deleted(context.this);
        let handle = self.instance_list.insert(new_instance);

        if run_events {
            self.run_instance_event(gml::ev::CREATE, 0, handle, handle, None)?;
        }

        Ok(Default::default())
    }

    pub fn instance_destroy(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.run_instance_event(gml::ev::DESTROY, 0, context.this, context.this, None)?;
        self.instance_list.mark_deleted(context.this);
        Ok(Default::default())
    }

    pub fn instance_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function instance_sprite");
        Ok(Default::default())
    }

    pub fn position_empty(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [any, any])?;
        Ok((!self.position_meeting(context, &[gml::ALL.into(), x, y])?.is_truthy()).into())
    }

    pub fn position_meeting(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, object_id) = expect_args!(args, [int, int, int])?;
        let meeting = match object_id {
            gml::SELF => self.check_collision_point(context.this, x, y, true),
            gml::OTHER => self.check_collision_point(context.other, x, y, true),
            obj => self.find_instance_with(obj, |handle| self.check_collision_point(handle, x, y, true)).is_some(),
        };
        Ok(meeting.into())
    }

    pub fn position_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y) = expect_args!(args, [int, int])?;
        let mut iter = self.instance_list.iter_by_insertion();
        while let Some(handle) = iter.next(&self.instance_list) {
            if self.check_collision_point(handle, x, y, true) {
                self.run_instance_event(gml::ev::DESTROY, 0, handle, handle, None)?;
                self.instance_list.mark_deleted(handle);
            }
        }
        Ok(Default::default())
    }

    pub fn position_change(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function position_change");
        Ok(Default::default())
    }

    pub fn instance_deactivate_all(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let notme = expect_args!(args, [bool])?;
        let mut iter = self.instance_list.iter_by_insertion();
        while let Some(handle) = iter.next(&self.instance_list) {
            self.instance_list.deactivate(handle);
        }
        if notme {
            self.instance_list.activate(context.this);
        }
        Ok(Default::default())
    }

    pub fn instance_deactivate_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let obj = expect_args!(args, [int])?;
        match obj {
            gml::SELF => self.instance_list.deactivate(context.this),
            gml::OTHER => self.instance_list.deactivate(context.other),
            gml::ALL => {
                let mut iter = self.instance_list.iter_by_insertion();
                while let Some(handle) = iter.next(&self.instance_list) {
                    self.instance_list.deactivate(handle);
                }
            },
            obj if obj < 100000 => {
                if let Some(ids) = self.assets.objects.get_asset(obj).map(|x| x.children.clone()) {
                    let mut iter = self.instance_list.iter_by_identity(ids);
                    while let Some(handle) = iter.next(&self.instance_list) {
                        self.instance_list.deactivate(handle);
                    }
                }
            },
            inst_id => {
                if let Some(handle) = self.instance_list.get_by_instid(inst_id) {
                    self.instance_list.deactivate(handle);
                }
            },
        }
        Ok(Default::default())
    }

    pub fn instance_deactivate_region(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (left, top, width, height, inside, notme) = expect_args!(args, [real, real, real, real, bool, bool])?;
        let mut iter = self.instance_list.iter_by_insertion();
        while let Some(handle) = iter.next(&self.instance_list) {
            let inst = self.instance_list.get(handle);
            let mask = self.get_instance_mask_sprite(handle);
            let outside = if mask.is_some() {
                inst.update_bbox(mask);
                left > inst.bbox_right.get().into()
                    || top > inst.bbox_bottom.get().into()
                    || left + width < inst.bbox_left.get().into()
                    || top + height < inst.bbox_top.get().into()
            } else {
                inst.x.get() < left || inst.x.get() > left + width || inst.y.get() < top || inst.y.get() > top + height
            };
            if outside != inside {
                self.instance_list.deactivate(handle);
            }
        }
        if notme {
            self.instance_list.activate(context.this);
        }
        Ok(Default::default())
    }

    pub fn instance_activate_all(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let mut iter = self.instance_list.iter_inactive();
        while let Some(handle) = iter.next(&self.instance_list) {
            self.instance_list.activate(handle);
        }
        Ok(Default::default())
    }

    pub fn instance_activate_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let obj = expect_args!(args, [int])?;
        match obj {
            gml::SELF => self.instance_list.activate(context.this),
            gml::OTHER => self.instance_list.activate(context.other),
            gml::ALL => {
                let mut iter = self.instance_list.iter_inactive();
                while let Some(handle) = iter.next(&self.instance_list) {
                    self.instance_list.activate(handle);
                }
            },
            obj if obj < 100000 => {
                if let Some(ids) = self.assets.objects.get_asset(obj).map(|x| x.children.clone()) {
                    let mut iter = self.instance_list.iter_inactive_by_identity(ids);
                    while let Some(handle) = iter.next(&self.instance_list) {
                        self.instance_list.activate(handle);
                    }
                }
            },
            inst_id => {
                if let Some(handle) = self.instance_list.get_by_instid(inst_id) {
                    self.instance_list.activate(handle);
                }
            },
        }
        Ok(Default::default())
    }

    pub fn instance_activate_region(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (left, top, width, height, inside) = expect_args!(args, [real, real, real, real, bool])?;
        let mut iter = self.instance_list.iter_inactive();
        while let Some(handle) = iter.next(&self.instance_list) {
            let inst = self.instance_list.get(handle);
            let mask = self.get_instance_mask_sprite(handle);
            let outside = if mask.is_some() {
                inst.update_bbox(mask);
                left > inst.bbox_right.get().into()
                    || top > inst.bbox_bottom.get().into()
                    || left + width < inst.bbox_left.get().into()
                    || top + height < inst.bbox_top.get().into()
            } else {
                inst.x.get() < left || inst.x.get() > left + width || inst.y.get() < top || inst.y.get() > top + height
            };
            if outside != inside {
                self.instance_list.activate(handle);
            }
        }
        Ok(Default::default())
    }

    pub fn room_goto(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let target = expect_args!(args, [int])?;
        self.scene_change = Some(SceneChange::Room(target));
        Ok(Default::default())
    }

    pub fn room_goto_previous(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self
            .room_order
            .iter()
            .position(|x| *x == self.room_id)
            .and_then(|x| x.checked_sub(1))
            .and_then(|x| self.room_order.get(x).copied())
        {
            Some(i) => {
                self.scene_change = Some(SceneChange::Room(i));
                Ok(Default::default())
            },
            None => Err(gml::Error::EndOfRoomOrder),
        }
    }

    pub fn room_goto_next(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.room_order.iter().position(|x| *x == self.room_id).and_then(|x| self.room_order.get(x + 1).copied())
        {
            Some(i) => {
                self.scene_change = Some(SceneChange::Room(i));
                Ok(Default::default())
            },
            None => Err(gml::Error::EndOfRoomOrder),
        }
    }

    pub fn room_previous(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let room = expect_args!(args, [int])?;
        Ok(self
            .room_order
            .iter()
            .position(|x| *x == room)
            .and_then(|x| x.checked_sub(1))
            .and_then(|x| self.room_order.get(x).copied())
            .unwrap_or(-1)
            .into())
    }

    pub fn room_next(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let room = expect_args!(args, [int])?;
        Ok(self
            .room_order
            .iter()
            .position(|x| *x == room)
            .and_then(|x| self.room_order.get(x + 1).copied())
            .unwrap_or(-1)
            .into())
    }

    pub fn room_restart(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.scene_change = Some(SceneChange::Room(self.room_id));
        Ok(Default::default())
    }

    pub fn game_end(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.scene_change = Some(SceneChange::End);
        Ok(Default::default())
    }

    pub fn game_restart(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.scene_change = Some(SceneChange::Restart);
        Ok(Default::default())
    }

    pub fn game_load(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let fname = expect_args!(args, [string])?;
        let mut file = std::fs::File::open(fname.as_ref())
            .map(std::io::BufReader::new)
            .map_err(|e| gml::Error::FunctionError("game_load".into(), format!("{}", e)))?;
        let mut magnum = [0u8; 4];
        file.read(&mut magnum).map_err(|e| gml::Error::FunctionError("game_load".into(), format!("{}", e)))?;
        if magnum != [0x1d, 0x02, 0x00, 0x00] {
            return Err(gml::Error::FunctionError("game_load".into(), "tried to load wrong version of save file".into()))
        }
        let save: GMSave = bincode::deserialize_from(file)
            .map_err(|e| gml::Error::FunctionError("game_load".into(), format!("{}", e)))?;
        save.into_game(self).map_err(|e| gml::Error::FunctionError("game_load".into(), e))?;
        Ok(Default::default())
    }

    pub fn game_save(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let fname = expect_args!(args, [string])?;
        let save = GMSave::from_game(self);
        let mut file = std::fs::File::create(fname.as_ref())
            .map(std::io::BufWriter::new)
            .map_err(|e| gml::Error::FunctionError("game_save".into(), format!("{}", e)))?;
        // write magic number (0x21c in GM8)
        file.write(&[0x1d, 0x02, 0x00, 0x00])
            .map_err(|e| gml::Error::FunctionError("game_save".into(), format!("{}", e)))?;
        bincode::serialize_into(&mut file, &save)
            .map_err(|e| gml::Error::FunctionError("game_save".into(), format!("{}", e)))?;
        file.flush().map_err(|e| gml::Error::FunctionError("game_save".into(), e.to_string()))?;
        Ok(Default::default())
    }

    pub fn transition_define(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, script_name) = expect_args!(args, [int, bytes])?;
        self.user_transitions.insert(id, UserTransition { script_name });
        Ok(Default::default())
    }

    pub fn transition_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let transition_id = expect_args!(args, [int])?;
        Ok(self.get_transition(transition_id).is_some().into())
    }

    pub fn sleep(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let millis = expect_args!(args, [int])?;
        if millis > 0 {
            datetime::sleep(std::time::Duration::from_millis(millis as u64));
            if let Some(ns) = self.spoofed_time_nanos.as_mut() {
                *ns += (millis as u128) * 1_000_000;
            }
            self.process_window_events();
        }
        Ok(Default::default())
    }

    pub fn yoyo_getplatform(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function YoYo_GetPlatform");
        Ok(Default::default())
    }

    pub fn yoyo_getdevice(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function YoYo_GetDevice");
        Ok(Default::default())
    }

    pub fn yoyo_openurl(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function YoYo_OpenURL");
        Ok(Default::default())
    }

    pub fn yoyo_openurl_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function YoYo_OpenURL_ext");
        Ok(Default::default())
    }

    pub fn yoyo_openurl_full(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function YoYo_OpenURL_full");
        Ok(Default::default())
    }

    pub fn yoyo_getdomain(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function YoYo_GetDomain");
        Ok(Default::default())
    }

    pub fn yoyo_gettimer(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function YoYo_GetTimer");
        Ok(Default::default())
    }

    pub fn yoyo_addvirtualkey(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function YoYo_AddVirtualKey");
        Ok(Default::default())
    }

    pub fn yoyo_deletevirtualkey(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function YoYo_DeleteVirtualKey");
        Ok(Default::default())
    }

    pub fn yoyo_showvirtualkey(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function YoYo_ShowVirtualKey");
        Ok(Default::default())
    }

    pub fn yoyo_hidevirtualkey(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function YoYo_HideVirtualKey");
        Ok(Default::default())
    }

    pub fn yoyo_enablealphablend(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function YoYo_EnableAlphaBlend");
        Ok(Default::default())
    }

    pub fn file_bin_open(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (filename, mode) = expect_args!(args, [string, int])?;
        let mode = match mode {
            0 => file::AccessMode::Read,
            1 => file::AccessMode::Write,
            2 | _ => file::AccessMode::Special,
        };
        match self.binary_files.add_from(|| Ok(file::BinaryHandle::open(filename.as_ref(), mode)?)) {
            Ok(i) => Ok((i + 1).into()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_open".into(), e.to_string())),
        }
    }

    pub fn file_bin_rewrite(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.binary_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.clear()) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_rewrite".into(), e.to_string())),
        }
    }

    pub fn file_bin_close(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;

        // flush buffer if possible
        self.binary_files
            .get_mut(handle - 1)
            .map_or(Err(file::Error::InvalidFile(handle)), |f| f.flush())
            .map_err(|e| gml::Error::FunctionError("file_bin_close".into(), e.to_string()))?;

        if self.binary_files.delete(handle - 1) {
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("file_bin_close".into(), file::Error::InvalidFile(handle).to_string()))
        }
    }

    pub fn file_bin_position(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.binary_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.tell()) {
            Ok(p) => Ok(f64::from(p as i32).into()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_position".into(), e.to_string())),
        }
    }

    pub fn file_bin_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.binary_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.size()) {
            Ok(l) => Ok(f64::from(l as i32).into()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_size".into(), e.to_string())),
        }
    }

    pub fn file_bin_seek(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (handle, pos) = expect_args!(args, [int, int])?;
        match self.binary_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.seek(pos)) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_seek".into(), e.to_string())),
        }
    }

    pub fn file_bin_read_byte(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.binary_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.read_byte()) {
            Ok(b) => Ok(f64::from(b).into()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_read_byte".into(), e.to_string())),
        }
    }

    pub fn file_bin_write_byte(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (handle, byte) = expect_args!(args, [int, int])?;
        match self
            .binary_files
            .get_mut(handle - 1)
            .map_or(Err(file::Error::InvalidFile(handle)), |f| f.write_byte(byte as u8))
        {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_bin_write_byte".into(), e.to_string())),
        }
    }

    pub fn file_text_open_read(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        use std::error::Error as _; // for .source() trait method

        match self.text_files.add_from(|| Ok(file::TextHandle::open(filename.as_ref(), file::AccessMode::Read)?)) {
            Ok(i) => Ok((i + 1).into()),
            Err(e)
                if e.source()
                    .and_then(|r| r.downcast_ref::<std::io::Error>())
                    .map_or(false, |s| s.kind() == std::io::ErrorKind::NotFound) =>
            {
                Ok((-1).into())
            },
            Err(e) => Err(gml::Error::FunctionError("file_text_open_read".into(), e.to_string())),
        }
    }

    pub fn file_text_open_write(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match self.text_files.add_from(|| Ok(file::TextHandle::open(filename.as_ref(), file::AccessMode::Write)?)) {
            Ok(i) => Ok((i + 1).into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_open_write".into(), e.to_string())),
        }
    }

    pub fn file_text_open_append(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match self.text_files.add_from(|| Ok(file::TextHandle::open(filename.as_ref(), file::AccessMode::Special)?)) {
            Ok(i) => Ok((i + 1).into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_open_append".into(), e.to_string())),
        }
    }

    pub fn file_text_close(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        let c = self.text_files.capacity();

        // flush buffer if possible
        self.text_files
            .get_mut(handle - 1)
            .map_or(Err(file::Error::InvalidFile(handle)), |f| f.flush())
            .map_err(|e| gml::Error::FunctionError("file_text_close".into(), e.to_string()))?;

        // NB: .delete() MUST be called - beware the short-circuit evaluation here!
        if self.text_files.delete(handle - 1) || (1..=c).contains(&handle) {
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("file_text_close".into(), file::Error::InvalidFile(handle).to_string()))
        }
    }

    pub fn file_text_read_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.text_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.read_string()) {
            Ok(s) => Ok(s.into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_read_string".into(), e.to_string())),
        }
    }

    pub fn file_text_read_real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.text_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.read_real()) {
            Ok(r) => Ok(r.into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_read_real".into(), e.to_string())),
        }
    }

    pub fn file_text_readln(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.text_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.skip_line()) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_text_readln".into(), e.to_string())),
        }
    }

    pub fn file_text_eof(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.text_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.is_eof()) {
            Ok(res) => Ok(res.into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_eof".into(), e.to_string())),
        }
    }

    pub fn file_text_eoln(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.text_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.is_eoln()) {
            Ok(res) => Ok(res.into()),
            Err(e) => Err(gml::Error::FunctionError("file_text_eoln".into(), e.to_string())),
        }
    }

    pub fn file_text_write_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (handle, text) = expect_args!(args, [int, bytes])?;
        match self
            .text_files
            .get_mut(handle - 1)
            .map_or(Err(file::Error::InvalidFile(handle)), |f| f.write_string(text.as_ref()))
        {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_text_write_string".into(), e.to_string())),
        }
    }

    pub fn file_text_write_real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (handle, num) = expect_args!(args, [int, real])?;
        match self
            .text_files
            .get_mut(handle - 1)
            .map_or(Err(file::Error::InvalidFile(handle)), |f| f.write_real(num.into()))
        {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_text_write_real".into(), e.to_string())),
        }
    }

    pub fn file_text_writeln(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let handle = expect_args!(args, [int])?;
        match self.text_files.get_mut(handle - 1).map_or(Err(file::Error::InvalidFile(handle)), |f| f.write_newline()) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_text_writeln".into(), e.to_string())),
        }
    }

    pub fn file_open_read(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match file::TextHandle::open(filename.as_ref(), file::AccessMode::Read) {
            Ok(f) => {
                self.open_file.replace(f);
            },
            Err(e) => {
                self.open_file.take();
                if e.kind() != std::io::ErrorKind::NotFound {
                    return Err(gml::Error::FunctionError("file_open_read".into(), e.to_string()))
                }
            },
        };
        Ok(Default::default())
    }

    pub fn file_open_write(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match file::TextHandle::open(filename.as_ref(), file::AccessMode::Write) {
            Ok(f) => {
                self.open_file.replace(f);
                Ok(Default::default())
            },
            Err(e) => {
                self.open_file.take();
                Err(gml::Error::FunctionError("file_open_write".into(), e.to_string()))
            },
        }
    }

    pub fn file_open_append(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match file::TextHandle::open(filename.as_ref(), file::AccessMode::Special) {
            Ok(f) => {
                self.open_file.replace(f);
                Ok(Default::default())
            },
            Err(e) => {
                self.open_file.take();
                Err(gml::Error::FunctionError("file_open_append".into(), e.to_string()))
            },
        }
    }

    pub fn file_close(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.open_file.take() {
            Some(mut f) => match f.flush() {
                Ok(()) => Ok(Default::default()),
                Err(e) => Err(gml::Error::FunctionError("file_close".into(), e.to_string())),
            },
            None => Ok(Default::default()),
        }
    }

    pub fn file_read_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.open_file.as_mut().map_or(Err(file::Error::LegacyFileUnopened), |f| f.read_string()) {
            Ok(s) => Ok(s.into()),
            Err(e) => Err(gml::Error::FunctionError("file_read_string".into(), e.to_string())),
        }
    }

    pub fn file_read_real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.open_file.as_mut().map_or(Err(file::Error::LegacyFileUnopened), |f| f.read_real()) {
            Ok(r) => Ok(r.into()),
            Err(e) => Err(gml::Error::FunctionError("file_read_real".into(), e.to_string())),
        }
    }

    pub fn file_readln(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.open_file.as_mut().map_or(Err(file::Error::LegacyFileUnopened), |f| f.skip_line()) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_readln".into(), e.to_string())),
        }
    }

    pub fn file_eof(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.open_file.as_mut().map_or(Err(file::Error::LegacyFileUnopened), |f| f.is_eof()) {
            Ok(res) => Ok(res.into()),
            Err(e) => Err(gml::Error::FunctionError("file_eof".into(), e.to_string())),
        }
    }

    pub fn file_eoln(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.open_file.as_mut().map_or(Err(file::Error::LegacyFileUnopened), |f| f.is_eoln()) {
            Ok(res) => Ok(res.into()),
            Err(e) => Err(gml::Error::FunctionError("file_eoln".into(), e.to_string())),
        }
    }

    pub fn file_write_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let text = expect_args!(args, [bytes])?;
        match self.open_file.as_mut().map_or(Err(file::Error::LegacyFileUnopened), |f| f.write_string(text.as_ref())) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_write_string".into(), e.to_string())),
        }
    }

    pub fn file_write_real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let num = expect_args!(args, [real])?;
        match self.open_file.as_mut().map_or(Err(file::Error::LegacyFileUnopened), |f| f.write_real(num.into())) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_write_real".into(), e.to_string())),
        }
    }

    pub fn file_writeln(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.open_file.as_mut().map_or(Err(file::Error::LegacyFileUnopened), |f| f.write_newline()) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_writeln".into(), e.to_string())),
        }
    }

    pub fn file_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).map(|x| match x {
            Value::Str(s) => file::file_exists(&self.decode_str(s.as_ref())).into(),
            Value::Real(_) => gml::FALSE.into(),
        })
    }

    pub fn file_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let filename = expect_args!(args, [string])?;
        match file::delete(filename.as_ref()) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("file_delete".into(), e.to_string())),
        }
    }

    pub fn file_rename(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (from, to) = expect_args!(args, [string, string])?;
        if file::rename(from.as_ref(), to.as_ref()).is_err() {
            // Fail silently
            eprintln!("Warning (file_rename): could not rename {} to {}", from, to);
        }
        Ok(Default::default())
    }

    pub fn file_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (from, to) = expect_args!(args, [string, string])?;
        if file::copy(from.as_ref(), to.as_ref()).is_err() {
            // Fail silently
            eprintln!("Warning (file_copy): could not copy {} to {}", from, to);
        }
        Ok(Default::default())
    }

    pub fn directory_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [any]).map(|x| match x {
            Value::Str(s) => file::dir_exists(&self.decode_str(s.as_ref())).into(),
            Value::Real(_) => gml::FALSE.into(),
        })
    }

    pub fn directory_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path = expect_args!(args, [string])?;
        match file::dir_create(path.as_ref()) {
            Ok(()) => Ok(Default::default()),
            Err(e) => Err(gml::Error::FunctionError("directory_create".into(), e.to_string())),
        }
    }

    pub fn file_find_first(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path, attribs) = expect_args!(args, [string, int])?;
        if path.ends_with("/") || path.ends_with("\\") {
            // match nothing
            self.file_finder = None;
            return Ok(b"".as_ref().into())
        }
        // unwrap arguments
        let path: &str = path.as_ref();
        let include_read_only = (attribs & 1) != 0;
        let include_hidden = (attribs & 2) != 0;
        let include_sys_file = (attribs & 4) != 0;
        let include_volume_id = (attribs & 8) != 0;
        let include_directory = (attribs & 16) != 0;
        let include_archive = (attribs & 32) != 0;
        match glob::glob_with(path, glob::MatchOptions { case_sensitive: false, ..Default::default() }) {
            Ok(paths) => {
                // add . and .. to start if necessary
                let path: &std::path::Path = path.as_ref();
                let preceding: Vec<std::path::PathBuf> = match path.file_name().and_then(|p| p.to_str()) {
                    Some("*") | Some(".*") | Some("*.") => vec![".".into(), "..".into()],
                    Some(".") => vec![".".into()],
                    Some("..") => vec!["..".into()],
                    _ => vec![],
                };
                self.file_finder = Some(Box::new(
                    preceding.into_iter().chain(
                        paths
                            .filter_map(Result::ok)
                            .filter(move |p| {
                                let md = match p.metadata() {
                                    Ok(m) => m,
                                    Err(_) => return false,
                                };
                                // false means the check isn't in yet
                                (include_read_only || !md.permissions().readonly())
                                    && (include_hidden || !false)
                                    && (include_sys_file || !false)
                                    && (include_volume_id || !false)
                                    && (include_directory || !md.is_dir())
                                    && (include_archive || !false)
                            })
                            .map(|p| p.file_name().map(|p| p.into()).unwrap_or(p)),
                    ),
                ));
                self.file_find_next(context, &[])
            },
            Err(e) => Err(gml::Error::FunctionError("file_find_first".into(), e.to_string())),
        }
    }

    pub fn file_find_next(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        while let Some(p) = self.file_finder.as_mut().and_then(|ff| ff.next()) {
            if let Some(p) = p.to_str().and_then(|p| self.encode_str_maybe(p)) {
                return Ok(Value::from(p.as_ref()))
            }
        }
        self.file_finder = None;
        Ok(b"".as_ref().into())
    }

    pub fn file_find_close(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.file_finder = None;
        Ok(Default::default())
    }

    pub fn file_attributes(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function file_attributes");
        Ok(Default::default())
    }

    pub fn filename_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let full_path = expect_args!(args, [string])?;
        if let Some(name) = full_path.as_ref().rsplitn(2, '\\').next() {
            Ok(name.to_string().into())
        } else {
            Ok(full_path.as_ref().into())
        }
    }

    pub fn filename_path(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let full_path = expect_args!(args, [string])?;
        if let Some(bs) = full_path.as_ref().rfind('\\') {
            Ok(full_path.as_ref()[..bs + 1].to_string().into())
        } else {
            Ok("".to_string().into())
        }
    }

    pub fn filename_dir(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let full_path = expect_args!(args, [string])?;
        if let Some(bs) = full_path.as_ref().rfind('\\') {
            Ok(full_path.as_ref()[..bs].to_string().into())
        } else {
            Ok("".to_string().into())
        }
    }

    pub fn filename_drive(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let full_path = expect_args!(args, [string])?;
        let drive = full_path.as_ref().chars().take(2).collect::<String>();
        if !drive.starts_with(':') && drive.ends_with(':') { Ok(drive.into()) } else { Ok("".to_string().into()) }
    }

    pub fn filename_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let full_path = expect_args!(args, [string])?;
        if let Some(dot) = full_path.as_ref().rfind('.') {
            Ok(full_path.as_ref()[dot..].to_string().into())
        } else {
            Ok("".to_string().into())
        }
    }

    pub fn filename_change_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (full_path, new_ext) = expect_args!(args, [string, string])?;
        let mut new_path = full_path.as_ref().rsplitn(2, '.').last().unwrap_or(full_path.as_ref()).to_string();
        new_path.push_str(new_ext.as_ref());
        Ok(new_path.into())
    }

    pub fn export_include_file(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let name = expect_args!(args, [bytes])?;
        let temp_directory = self.decode_str(self.temp_directory.as_ref()).into_owned().into();
        let program_directory = self.decode_str(self.program_directory.as_ref()).into_owned().into();
        if let Some(file) = self.included_files.iter_mut().filter(|i| name.eq_ignore_ascii_case(i.name.as_ref())).next()
        {
            match file.export(temp_directory, program_directory) {
                Ok(()) => Ok(Default::default()),
                Err(e) => Err(gml::Error::FunctionError("export_include_file".into(), e.to_string())),
            }
        } else {
            Err(gml::Error::FunctionError("export_include_file".into(), "Trying to export non-existing file.".into()))
        }
    }

    pub fn export_include_file_location(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (name, path) = expect_args!(args, [bytes, string])?;
        if let Some(file) = self.included_files.iter_mut().filter(|i| name.eq_ignore_ascii_case(i.name.as_ref())).next()
        {
            let path_ref: &str = path.as_ref();
            match file.export_to(path_ref.as_ref()) {
                Ok(()) => Ok(Default::default()),
                Err(e) => Err(gml::Error::FunctionError("export_include_file_location".into(), e.to_string())),
            }
        } else {
            Err(gml::Error::FunctionError(
                "export_include_file_location".into(),
                "Trying to export non-existing file.".into(),
            ))
        }
    }

    pub fn discard_include_file(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let name = expect_args!(args, [bytes])?;
        if let Some(file) = self.included_files.iter_mut().filter(|i| name.eq_ignore_ascii_case(i.name.as_ref())).next()
        {
            file.data = None;
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("discard_include_file".into(), "Trying to discard non-existing file.".into()))
        }
    }

    pub fn execute_program(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (prog, prog_args, wait) = expect_args!(args, [string, string, bool])?;
        // Rust doesn't let you execute a program with just a string, so unescape it manually
        let mut command_array = Vec::new();
        let mut buf = Some(String::new());
        let mut quote_count = 0;
        for c in format!("{} {}", prog, prog_args).replace("\\\"", "\"\"\"").chars() {
            match c {
                '"' => {
                    buf = buf.or(Some("".into()));
                    quote_count += 1;
                    if quote_count > 2 {
                        quote_count = 0;
                        buf.as_mut().unwrap().push('"');
                    }
                },
                c if c.is_whitespace() && quote_count != 1 => {
                    quote_count %= 2;
                    if let Some(s) = buf {
                        command_array.push(s);
                        buf = None;
                    }
                },
                c => {
                    quote_count %= 2;
                    buf = buf.or(Some("".into()));
                    buf.as_mut().unwrap().push(c);
                },
            }
        }
        if let Some(s) = buf {
            command_array.push(s);
        }
        if command_array.is_empty() {
            return Err(gml::Error::FunctionError("execute_program".into(), "Cannot execute an empty string".into()))
        }
        // Actually run the program
        match Command::new(&command_array[0]).args(&command_array[1..]).spawn() {
            Ok(mut child) => {
                if wait {
                    // wait() closes stdin. This is inaccurate, but Rust doesn't offer an alternative.
                    if let Err(e) = child.wait() {
                        return Err(gml::Error::FunctionError(
                            "execute_program".into(),
                            format!("Cannot wait for {}: {}", prog, e),
                        ))
                    }
                    self.process_window_events();
                }
                Ok(Default::default())
            },
            Err(e) => {
                Err(gml::Error::FunctionError("execute_program".into(), format!("Cannot execute {}: {}", prog, e)))
            },
        }
    }

    pub fn execute_shell(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function execute_shell");
        Ok(Default::default())
    }

    pub fn parameter_count(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        // Gamemaker doesn't count parameter 0 (the game exe) as a "parameter"
        return Ok((self.parameters.len() - 1).into())
    }

    pub fn parameter_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let param_index = expect_args!(args, [int])?;
        if param_index >= 0 {
            Ok(match self.parameters.get(param_index as usize) {
                Some(a) => a.clone(),
                None => "".to_string(),
            }
            .into())
        } else {
            Ok("".into())
        }
    }

    pub fn environment_get_variable(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let name = expect_args!(args, [bytes])?;
        // get environment variable
        let env_os = std::env::var_os(self.decode_str(name.as_ref()).as_ref()).unwrap_or("".into());
        // convert to bytes, "" if impossible
        let env = env_os.to_str().and_then(|s| self.encode_str_maybe(s)).unwrap_or(b"".as_ref().into());
        Ok(env.as_ref().into())
    }

    pub fn registry_write_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function registry_write_string");
        Ok(Default::default())
    }

    pub fn registry_write_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function registry_write_real");
        Ok(Default::default())
    }

    pub fn registry_read_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function registry_read_string");
        Ok(Default::default())
    }

    pub fn registry_read_real(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function registry_read_real");
        Ok(Default::default())
    }

    pub fn registry_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function registry_exists");
        Ok(Default::default())
    }

    pub fn registry_write_string_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function registry_write_string_ext");
        Ok(Default::default())
    }

    pub fn registry_write_real_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function registry_write_real_ext");
        Ok(Default::default())
    }

    pub fn registry_read_string_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function registry_read_string_ext");
        Ok(Default::default())
    }

    pub fn registry_read_real_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function registry_read_real_ext");
        Ok(Default::default())
    }

    pub fn registry_exists_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function registry_exists_ext");
        Ok(Default::default())
    }

    pub fn registry_set_root(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function registry_set_root");
        Ok(Default::default())
    }

    pub fn ini_open(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let name = expect_args!(args, [bytes])?;
        let name_str = self.decode_str(name.as_ref());
        if file::file_exists(&name_str) {
            match ini::Ini::load_from_file(name_str.as_ref()) {
                Ok(ini) => {
                    self.open_ini = Some((ini, name));
                    Ok(Default::default())
                },
                Err(e) => Err(gml::Error::FunctionError("ini_open".into(), format!("{}", e))),
            }
        } else {
            self.open_ini = Some((ini::Ini::new(), name));
            Ok(Default::default())
        }
    }

    pub fn ini_close(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        match self.open_ini.as_ref() {
            Some((ini, path)) => match ini.write_to_file(self.decode_str(path.as_ref()).as_ref()) {
                Ok(()) => {
                    self.open_ini = None;
                    Ok(Default::default())
                },
                Err(e) => Err(gml::Error::FunctionError("ini_close".into(), format!("{}", e))),
            },
            None => Ok(Default::default()),
        }
    }

    pub fn ini_read_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (section, key, default) = expect_args!(args, [string, string, string])?;
        match self.open_ini.as_ref() {
            Some((ini, _)) => Ok(ini
                .section(Some(section.as_ref()))
                .and_then(|s| s.get(key))
                .unwrap_or(default.as_ref())
                .to_string()
                .into()),
            None => Err(gml::Error::FunctionError(
                "ini_read_string".into(),
                "Trying to read from undefined INI file".to_string(),
            )),
        }
    }

    pub fn ini_read_real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (section, key, default) = expect_args!(args, [string, string, real])?;
        match self.open_ini.as_ref() {
            Some((ini, _)) => match ini.section(Some(section.as_ref())).and_then(|s| s.get(key)) {
                Some(val) => match val.parse::<f64>() {
                    Ok(x) => Ok(x.into()),
                    Err(_) => Ok(Default::default()),
                },
                None => Ok(default.into()),
            },
            None => Err(gml::Error::FunctionError(
                "ini_read_real".into(),
                "Trying to read from undefined INI file".to_string(),
            )),
        }
    }

    pub fn ini_write_string(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (section, key, val) = expect_args!(args, [string, string, string])?;
        match self.open_ini.as_mut() {
            Some((ini, _)) => {
                ini.with_section(Some(section.as_ref())).set(key.as_ref(), val.as_ref());
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError(
                "ini_write_string".into(),
                "Trying to write to undefined INI file".to_string(),
            )),
        }
    }

    pub fn ini_write_real(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (section, key, val) = expect_args!(args, [string, string, real])?;
        match self.open_ini.as_mut() {
            Some((ini, _)) => {
                ini.with_section(Some(section.as_ref())).set(key.as_ref(), val.to_string());
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError(
                "ini_write_real".into(),
                "Trying to write to undefined INI file".to_string(),
            )),
        }
    }

    pub fn ini_key_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (section, key) = expect_args!(args, [string, string])?;
        match self.open_ini.as_ref() {
            Some((ini, _)) => {
                Ok(ini.section(Some(section.as_ref())).map(|s| s.contains_key(key)).unwrap_or(false).into())
            },
            None => Err(gml::Error::FunctionError(
                "ini_key_exists".into(),
                "Trying to read from undefined INI file".to_string(),
            )),
        }
    }

    pub fn ini_section_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let section = expect_args!(args, [string])?;
        match self.open_ini.as_ref() {
            Some((ini, _)) => Ok(ini.section(Some(section.as_ref())).is_some().into()),
            None => Err(gml::Error::FunctionError(
                "ini_section_exists".into(),
                "Trying to read from undefined INI file".to_string(),
            )),
        }
    }

    pub fn ini_key_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (section, key) = expect_args!(args, [string, string])?;
        match self.open_ini.as_mut() {
            Some((ini, _)) => {
                ini.delete_from(Some(section.as_ref()), key.as_ref());
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError(
                "ini_key_delete".into(),
                "Trying to change undefined INI file".to_string(),
            )),
        }
    }

    pub fn ini_section_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let section = expect_args!(args, [string])?;
        match self.open_ini.as_mut() {
            Some((ini, _)) => {
                ini.delete(Some(section.as_ref()));
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError(
                "ini_section_delete".into(),
                "Trying to change undefined INI file".to_string(),
            )),
        }
    }

    pub fn disk_free(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path = match args.get(0).clone() {
            Some(Value::Str(p)) => p.as_ref().get(0).map(|&x| x as char),
            _ => None,
        };
        Ok(self.window.disk_free(path).map(|x| x as f64).unwrap_or(-1f64).into())
    }

    pub fn disk_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path = match args.get(0).clone() {
            Some(Value::Str(p)) => p.as_ref().get(0).map(|&x| x as char),
            _ => None,
        };
        Ok(self.window.disk_size(path).map(|x| x as f64).unwrap_or(-1f64).into())
    }

    pub fn splash_set_caption(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_caption");
        Ok(Default::default())
    }

    pub fn splash_set_fullscreen(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_fullscreen");
        Ok(Default::default())
    }

    pub fn splash_set_border(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_border");
        Ok(Default::default())
    }

    pub fn splash_set_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function splash_set_size");
        Ok(Default::default())
    }

    pub fn splash_set_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function splash_set_position");
        Ok(Default::default())
    }

    pub fn splash_set_adapt(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_adapt");
        Ok(Default::default())
    }

    pub fn splash_set_top(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_top");
        Ok(Default::default())
    }

    pub fn splash_set_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_color");
        Ok(Default::default())
    }

    pub fn splash_set_main(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_main");
        Ok(Default::default())
    }

    pub fn splash_set_scale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_scale");
        Ok(Default::default())
    }

    pub fn splash_set_cursor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_cursor");
        Ok(Default::default())
    }

    pub fn splash_set_interrupt(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_interrupt");
        Ok(Default::default())
    }

    pub fn splash_set_stop_key(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_stop_key");
        Ok(Default::default())
    }

    pub fn splash_set_close_button(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_close_button");
        Ok(Default::default())
    }

    pub fn splash_set_stop_mouse(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function splash_set_stop_mouse");
        Ok(Default::default())
    }

    pub fn splash_show_video(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function splash_show_video");
        Ok(Default::default())
    }

    pub fn splash_show_image(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function splash_show_image");
        Ok(Default::default())
    }

    pub fn splash_show_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function splash_show_text");
        Ok(Default::default())
    }

    pub fn splash_show_web(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function splash_show_web");
        Ok(Default::default())
    }

    pub fn show_image(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function show_image");
        Ok(Default::default())
    }

    pub fn show_video(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function show_video");
        Ok(Default::default())
    }

    pub fn show_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function show_text");
        Ok(Default::default())
    }

    pub fn show_message(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let _text = expect_args!(args, [string])?;
        let width = 300;
        let height = 200;

        let clear_colour = Colour::new(1.0, 142.0 / 255.0, 250.0 / 255.0);
        let options =
            RendererOptions { size: (width, height), vsync: false, interpolate_pixels: false, ..Default::default() };

        // TODO: this should block as a dialog, not block the entire fucking thread
        // otherwise windows thinks it's not responding or whatever

        let wb = window::WindowBuilder::new().with_size(width, height);
        let mut window = wb.build().map_err(|e| gml::Error::FunctionError("show_message".into(), e))?;
        let mut renderer = Renderer::new((), &options, &window, clear_colour)
            .map_err(|e| gml::Error::FunctionError("show_message".into(), e))?;
        window.set_visible(true);
        renderer.set_vsync(false);

        loop {
            window.process_events();
            if window.close_requested() {
                break
            }

            if window.get_inner_size() != (0, 0) {
                renderer.finish(width, height, clear_colour);
            }
        }

        // restore renderer
        // self.renderer.set_current(); <- TODO, obviously

        Ok(Default::default())
    }

    pub fn show_question(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function show_question");
        Ok(Default::default())
    }

    pub fn show_error(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function show_error");
        Ok(Default::default())
    }

    pub fn show_info(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function show_info");
        Ok(Default::default())
    }

    pub fn load_info(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function load_info");
        Ok(Default::default())
    }

    pub fn highscore_show(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function highscore_show");
        Ok(Default::default())
    }

    pub fn highscore_set_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function highscore_set_background");
        Ok(Default::default())
    }

    pub fn highscore_set_border(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function highscore_set_border");
        Ok(Default::default())
    }

    pub fn highscore_set_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function highscore_set_font");
        Ok(Default::default())
    }

    pub fn highscore_set_strings(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function highscore_set_strings");
        Ok(Default::default())
    }

    pub fn highscore_set_colors(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function highscore_set_colors");
        Ok(Default::default())
    }

    pub fn highscore_show_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        println!("Called unimplemented kernel function highscore_show_ext");
        Ok(Default::default())
    }

    pub fn highscore_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function highscore_clear");
        Ok(Default::default())
    }

    pub fn highscore_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function highscore_add");
        Ok(Default::default())
    }

    pub fn highscore_add_current(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function highscore_add_current");
        Ok(Default::default())
    }

    pub fn highscore_value(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function highscore_value");
        Ok(Default::default())
    }

    pub fn highscore_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function highscore_name");
        Ok(Default::default())
    }

    pub fn draw_highscore(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function draw_highscore");
        Ok(Default::default())
    }

    pub fn show_message_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function show_message_ext");
        Ok(Default::default())
    }

    pub fn message_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_background")
        // TODO
        Ok(Default::default())
    }

    pub fn message_button(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_button")
        // TODO
        Ok(Default::default())
    }

    pub fn message_alpha(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_alpha")
        // TODO
        Ok(Default::default())
    }

    pub fn message_text_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        //unimplemented!("Called unimplemented kernel function message_text_font")
        // TODO
        Ok(Default::default())
    }

    pub fn message_button_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        //unimplemented!("Called unimplemented kernel function message_button_font")
        // TODO
        Ok(Default::default())
    }

    pub fn message_input_font(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        //unimplemented!("Called unimplemented kernel function message_input_font")
        // TODO
        Ok(Default::default())
    }

    pub fn message_text_charset(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function message_text_charset")
        // TODO
        Ok(Default::default())
    }

    pub fn message_mouse_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_mouse_color")
        // TODO
        Ok(Default::default())
    }

    pub fn message_input_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function message_input_color")
        // TODO
        Ok(Default::default())
    }

    pub fn message_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function message_position")
        // TODO
        Ok(Default::default())
    }

    pub fn message_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function message_size")
        // TODO
        Ok(Default::default())
    }

    pub fn message_caption(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function message_caption")
        // TODO
        Ok(Default::default())
    }

    pub fn show_menu(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function show_menu");
        Ok(Default::default())
    }

    pub fn show_menu_pos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function show_menu_pos");
        Ok(Default::default())
    }

    pub fn get_integer(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function get_integer");
        Ok(Default::default())
    }

    pub fn get_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function get_string");
        Ok(Default::default())
    }

    pub fn get_color(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function get_color");
        Ok(Default::default())
    }

    pub fn get_open_filename(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function get_open_filename");
        Ok(Default::default())
    }

    pub fn get_save_filename(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function get_save_filename");
        Ok(Default::default())
    }

    pub fn get_directory(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function get_directory");
        Ok(Default::default())
    }

    pub fn get_directory_alt(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function get_directory_alt");
        Ok(Default::default())
    }

    pub fn keyboard_get_numlock(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.input_manager.key_get_numlock().into())
    }

    pub fn keyboard_set_numlock(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [bool]).map(|x| self.input_manager.key_set_numlock(x))?;
        Ok(Default::default())
    }

    pub fn keyboard_key_press(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function keyboard_key_press");
        Ok(Default::default())
    }

    pub fn keyboard_key_release(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function keyboard_key_release");
        Ok(Default::default())
    }

    pub fn keyboard_set_map(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (real, mapped) = expect_args!(args, [int, int])?;
        self.input_manager.key_set_map(real as usize, mapped as usize);
        Ok(Default::default())
    }

    pub fn keyboard_get_map(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        Ok((self.input_manager.key_get_map(key as usize) as i32).into())
    }

    pub fn keyboard_unset_map(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.input_manager.key_unmap_all();
        Ok(Default::default())
    }

    pub fn keyboard_check(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        match key {
            k if k < 0 => Ok(gml::FALSE.into()),
            0 => Ok((!self.input_manager.key_check_any()).into()),
            1 => Ok(self.input_manager.key_check_any().into()),
            key => Ok(self.input_manager.key_check(key as usize).into()),
        }
    }

    pub fn keyboard_check_pressed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        match key {
            k if k < 0 => Ok(gml::FALSE.into()),
            0 => Ok((!self.input_manager.key_check_any_pressed()).into()),
            1 => Ok(self.input_manager.key_check_any_pressed().into()),
            key => Ok(self.input_manager.key_check_pressed(key as usize).into()),
        }
    }

    pub fn keyboard_check_released(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        match key {
            k if k < 0 => Ok(gml::FALSE.into()),
            0 => Ok((!self.input_manager.key_check_any_released()).into()),
            1 => Ok(self.input_manager.key_check_any_released().into()),
            key => Ok(self.input_manager.key_check_released(key as usize).into()),
        }
    }

    pub fn keyboard_check_direct(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        match key {
            k if k < 0 => Ok(gml::FALSE.into()),
            0 => Ok((!self.input_manager.key_check_any()).into()),
            1 => Ok(self.input_manager.key_check_any().into()),
            160 => Ok(self.input_manager.key_check_lshift().into()),
            161 => Ok(self.input_manager.key_check_rshift().into()),
            162 => Ok(self.input_manager.key_check_lctrl().into()),
            163 => Ok(self.input_manager.key_check_rctrl().into()),
            164 => Ok(self.input_manager.key_check_lalt().into()),
            165 => Ok(self.input_manager.key_check_ralt().into()),
            key => Ok(self.input_manager.key_check(key as usize).into()),
        }
    }

    pub fn mouse_check_button(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let button = expect_args!(args, [int])?;
        match button {
            -1 => Ok(self.input_manager.mouse_check_any().into()),
            0 => Ok((!self.input_manager.mouse_check_any()).into()),
            1 => Ok(self.input_manager.mouse_check(MouseButton::Left).into()),
            2 => Ok(self.input_manager.mouse_check(MouseButton::Right).into()),
            3 => Ok(self.input_manager.mouse_check(MouseButton::Middle).into()),
            _ => Ok(gml::FALSE.into()),
        }
    }

    pub fn mouse_check_button_pressed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let button = expect_args!(args, [int])?;
        match button {
            -1 => Ok(self.input_manager.mouse_check_any_pressed().into()),
            0 => Ok((!self.input_manager.mouse_check_any_pressed()).into()),
            1 => Ok(self.input_manager.mouse_check_pressed(MouseButton::Left).into()),
            2 => Ok(self.input_manager.mouse_check_pressed(MouseButton::Right).into()),
            3 => Ok(self.input_manager.mouse_check_pressed(MouseButton::Middle).into()),
            _ => Ok(gml::FALSE.into()),
        }
    }

    pub fn mouse_check_button_released(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let button = expect_args!(args, [int])?;
        match button {
            -1 => Ok(self.input_manager.mouse_check_any_released().into()),
            0 => Ok((!self.input_manager.mouse_check_any_released()).into()),
            1 => Ok(self.input_manager.mouse_check_released(MouseButton::Left).into()),
            2 => Ok(self.input_manager.mouse_check_released(MouseButton::Right).into()),
            3 => Ok(self.input_manager.mouse_check_released(MouseButton::Middle).into()),
            _ => Ok(gml::FALSE.into()),
        }
    }

    pub fn mouse_wheel_up(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.input_manager.mouse_check_scroll_up().into())
    }

    pub fn mouse_wheel_down(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.input_manager.mouse_check_scroll_down().into())
    }

    pub fn joystick_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_exists")
        // TODO
        Ok(gml::FALSE.into())
    }

    pub fn joystick_direction(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_direction")
        // TODO
        Ok(101.into())
    }

    pub fn joystick_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_name")
        // TODO
        Ok("".into())
    }

    pub fn joystick_axes(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_axes")
        // TODO
        Ok(0.into())
    }

    pub fn joystick_buttons(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_buttons")
        // TODO
        Ok(0.into())
    }

    pub fn joystick_has_pov(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_has_pov")
        // TODO
        Ok(gml::FALSE.into())
    }

    pub fn joystick_check_button(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function joystick_check_button")
        // TODO
        Ok(gml::FALSE.into())
    }

    pub fn joystick_xpos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_xpos")
        // TODO
        Ok(0.into())
    }

    pub fn joystick_ypos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_ypos")
        // TODO
        Ok(0.into())
    }

    pub fn joystick_zpos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_zpos")
        // TODO
        Ok(0.into())
    }

    pub fn joystick_rpos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_rpos")
        // TODO
        Ok(0.into())
    }

    pub fn joystick_upos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_upos")
        // TODO
        Ok(0.into())
    }

    pub fn joystick_vpos(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_vpos")
        // TODO
        Ok(0.into())
    }

    pub fn joystick_pov(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function joystick_pov")
        // TODO
        Ok((-1).into())
    }

    pub fn keyboard_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let key = expect_args!(args, [int])?;
        self.process_window_events();
        if key > 0 {
            self.input_manager.key_clear(key as usize);
        }
        Ok(Default::default())
    }

    pub fn mouse_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mouse_clear");
        Ok(Default::default())
    }

    pub fn io_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.process_window_events();
        self.input_manager.clear();
        Ok(Default::default())
    }

    pub fn io_handle(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.process_window_events();
        Ok(Default::default())
    }

    pub fn keyboard_wait(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        if self.play_type == PlayType::Normal {
            self.input_manager.key_set_lastkey(0);
            while self.input_manager.key_get_lastkey() == 0 {
                datetime::sleep(std::time::Duration::from_millis(50));
                self.process_window_events();
            }
        }
        Ok(Default::default())
    }

    pub fn mouse_wait(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mouse_wait");
        Ok(Default::default())
    }

    pub fn mplay_init_ipx(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_init_ipx");
        Ok(Default::default())
    }

    pub fn mplay_init_tcpip(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_init_tcpip");
        Ok(Default::default())
    }

    pub fn mplay_init_modem(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function mplay_init_modem");
        Ok(Default::default())
    }

    pub fn mplay_init_serial(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function mplay_init_serial");
        Ok(Default::default())
    }

    pub fn mplay_connect_status(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_connect_status");
        Ok(Default::default())
    }

    pub fn mplay_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_end");
        Ok(Default::default())
    }

    pub fn mplay_session_mode(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_session_mode");
        Ok(Default::default())
    }

    pub fn mplay_session_create(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function mplay_session_create");
        Ok(Default::default())
    }

    pub fn mplay_session_find(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_session_find");
        Ok(Default::default())
    }

    pub fn mplay_session_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_session_name");
        Ok(Default::default())
    }

    pub fn mplay_session_join(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function mplay_session_join");
        Ok(Default::default())
    }

    pub fn mplay_session_status(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_session_status");
        Ok(Default::default())
    }

    pub fn mplay_session_end(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_session_end");
        Ok(Default::default())
    }

    pub fn mplay_player_find(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_player_find");
        Ok(Default::default())
    }

    pub fn mplay_player_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_player_name");
        Ok(Default::default())
    }

    pub fn mplay_player_id(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_player_id");
        Ok(Default::default())
    }

    pub fn mplay_data_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function mplay_data_write");
        Ok(Default::default())
    }

    pub fn mplay_data_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_data_read");
        Ok(Default::default())
    }

    pub fn mplay_data_mode(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_data_mode");
        Ok(Default::default())
    }

    pub fn mplay_message_send(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function mplay_message_send");
        Ok(Default::default())
    }

    pub fn mplay_message_send_guaranteed(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function mplay_message_send_guaranteed");
        Ok(Default::default())
    }

    pub fn mplay_message_receive(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_message_receive");
        Ok(Default::default())
    }

    pub fn mplay_message_id(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_message_id");
        Ok(Default::default())
    }

    pub fn mplay_message_value(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_message_value");
        Ok(Default::default())
    }

    pub fn mplay_message_player(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_message_player");
        Ok(Default::default())
    }

    pub fn mplay_message_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function mplay_message_name");
        Ok(Default::default())
    }

    pub fn mplay_message_count(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_message_count");
        Ok(Default::default())
    }

    pub fn mplay_message_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function mplay_message_clear");
        Ok(Default::default())
    }

    pub fn mplay_ipaddress(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(network::get_local_ip().unwrap_or(std::net::Ipv4Addr::LOCALHOST.into()).to_string().into())
    }

    pub fn event_inherited(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let parent = self
            .assets
            .objects
            .get_asset(context.event_object)
            .ok_or(gml::Error::NonexistentAsset(asset::Type::Object, context.event_object))?
            .parent_index;
        if parent >= 0 {
            self.run_instance_event(
                context.event_type,
                context.event_number as _,
                context.this,
                context.other,
                Some(parent),
            )?;
        }
        Ok(Default::default())
    }

    pub fn event_perform(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (event_type, event_number) = expect_args!(args, [int, int])?;
        self.run_instance_event(event_type as _, event_number as _, context.this, context.other, None)?;
        Ok(Default::default())
    }

    pub fn event_user(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let number = expect_args!(args, [int])?;
        if number >= 0 && number <= 15 {
            self.run_instance_event(7, (10 + number) as _, context.this, context.other, None)?;
        }
        Ok(Default::default())
    }

    pub fn event_perform_object(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object, event_type, event_number) = expect_args!(args, [int, int, int])?;
        self.run_instance_event(event_type as _, event_number as _, context.this, context.other, Some(object))?;
        Ok(Default::default())
    }

    pub fn external_define(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        if let (Some(dll_name), Some(fn_name), Some(call_conv), Some(res_type), Some(argnumb)) =
            (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4))
        {
            let dll_name = RCStr::from(dll_name.clone());
            let fn_name = RCStr::from(fn_name.clone());
            let call_conv = match call_conv.round() {
                0 => external::CallConv::Cdecl,
                _ => external::CallConv::Stdcall,
            };
            let res_type = match res_type.round() {
                0 => external::DLLValueType::Real,
                _ => external::DLLValueType::Str,
            };
            let argnumb = argnumb.round();
            if args.len() as i32 != 5 + argnumb {
                return Err(gml::Error::WrongArgumentCount(5 + argnumb.max(5) as usize, args.len()))
            }
            let arg_types = args[5..]
                .iter()
                .map(|v| match v.round() {
                    0 => external::DLLValueType::Real,
                    _ => external::DLLValueType::Str,
                })
                .collect::<Vec<_>>();
            self.externals.push(Some(
                external::External::new(
                    external::DefineInfo { dll_name, fn_name, call_conv, res_type, arg_types },
                    self.play_type == PlayType::Record,
                    match self.gm_version {
                        Version::GameMaker8_0 => self.encoding,
                        Version::GameMaker8_1 => encoding_rs::UTF_8,
                    },
                )
                .map_err(|e| gml::Error::FunctionError("external_define".into(), e))?,
            ));
            Ok((self.externals.len() - 1).into())
        } else {
            Err(gml::Error::WrongArgumentCount(5, args.len()))
        }
    }

    pub fn external_call(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        if let Some(id) = args.get(0) {
            let id = id.round();
            if let Some(external) = self.externals.get_asset(id) {
                return external.call(&args[1..])
            }
        }
        Ok(Default::default())
    }

    pub fn external_free(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let dll_name = expect_args!(args, [bytes])?;
        for e_opt in self.externals.iter_mut() {
            if let Some(e) = e_opt {
                if e.info.dll_name.eq_ignore_ascii_case(dll_name.as_ref()) {
                    drop(e);
                    *e_opt = None;
                }
            }
        }
        Ok(Default::default())
    }

    pub fn get_function_address(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let _name = expect_args!(args, [any])?;
        // We're definitely not ABI compliant with the original GM (at least for now),
        // and don't have any compatibility layer as well, so let it just return -1,
        // as the GM implementation does for unknown function names.
        Ok((-1).into())
    }

    pub fn external_define0(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function external_define0");
        Ok(Default::default())
    }

    pub fn external_call0(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function external_call0");
        Ok(Default::default())
    }

    pub fn external_define1(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function external_define1");
        Ok(Default::default())
    }

    pub fn external_call1(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function external_call1");
        Ok(Default::default())
    }

    pub fn external_define2(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function external_define2");
        Ok(Default::default())
    }

    pub fn external_call2(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function external_call2");
        Ok(Default::default())
    }

    pub fn external_define3(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function external_define3");
        Ok(Default::default())
    }

    pub fn external_call3(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function external_call3");
        Ok(Default::default())
    }

    pub fn external_define4(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        println!("Called unimplemented kernel function external_define4");
        Ok(Default::default())
    }

    pub fn external_call4(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function external_call4");
        Ok(Default::default())
    }

    pub fn external_define5(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function external_define5");
        Ok(Default::default())
    }

    pub fn external_call5(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function external_call5");
        Ok(Default::default())
    }

    pub fn external_define6(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function external_define6");
        Ok(Default::default())
    }

    pub fn external_call6(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        println!("Called unimplemented kernel function external_call6");
        Ok(Default::default())
    }

    pub fn external_define7(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function external_define7");
        Ok(Default::default())
    }

    pub fn external_call7(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        println!("Called unimplemented kernel function external_call7");
        Ok(Default::default())
    }

    pub fn external_define8(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function external_define8");
        Ok(Default::default())
    }

    pub fn external_call8(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        println!("Called unimplemented kernel function external_call8");
        Ok(Default::default())
    }

    pub fn execute_string(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        if let Some(Value::Str(code)) = args.get(0) {
            match self.compiler.compile(code.as_ref()) {
                Ok(instrs) => {
                    let mut new_args: [Value; 16] = Default::default();
                    for (src, dest) in args[1..].iter().zip(new_args.iter_mut()) {
                        *dest = src.clone();
                    }
                    let mut new_context = Context {
                        arguments: new_args,
                        locals: DummyFieldHolder::new(),
                        return_value: Default::default(),
                        ..*context
                    };
                    self.execute(&instrs, &mut new_context)?;
                    Ok(new_context.return_value)
                },
                Err(e) => Err(gml::Error::FunctionError("execute_string".into(), e.message)),
            }
        } else {
            // eg execute_string(42) - does nothing, returns 0
            Ok(Default::default())
        }
    }

    pub fn execute_file(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        if let Some(Value::Str(path)) = args.get(0) {
            let mut new_args: [Value; 16] = Default::default();
            for (src, dest) in args.iter().zip(new_args.iter_mut()) {
                *dest = src.clone();
            }
            match std::fs::read(self.decode_str(path.as_ref()).as_ref()) {
                Ok(code) => {
                    new_args[0] = code.into();
                    self.execute_string(context, &new_args)
                },
                Err(e) => Err(gml::Error::FunctionError("execute_file".into(), format!("{}", e))),
            }
        } else {
            Err(gml::Error::FunctionError("execute_file".into(), "Trying to execute a number.".to_string()))
        }
    }

    pub fn window_handle(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        return Ok(self.window.window_handle().into())
    }

    pub fn show_debug_message(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let message = expect_args!(args, [any])?;
        println!("{}", self.decode_str(message.repr().as_ref()));
        Ok(Default::default())
    }

    pub fn set_program_priority(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function set_program_priority");
        Ok(Default::default())
    }

    pub fn set_application_title(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let _title = expect_args!(args, [any])?;
        // In GM8, the game is made out of two windows. One is the one you see, and its caption is
        // managed by room_caption and (somewhat) window_set_caption. The other's caption is set by
        // set_application_title, and its caption only shows up in the taskbar and task manager.
        // The emulator only uses one window, and emulating this behaviour isn't possible with just
        // one window, so emulating set_application_title isn't possible.
        // It's a write-only attribute, so simply making it a NOP doesn't hurt anything.
        Ok(Default::default())
    }

    pub fn variable_global_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let identifier = expect_args!(args, [bytes])?;
        if let Some(var) = mappings::get_instance_variable_by_name(identifier.as_ref()) {
            Ok(self.globals.vars.contains_key(var).into())
        } else {
            let field_id = self.compiler.get_field_id(identifier.as_ref());
            Ok(self.globals.fields.contains_key(&field_id).into())
        }
    }

    pub fn variable_global_get(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let identifier = expect_args!(args, [any])?;
        self.variable_global_array_get(context, &[identifier, 0.into()])
    }

    pub fn variable_global_array_get(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, index) = expect_args!(args, [bytes, int])?;
        let index = index as u32;
        if let Some(var) = mappings::get_instance_variable_by_name(identifier.as_ref()) {
            Ok(self.globals.vars.get(var).and_then(|x| x.get(index)).unwrap_or_default())
        } else {
            let field_id = self.compiler.get_field_id(identifier.as_ref());
            Ok(self.globals.fields.get(&field_id).and_then(|x| x.get(index)).unwrap_or_default())
        }
    }

    pub fn variable_global_array2_get(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, index1, index2) = expect_args!(args, [any, int, int])?;
        self.variable_global_array_get(context, &[identifier, ((index1 * 32000) + index2).into()])
    }

    pub fn variable_global_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, value) = expect_args!(args, [any, any])?;
        self.variable_global_array_set(context, &[identifier, 0.into(), value])
    }

    pub fn variable_global_array_set(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, index, value) = expect_args!(args, [bytes, int, any])?;
        let index = index as u32;
        if let Some(var) = mappings::get_instance_variable_by_name(identifier.as_ref()) {
            if let Some(field) = self.globals.vars.get_mut(var) {
                field.set(index, value);
            } else {
                self.globals.vars.insert(*var, Field::new(index, value));
            }
        } else {
            let field_id = self.compiler.get_field_id(identifier.as_ref());
            if let Some(field) = self.globals.fields.get_mut(&field_id) {
                field.set(index, value);
            } else {
                self.globals.fields.insert(field_id, Field::new(index, value));
            }
        }
        Ok(Default::default())
    }

    pub fn variable_global_array2_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, index1, index2, value) = expect_args!(args, [any, int, int, any])?;
        self.variable_global_array_set(context, &[identifier, ((index1 * 32000) + index2).into(), value])
    }

    pub fn variable_local_exists(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let identifier = expect_args!(args, [bytes])?;
        if mappings::get_instance_variable_by_name(identifier.as_ref()).is_some() {
            Ok(gml::TRUE.into())
        } else {
            let instance = self.instance_list.get(context.this);
            let field_id = self.compiler.get_field_id(identifier.as_ref());
            Ok(instance.fields.borrow().contains_key(&field_id).into())
        }
    }

    pub fn variable_local_get(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let identifier = expect_args!(args, [any])?;
        self.variable_local_array_get(context, &[identifier, 0.into()])
    }

    pub fn variable_local_array_get(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, index) = expect_args!(args, [bytes, int])?;
        let index = index as u32;
        if let Some(var) = mappings::get_instance_variable_by_name(identifier.as_ref()) {
            self.get_instance_var(context.this, var, index, context)
        } else {
            let instance = self.instance_list.get(context.this);
            let field_id = self.compiler.get_field_id(identifier.as_ref());
            Ok(instance.fields.borrow().get(&field_id).and_then(|x| x.get(index)).unwrap_or_default())
        }
    }

    pub fn variable_local_array2_get(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, index1, index2) = expect_args!(args, [any, int, int])?;
        self.variable_local_array_get(context, &[identifier, ((index1 * 32000) + index2).into()])
    }

    pub fn variable_local_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, value) = expect_args!(args, [any, any])?;
        self.variable_local_array_set(context, &[identifier, 0.into(), value])
    }

    pub fn variable_local_array_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, index, value) = expect_args!(args, [bytes, int, any])?;
        let index = index as u32;
        if let Some(var) = mappings::get_instance_variable_by_name(identifier.as_ref()) {
            self.set_instance_var(context.this, var, index, value, context)?;
        } else {
            let mut fields = self.instance_list.get(context.this).fields.borrow_mut();
            let field_id = self.compiler.get_field_id(identifier.as_ref());
            if let Some(field) = fields.get_mut(&field_id) {
                field.set(index, value);
            } else {
                fields.insert(field_id, Field::new(index, value));
            }
        }
        Ok(Default::default())
    }

    pub fn variable_local_array2_set(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (identifier, index1, index2, value) = expect_args!(args, [any, int, int, any])?;
        self.variable_local_array_set(context, &[identifier, ((index1 * 32000) + index2).into(), value])
    }

    pub fn clipboard_has_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function clipboard_has_text");
        Ok(Default::default())
    }

    pub fn clipboard_set_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function clipboard_set_text");
        Ok(Default::default())
    }

    pub fn clipboard_get_text(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function clipboard_get_text");
        Ok(Default::default())
    }

    pub fn date_current_datetime(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(DateTime::now_or_nanos(self.spoofed_time_nanos).into())
    }

    pub fn date_current_date(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(DateTime::now_or_nanos(self.spoofed_time_nanos).date().into())
    }

    pub fn date_current_time(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(DateTime::now_or_nanos(self.spoofed_time_nanos).time().into())
    }

    pub fn date_create_datetime(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        // todo: this may give different results than gm8 due to floating point error?
        let (year, month, day, hour, minute, second) = expect_args!(args, [int, int, int, int, int, int])?;
        Ok(DateTime::from_ymdhms(year, month, day, hour, minute, second).map(Real::from).unwrap_or(0.into()).into())
    }

    pub fn date_create_date(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (year, month, day) = expect_args!(args, [int, int, int])?;
        Ok(DateTime::from_ymd(year, month, day).map(Real::from).unwrap_or(0.into()).into())
    }

    pub fn date_create_time(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (hour, minute, second) = expect_args!(args, [int, int, int])?;
        Ok(DateTime::from_hms(hour, minute, second).map(Real::from).unwrap_or(0.into()).into())
    }

    pub fn date_valid_datetime(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (year, month, day, hour, minute, second) = expect_args!(args, [int, int, int, int, int, int])?;
        Ok(DateTime::from_ymd(year, month, day).and_then(|_| DateTime::from_hms(hour, minute, second)).is_some().into())
    }

    pub fn date_valid_date(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function date_valid_date");
        Ok(Default::default())
    }

    pub fn date_valid_time(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function date_valid_time");
        Ok(Default::default())
    }

    pub fn date_inc_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_inc_year");
        Ok(Default::default())
    }

    pub fn date_inc_month(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_inc_month");
        Ok(Default::default())
    }

    pub fn date_inc_week(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_inc_week");
        Ok(Default::default())
    }

    pub fn date_inc_day(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (datetime, amount) = expect_args!(args, [real, int])?;
        Ok((datetime + Real::from(amount)).into())
    }

    pub fn date_inc_hour(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (datetime, amount) = expect_args!(args, [real, int])?;
        Ok((datetime + Real::from(amount) / 24.into() * if datetime <= 0.into() { Real::from(-1) } else { 1.into() })
            .into())
    }

    pub fn date_inc_minute(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (datetime, amount) = expect_args!(args, [real, int])?;
        Ok((datetime + Real::from(amount) / 1440.into() * if datetime <= 0.into() { Real::from(-1) } else { 1.into() })
            .into())
    }

    pub fn date_inc_second(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (datetime, amount) = expect_args!(args, [real, int])?;
        Ok((datetime
            + Real::from(amount) / 86400.into() * if datetime <= 0.into() { Real::from(-1) } else { 1.into() })
        .into())
    }

    pub fn date_get_year(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).year().into())
    }

    pub fn date_get_month(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).month().into())
    }

    pub fn date_get_week(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).week().into())
    }

    pub fn date_get_day(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).day().into())
    }

    pub fn date_get_hour(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).hour().into())
    }

    pub fn date_get_minute(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).minute().into())
    }

    pub fn date_get_second(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).second().into())
    }

    pub fn date_get_weekday(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).weekday().into())
    }

    pub fn date_get_day_of_year(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).day_of_year().into())
    }

    pub fn date_get_hour_of_year(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).hour_of_year().into())
    }

    pub fn date_get_minute_of_year(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).minute_of_year().into())
    }

    pub fn date_get_second_of_year(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let datetime = expect_args!(args, [real])?;
        Ok(DateTime::from(datetime).second_of_year().into())
    }

    pub fn date_year_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_year_span");
        Ok(Default::default())
    }

    pub fn date_month_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_month_span");
        Ok(Default::default())
    }

    pub fn date_week_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_week_span");
        Ok(Default::default())
    }

    pub fn date_day_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_day_span");
        Ok(Default::default())
    }

    pub fn date_hour_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_hour_span");
        Ok(Default::default())
    }

    pub fn date_minute_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_minute_span");
        Ok(Default::default())
    }

    pub fn date_second_span(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_second_span");
        Ok(Default::default())
    }

    pub fn date_compare_datetime(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_compare_datetime");
        Ok(Default::default())
    }

    pub fn date_compare_date(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_compare_date");
        Ok(Default::default())
    }

    pub fn date_compare_time(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function date_compare_time");
        Ok(Default::default())
    }

    pub fn date_date_of(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_date_of");
        Ok(Default::default())
    }

    pub fn date_time_of(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_time_of");
        Ok(Default::default())
    }

    pub fn date_datetime_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_datetime_string");
        Ok(Default::default())
    }

    pub fn date_date_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_date_string");
        Ok(Default::default())
    }

    pub fn date_time_string(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_time_string");
        Ok(Default::default())
    }

    pub fn date_days_in_month(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_days_in_month");
        Ok(Default::default())
    }

    pub fn date_days_in_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_days_in_year");
        Ok(Default::default())
    }

    pub fn date_leap_year(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_leap_year");
        Ok(Default::default())
    }

    pub fn date_is_today(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function date_is_today");
        Ok(Default::default())
    }

    pub fn sprite_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.sprite_get_name(context, args)
    }

    pub fn sprite_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        Ok(self.assets.sprites.get_asset(sprite).is_some().into())
    }

    pub fn sprite_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let asset_id = expect_args!(args, [int])?;
        Ok(self.assets.sprites.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
    }

    pub fn sprite_get_number(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.frames.len().into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn sprite_get_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) { Ok(sprite.width.into()) } else { Ok((-1).into()) }
    }

    pub fn sprite_get_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.height.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn sprite_get_xoffset(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.origin_x.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn sprite_get_yoffset(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.origin_y.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn sprite_get_bbox_left(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.bbox_left.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn sprite_get_bbox_right(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.bbox_right.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn sprite_get_bbox_top(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.bbox_top.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn sprite_get_bbox_bottom(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite) {
            Ok(sprite.bbox_bottom.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn sprite_set_offset(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite, x, y) = expect_args!(args, [int, int, int])?;
        if let Some(sprite) = self.assets.sprites.get_asset_mut(sprite) {
            sprite.origin_x = x;
            sprite.origin_y = y;
        }
        Ok(Default::default())
    }

    pub fn sprite_set_alpha_from_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function sprite_set_alpha_from_sprite");
        Ok(Default::default())
    }

    pub fn sprite_create_from_screen(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, width, height, transparency, smooth, origin_x, origin_y) =
            expect_args!(args, [int, int, int, int, int, bool, int, int])?;
        let (removeback, fill_transparent) = match self.gm_version {
            Version::GameMaker8_0 => (transparency != 0, true),
            Version::GameMaker8_1 => (transparency == 1, transparency != 2),
        };
        // i know we're downloading the thing and reuploading it instead of doing it all in one go
        // but we need the pixel data to make the colliders
        let x = x.max(0);
        let y = y.max(0);
        let width = width.min(self.unscaled_width as i32 - x);
        let height = height.min(self.unscaled_height as i32 - y);
        self.renderer.flush_queue();
        let rgba = self.renderer.get_pixels(x, y, width, height);
        let mut image = RgbaImage::from_vec(width as _, height as _, rgba.into_vec()).unwrap();
        asset::sprite::process_image(&mut image, removeback, smooth, fill_transparent);
        if self.gm_version == Version::GameMaker8_1 && transparency == -1 {
            // make entire image opaque
            image.pixels_mut().for_each(|p| p[3] = 255);
        }
        let colliders = asset::sprite::make_colliders_precise(std::slice::from_ref(&image), 0, false);
        let frames = vec![asset::sprite::Frame {
            width: width as _,
            height: height as _,
            atlas_ref: self
                .renderer
                .upload_sprite(image.into_raw().into_boxed_slice(), width, height, origin_x, origin_y)
                .map_err(|e| gml::Error::FunctionError("sprite_create_from_screen".into(), e.into()))?,
        }];
        let sprite_id = self.assets.sprites.len();
        self.assets.sprites.push(Some(Box::new(asset::Sprite {
            name: format!("__newsprite{}", sprite_id).into(),
            frames,
            bbox_left: colliders[0].bbox_left,
            bbox_right: colliders[0].bbox_right,
            bbox_top: colliders[0].bbox_top,
            bbox_bottom: colliders[0].bbox_bottom,
            colliders: colliders,
            width: width as _,
            height: height as _,
            origin_x,
            origin_y,
            per_frame_colliders: false,
        })));
        Ok(sprite_id.into())
    }

    pub fn sprite_add_from_screen(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, x, y, width, height, removeback, smooth) =
            expect_args!(args, [int, int, int, int, int, bool, bool])?;
        if let Some(sprite) = self.assets.sprites.get_asset_mut(sprite_id) {
            // get image
            let x = x.max(0);
            let y = y.max(0);
            let width = width.min(self.unscaled_width as i32 - x);
            let height = height.min(self.unscaled_height as i32 - y);
            self.renderer.flush_queue();
            let rgba = self.renderer.get_pixels(x, y, width, height);
            let mut image = RgbaImage::from_vec(width as _, height as _, rgba.into_vec()).unwrap();
            asset::sprite::process_image(&mut image, removeback, smooth, true);
            asset::sprite::scale(&mut image, sprite.width, sprite.height);
            // generate collision
            let mut images = Vec::with_capacity(sprite.frames.len() + 1);
            // can't use .map() because closures cause borrowing issues
            for f in sprite.frames.iter() {
                images.push(
                    RgbaImage::from_vec(f.width, f.height, self.renderer.dump_sprite(&f.atlas_ref).into_vec()).unwrap(),
                );
            }
            images.push(image);
            let sprite = self.assets.sprites.get_asset_mut(sprite_id).unwrap();
            sprite.colliders = asset::sprite::make_colliders_precise(&images, 0, sprite.per_frame_colliders);
            sprite.bbox_left = sprite.colliders.iter().map(|c| c.bbox_left).min().unwrap();
            sprite.bbox_top = sprite.colliders.iter().map(|c| c.bbox_top).min().unwrap();
            sprite.bbox_right = sprite.colliders.iter().map(|c| c.bbox_right).max().unwrap();
            sprite.bbox_bottom = sprite.colliders.iter().map(|c| c.bbox_bottom).max().unwrap();
            // upload frame
            let image = images.pop().unwrap();
            sprite.frames.push(asset::sprite::Frame {
                width: sprite.width as _,
                height: sprite.height as _,
                atlas_ref: self
                    .renderer
                    .upload_sprite(
                        image.into_raw().into_boxed_slice(),
                        sprite.width as _,
                        sprite.height as _,
                        sprite.origin_x,
                        sprite.origin_y,
                    )
                    .map_err(|e| gml::Error::FunctionError("sprite_add_from_screen".into(), e.into()))?,
            });
            Ok(Default::default())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_id))
        }
    }

    pub fn sprite_create_from_surface(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, x, y, width, height, transparency, smooth, origin_x, origin_y) =
            expect_args!(args, [int, int, int, int, int, int, bool, int, int])?;
        if self.surface_target == Some(surf_id) {
            self.renderer.flush_queue();
        }
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            let x = x.max(0);
            let y = y.max(0);
            let width = width.min(surf.width as i32 - x);
            let height = height.min(surf.height as i32 - y);
            let (removeback, fill_transparent) = match self.gm_version {
                Version::GameMaker8_0 => (transparency != 0, true),
                Version::GameMaker8_1 => (transparency == 1, transparency != 2),
            };
            let rgba = self.renderer.dump_sprite_part(&surf.atlas_ref, x, y, width, height);
            let mut image = RgbaImage::from_vec(width as _, height as _, rgba.into_vec()).unwrap();
            asset::sprite::process_image(&mut image, removeback, smooth, fill_transparent);
            if self.gm_version == Version::GameMaker8_1 && transparency == -1 {
                // make entire image opaque
                image.pixels_mut().for_each(|p| p[3] = 255);
            }
            let colliders = asset::sprite::make_colliders_precise(std::slice::from_ref(&image), 0, false);
            let frames = vec![asset::sprite::Frame {
                width: width as _,
                height: height as _,
                atlas_ref: self
                    .renderer
                    .upload_sprite(image.into_raw().into_boxed_slice(), width, height, origin_x, origin_y)
                    .map_err(|e| gml::Error::FunctionError("sprite_create_from_surface".into(), e.into()))?,
            }];
            let sprite_id = self.assets.sprites.len();
            self.assets.sprites.push(Some(Box::new(asset::Sprite {
                name: format!("__newsprite{}", sprite_id).into(),
                frames,
                bbox_left: colliders[0].bbox_left,
                bbox_right: colliders[0].bbox_right,
                bbox_top: colliders[0].bbox_top,
                bbox_bottom: colliders[0].bbox_bottom,
                colliders: colliders,
                width: width as _,
                height: height as _,
                origin_x,
                origin_y,
                per_frame_colliders: false,
            })));
            Ok(sprite_id.into())
        } else {
            Err(gml::Error::FunctionError(
                "sprite_create_from_surface".into(),
                format!("Surface {} does not exist", surf_id),
            ))
        }
    }

    pub fn sprite_add_from_surface(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, surf_id, x, y, width, height, removeback, smooth) =
            expect_args!(args, [int, int, int, int, int, int, bool, bool])?;
        if let Some(sprite) = self.assets.sprites.get_asset_mut(sprite_id) {
            if let Some(surf) = self.surfaces.get_asset(surf_id) {
                // get image
                let x = x.max(0);
                let y = y.max(0);
                let width = width.min(surf.width as i32 - x);
                let height = height.min(surf.height as i32 - y);
                let rgba = self.renderer.dump_sprite_part(&surf.atlas_ref, x, y, width, height);
                let mut image = RgbaImage::from_vec(width as _, height as _, rgba.into_vec()).unwrap();
                asset::sprite::process_image(&mut image, removeback, smooth, true);
                asset::sprite::scale(&mut image, sprite.width, sprite.height);
                // generate collision
                let mut images = Vec::with_capacity(sprite.frames.len() + 1);
                // can't use .map() because closures cause borrowing issues
                for f in sprite.frames.iter() {
                    images.push(
                        RgbaImage::from_vec(f.width, f.height, self.renderer.dump_sprite(&f.atlas_ref).into_vec())
                            .unwrap(),
                    );
                }
                images.push(image);
                let sprite = self.assets.sprites.get_asset_mut(sprite_id).unwrap();
                sprite.colliders = asset::sprite::make_colliders_precise(&images, 0, sprite.per_frame_colliders);
                sprite.bbox_left = sprite.colliders.iter().map(|c| c.bbox_left).min().unwrap();
                sprite.bbox_top = sprite.colliders.iter().map(|c| c.bbox_top).min().unwrap();
                sprite.bbox_right = sprite.colliders.iter().map(|c| c.bbox_right).max().unwrap();
                sprite.bbox_bottom = sprite.colliders.iter().map(|c| c.bbox_bottom).max().unwrap();
                // upload frame
                let image = images.pop().unwrap();
                sprite.frames.push(asset::sprite::Frame {
                    width: sprite.width as _,
                    height: sprite.height as _,
                    atlas_ref: self
                        .renderer
                        .upload_sprite(
                            image.into_raw().into_boxed_slice(),
                            sprite.width as _,
                            sprite.height as _,
                            sprite.origin_x,
                            sprite.origin_y,
                        )
                        .map_err(|e| gml::Error::FunctionError("sprite_add_from_surface".into(), e.into()))?,
                });
                Ok(Default::default())
            } else {
                Err(gml::Error::FunctionError(
                    "sprite_add_from_surface".into(),
                    format!("Surface {} does not exist", surf_id),
                ))
            }
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_id))
        }
    }

    pub fn sprite_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (fname, imgnumb, removeback, smooth, origin_x, origin_y) =
            expect_args!(args, [string, int, bool, bool, int, int])?;
        let imgnumb = imgnumb.max(1) as usize;
        let mut images = match file::load_animation(fname.as_ref(), imgnumb) {
            Ok(frames) => frames,
            Err(e) => {
                eprintln!("Warning: sprite_add on {} failed: {}", fname, e);
                return Ok((-1).into())
            },
        };
        for image in images.iter_mut() {
            asset::sprite::process_image(image, removeback, smooth, true);
        }
        let (width, height) = images[0].dimensions();
        // make colliders
        let colliders = asset::sprite::make_colliders_precise(&images, 0, false);
        // collect atlas refs
        // yes i know it's a new texture for every frame like in gm8 but it's fine
        let frames = images
            .drain(..)
            .map(|i| {
                Ok(asset::sprite::Frame {
                    width,
                    height,
                    atlas_ref: self
                        .renderer
                        .upload_sprite(i.into_raw().into_boxed_slice(), width as _, height as _, origin_x, origin_y)
                        .map_err(|e| gml::Error::FunctionError("sprite_add".into(), e.into()))?,
                })
            })
            .collect::<gml::Result<_>>()?;
        let sprite_id = self.assets.sprites.len();
        self.assets.sprites.push(Some(Box::new(asset::Sprite {
            name: format!("__newsprite{}", sprite_id).into(),
            frames,
            bbox_left: colliders[0].bbox_left,
            bbox_right: colliders[0].bbox_right,
            bbox_top: colliders[0].bbox_top,
            bbox_bottom: colliders[0].bbox_bottom,
            colliders,
            width,
            height,
            origin_x,
            origin_y,
            per_frame_colliders: false,
        })));
        Ok(sprite_id.into())
    }

    pub fn sprite_replace(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, fname, imgnumb, removeback, smooth, origin_x, origin_y) =
            expect_args!(args, [int, string, int, bool, bool, int, int])?;
        if let Some(sprite) = self.assets.sprites.get_asset_mut(sprite_id) {
            for frame in &sprite.frames {
                self.renderer.delete_sprite(frame.atlas_ref);
            }
            let imgnumb = imgnumb.max(1) as usize;
            let mut images = match file::load_animation(fname.as_ref(), imgnumb) {
                Ok(frames) => frames,
                Err(e) => {
                    eprintln!("Warning: sprite_replace on {} failed: {}", fname, e);
                    return Ok((-1).into())
                },
            };
            for image in images.iter_mut() {
                asset::sprite::process_image(image, removeback, smooth, true);
            }
            let (width, height) = images[0].dimensions();
            // make colliders
            let colliders = asset::sprite::make_colliders_precise(&images, 0, false);
            // collect atlas refs
            let renderer = &mut self.renderer;
            let frames = images
                .drain(..)
                .map(|i| {
                    Ok(asset::sprite::Frame {
                        width,
                        height,
                        atlas_ref: renderer
                            .upload_sprite(i.into_raw().into_boxed_slice(), width as _, height as _, origin_x, origin_y)
                            .map_err(|e| gml::Error::FunctionError("sprite_replace".into(), e.into()))?,
                    })
                })
                .collect::<gml::Result<_>>()?;
            *sprite = Box::new(asset::Sprite {
                name: sprite.name.clone(),
                frames,
                bbox_left: colliders[0].bbox_left,
                bbox_right: colliders[0].bbox_right,
                bbox_top: colliders[0].bbox_top,
                bbox_bottom: colliders[0].bbox_bottom,
                colliders,
                width,
                height,
                origin_x,
                origin_y,
                per_frame_colliders: false,
            });
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("sprite_replace".into(), "Trying to replace non-existing sprite.".into()))
        }
    }

    pub fn sprite_add_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function sprite_add_sprite");
        Ok(Default::default())
    }

    pub fn sprite_replace_sprite(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function sprite_replace_sprite");
        Ok(Default::default())
    }

    pub fn sprite_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sprite_id = expect_args!(args, [int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_id) {
            for frame in &sprite.frames {
                self.renderer.delete_sprite(frame.atlas_ref);
            }
        } else {
            return Err(gml::Error::FunctionError("sprite_delete".into(), "Trying to delete non-existing sprite".into()))
        }
        self.assets.sprites[sprite_id as usize] = None;
        Ok(Default::default())
    }

    pub fn sprite_duplicate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function sprite_duplicate");
        Ok(Default::default())
    }

    pub fn sprite_assign(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function sprite_assign");
        Ok(Default::default())
    }

    pub fn sprite_merge(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function sprite_merge");
        Ok(Default::default())
    }

    pub fn sprite_save(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, subimg, fname) = expect_args!(args, [int, int, string])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_id) {
            let image_index = subimg % sprite.frames.len() as i32;
            if let Some(frame) = sprite.get_frame(Real::from(image_index)) {
                // get RGBA
                if let Err(e) = file::save_image(
                    fname.as_ref(),
                    RgbaImage::from_vec(frame.width, frame.height, self.renderer.dump_sprite(&frame.atlas_ref).into())
                        .unwrap(),
                ) {
                    return Err(gml::Error::FunctionError("sprite_save".into(), e.to_string()))
                }
            }
        }
        Ok(Default::default())
    }

    pub fn sprite_save_strip(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function sprite_save_strip");
        Ok(Default::default())
    }

    pub fn sprite_collision_mask(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, sepmasks, bboxmode, bbleft, bbtop, bbright, bbbottom, kind, tolerance) =
            expect_args!(args, [int, bool, int, int, int, int, int, int, int])?;
        let tolerance = tolerance.min(255).max(0) as u8;
        let sepmasks = sepmasks;
        if let Some(sprite) = self.assets.sprites.get_asset_mut(sprite_id) {
            // formulate requested bounding box
            let bbox = match bboxmode {
                0 => None, // automatic
                1 => Some(asset::sprite::BoundingBox {
                    // full image
                    left: 0,
                    right: sprite.width - 1,
                    top: 0,
                    bottom: sprite.height - 1,
                }),
                _ => Some(asset::sprite::BoundingBox {
                    // user defined
                    left: bbleft.max(0) as u32,
                    right: (bbright as u32).min(sprite.width),
                    top: bbtop.max(0) as u32,
                    bottom: (bbbottom as u32).min(sprite.height),
                }),
            };

            // download frames from gpu
            let renderer = &mut self.renderer;
            let frames = sprite
                .frames
                .iter()
                .map(|f| RgbaImage::from_vec(f.width, f.height, renderer.dump_sprite(&f.atlas_ref).to_vec()).unwrap())
                .collect::<Vec<RgbaImage>>();

            // make colliders
            sprite.colliders = match kind {
                0 => asset::sprite::make_colliders_precise(&frames, tolerance, sepmasks), // precise
                _ => asset::sprite::make_colliders_shaped(&frames, tolerance, sepmasks, bbox, match kind {
                    1 => Some(asset::sprite::ColliderShape::Rectangle),
                    2 => Some(asset::sprite::ColliderShape::Ellipse),
                    3 => Some(asset::sprite::ColliderShape::Diamond),
                    _ => None,
                }),
            };

            // set bbox variables manually if needed (even if using precise collision)
            if let Some(bbox) = bbox {
                for c in &mut sprite.colliders {
                    c.bbox_left = bbox.left;
                    c.bbox_top = bbox.top;
                    c.bbox_right = bbox.right;
                    c.bbox_bottom = bbox.bottom;
                }
            }
            sprite.bbox_left = sprite.colliders.iter().map(|c| c.bbox_left).min().unwrap();
            sprite.bbox_top = sprite.colliders.iter().map(|c| c.bbox_top).min().unwrap();
            sprite.bbox_right = sprite.colliders.iter().map(|c| c.bbox_right).max().unwrap();
            sprite.bbox_bottom = sprite.colliders.iter().map(|c| c.bbox_bottom).max().unwrap();
        }
        Ok(Default::default())
    }

    pub fn sprite_set_cache_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function sprite_set_cache_size");
        Ok(Default::default())
    }

    pub fn sprite_set_cache_size_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function sprite_set_cache_size_ext");
        Ok(Default::default())
    }

    pub fn background_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.background_get_name(context, args)
    }

    pub fn background_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let background_id = expect_args!(args, [int])?;
        Ok(self.assets.backgrounds.get_asset(background_id).is_some().into())
    }

    pub fn background_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let asset_id = expect_args!(args, [int])?;
        Ok(self.assets.backgrounds.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
    }

    pub fn background_get_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let background_id = expect_args!(args, [int])?;
        if let Some(background) = self.assets.backgrounds.get_asset(background_id) {
            Ok(background.width.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn background_get_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let background_id = expect_args!(args, [int])?;
        if let Some(background) = self.assets.backgrounds.get_asset(background_id) {
            Ok(background.height.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn background_set_alpha_from_background(
        &mut self,
        _context: &mut Context,
        args: &[Value],
    ) -> gml::Result<Value> {
        let (dst_id, src_id) = expect_args!(args, [int, int])?;
        if self.assets.backgrounds.get_asset(dst_id).filter(|bg| bg.atlas_ref.is_some()).is_none() {
            return Ok(Default::default())
        }
        let (alpha_src, src_w) =
            match self.assets.backgrounds.get_asset(src_id).map(|bg| (bg.atlas_ref.as_ref(), bg.width)) {
                Some((Some(atlas_ref), w)) => (self.renderer.dump_sprite(atlas_ref), w),
                _ => return Ok(Default::default()),
            };
        if let Some((Some(atlas_ref), dst_w, dst_h)) =
            self.assets.backgrounds.get_asset_mut(dst_id).map(|bg| (bg.atlas_ref.as_mut(), bg.width, bg.height))
        {
            let mut dst = self.renderer.dump_sprite(atlas_ref);
            self.renderer.delete_sprite(*atlas_ref);
            for (dst_row, src_row) in dst.chunks_mut(dst_w as usize * 4).zip(alpha_src.chunks(src_w as usize * 4)) {
                for (dst_col, src_col) in dst_row.chunks_mut(4).zip(src_row.chunks(4)) {
                    dst_col[3] = src_col[3];
                }
            }
            *atlas_ref = self
                .renderer
                .upload_sprite(dst, dst_w as _, dst_h as _, 0, 0)
                .map_err(|e| gml::Error::FunctionError("background_set_alpha_from_background".into(), e))?;
        }
        Ok(Default::default())
    }

    pub fn background_create_from_screen(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, width, height, removeback, smooth) = expect_args!(args, [int, int, int, int, bool, bool])?;
        let x = x.max(0);
        let y = y.max(0);
        let width = width.min(self.unscaled_width as i32 - x);
        let height = height.min(self.unscaled_height as i32 - y);
        self.renderer.flush_queue();
        let rgba = self.renderer.get_pixels(x, y, width, height);
        let mut image = RgbaImage::from_vec(width as _, height as _, rgba.into_vec()).unwrap();
        asset::sprite::process_image(&mut image, removeback, smooth, true);
        let background_id = self.assets.backgrounds.len();
        self.assets.backgrounds.push(Some(Box::new(asset::Background {
            name: format!("__newbackground{}", background_id).into(),
            width: width as _,
            height: height as _,
            atlas_ref: Some(
                self.renderer
                    .upload_sprite(image.into_raw().into_boxed_slice(), width, height, 0, 0)
                    .map_err(|e| gml::Error::FunctionError("background_create_from_screen".into(), e.into()))?,
            ),
        })));
        Ok(background_id.into())
    }

    pub fn background_create_from_surface(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (surf_id, x, y, width, height, removeback, smooth) =
            expect_args!(args, [int, int, int, int, int, bool, bool])?;
        if self.surface_target == Some(surf_id) {
            self.renderer.flush_queue();
        }
        if let Some(surf) = self.surfaces.get_asset(surf_id) {
            let x = x.max(0);
            let y = y.max(0);
            let width = width.min(surf.width as i32 - x);
            let height = height.min(surf.height as i32 - y);
            let rgba = self.renderer.dump_sprite_part(&surf.atlas_ref, x, y, width, height);
            let mut image = RgbaImage::from_vec(width as _, height as _, rgba.into_vec()).unwrap();
            asset::sprite::process_image(&mut image, removeback, smooth, true);
            let background_id = self.assets.backgrounds.len();
            self.assets.backgrounds.push(Some(Box::new(asset::Background {
                name: format!("__newbackground{}", background_id).into(),
                width: width as _,
                height: height as _,
                atlas_ref: Some(
                    self.renderer
                        .upload_sprite(image.into_raw().into_boxed_slice(), width, height, 0, 0)
                        .map_err(|e| gml::Error::FunctionError("background_create_from_surface".into(), e.into()))?,
                ),
            })));
            Ok(background_id.into())
        } else {
            Err(gml::Error::FunctionError(
                "background_create_from_surface".into(),
                format!("Surface {} does not exist", surf_id),
            ))
        }
    }

    pub fn background_create_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (w, h, col) = expect_args!(args, [int, int, int])?;
        let background_id = self.assets.backgrounds.len();
        self.assets.backgrounds.push(Some(Box::new(asset::Background {
            name: format!("__newbackground{}", background_id).into(),
            width: w as _,
            height: h as _,
            atlas_ref: Some(
                self.renderer
                    .create_sprite_colour(w, h, (col as u32).into())
                    .map_err(|e| gml::Error::FunctionError("background_create_color".into(), e))?,
            ),
        })));
        Ok(background_id.into())
    }

    pub fn background_create_gradient(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function background_create_gradient");
        Ok(Default::default())
    }

    pub fn background_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (fname, removeback, smooth) = expect_args!(args, [string, bool, bool])?;
        let mut image = match file::load_image(fname.as_ref()) {
            Ok(im) => im,
            Err(e) => {
                eprintln!("Warning: background_add on {} failed: {}", fname, e);
                return Ok((-1).into())
            },
        };
        asset::sprite::process_image(&mut image, removeback, smooth, true);
        let width = image.width();
        let height = image.height();
        let atlas_ref = self
            .renderer
            .upload_sprite(image.into_raw().into_boxed_slice(), width as _, height as _, 0, 0)
            .map_err(|e| gml::Error::FunctionError("background_add".into(), e.into()))?;
        let background_id = self.assets.backgrounds.len();
        self.assets.backgrounds.push(Some(Box::new(asset::Background {
            name: format!("__newbackground{}", background_id).into(),
            width,
            height,
            atlas_ref: Some(atlas_ref),
        })));
        Ok(background_id.into())
    }

    pub fn background_replace(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (background_id, fname, removeback, smooth) = expect_args!(args, [int, string, bool, bool])?;
        if let Some(background) = self.assets.backgrounds.get_asset_mut(background_id) {
            if let Some(atlas_ref) = background.atlas_ref {
                self.renderer.delete_sprite(atlas_ref);
            }
            let mut image = match file::load_image(fname.as_ref()) {
                Ok(im) => im,
                Err(e) => {
                    eprintln!("Warning: background_replace on {} failed: {}", fname, e);
                    return Ok((-1).into())
                },
            };
            asset::sprite::process_image(&mut image, removeback, smooth, true);
            let width = image.width();
            let height = image.height();
            let atlas_ref = self
                .renderer
                .upload_sprite(image.into_raw().into_boxed_slice(), width as _, height as _, 0, 0)
                .map_err(|e| gml::Error::FunctionError("background_replace".into(), e.into()))?;
            background.atlas_ref = Some(atlas_ref);
            background.width = width;
            background.height = height;
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError(
                "background_replace".into(),
                "Trying to replace non-existing background.".into(),
            ))
        }
    }

    pub fn background_add_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function background_add_background");
        Ok(Default::default())
    }

    pub fn background_replace_background(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function background_replace_background");
        Ok(Default::default())
    }

    pub fn background_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let background_id = expect_args!(args, [int])?;
        if let Some(background) = self.assets.backgrounds.get_asset(background_id) {
            if let Some(atlas_ref) = background.atlas_ref {
                self.renderer.delete_sprite(atlas_ref);
            }
        } else {
            return Err(gml::Error::FunctionError(
                "background_delete".into(),
                "Trying to delete non-existing background".into(),
            ))
        }
        self.assets.backgrounds[background_id as usize] = None;
        Ok(Default::default())
    }

    pub fn background_duplicate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function background_duplicate");
        Ok(Default::default())
    }

    pub fn background_assign(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (dst_id, src_id) = expect_args!(args, [int, int])?;
        if let Some(src) = self.assets.backgrounds.get_asset(src_id) {
            if let Some(background) = self.assets.backgrounds.get_asset(dst_id) {
                if let Some(atlas_ref) = background.atlas_ref {
                    self.renderer.delete_sprite(atlas_ref);
                }
            }
            if dst_id >= 0 && self.assets.backgrounds.len() > dst_id as usize {
                let dst_atlref = match src.atlas_ref.as_ref() {
                    Some(ar) => Some(
                        self.renderer
                            .duplicate_sprite(ar)
                            .map_err(|e| gml::Error::FunctionError("background_assign".into(), e.into()))?,
                    ),
                    None => None,
                };
                self.assets.backgrounds[dst_id as usize] = Some(Box::new(asset::Background {
                    atlas_ref: dst_atlref,
                    width: src.width,
                    height: src.height,
                    name: src.name.clone(),
                }));
                Ok(Default::default())
            } else {
                Err(gml::Error::FunctionError(
                    "background_assign".into(),
                    "Destination background has an invalid index".into(),
                ))
            }
        } else {
            Err(gml::Error::FunctionError("background_assign".into(), "Source background does not exist".into()))
        }
    }

    pub fn background_save(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (background_id, fname) = expect_args!(args, [int, string])?;
        if let Some(background) = self.assets.backgrounds.get_asset(background_id) {
            if let Some(atlas_ref) = background.atlas_ref.as_ref() {
                // get RGBA
                if let Err(e) = file::save_image(
                    fname.as_ref(),
                    RgbaImage::from_vec(
                        background.width,
                        background.height,
                        self.renderer.dump_sprite(atlas_ref).into(),
                    )
                    .unwrap(),
                ) {
                    return Err(gml::Error::FunctionError("background_save".into(), e.to_string()))
                }
            }
        }
        Ok(Default::default())
    }

    pub fn sound_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.sound_get_name(context, args)
    }

    pub fn sound_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // TODO: uncomment this when there are sounds
        //let sound = expect_args!(args, [int])?;
        //Ok(self.assets.sounds.get_asset(sound).is_some().into())
        todo!()
    }

    pub fn sound_get_name(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // TODO: uncomment this when there are sounds
        //let asset_id = expect_args!(args, [int])?;
        //Ok(self.assets.sounds.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
        todo!()
    }

    pub fn sound_get_kind(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function sound_get_kind");
        Ok(Default::default())
    }

    pub fn sound_get_preload(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function sound_get_preload");
        Ok(Default::default())
    }

    pub fn sound_discard(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function sound_discard")
        Ok(Default::default())
    }

    pub fn sound_restore(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function sound_restore")
        Ok(Default::default())
    }

    pub fn sound_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function sound_add");
        Ok(Default::default())
    }

    pub fn sound_replace(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        //unimplemented!("Called unimplemented kernel function sound_replace")
        Ok(Default::default())
    }

    pub fn sound_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function sound_delete");
        Ok(Default::default())
    }

    pub fn font_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.font_get_name(context, args)
    }

    pub fn font_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let font = expect_args!(args, [int])?;
        Ok(self.assets.fonts.get_asset(font).is_some().into())
    }

    pub fn font_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let asset_id = expect_args!(args, [int])?;
        Ok(self.assets.fonts.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
    }

    pub fn font_get_fontname(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function font_get_fontname");
        Ok(Default::default())
    }

    pub fn font_get_size(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function font_get_size");
        Ok(Default::default())
    }

    pub fn font_get_bold(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function font_get_bold");
        Ok(Default::default())
    }

    pub fn font_get_italic(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function font_get_italic");
        Ok(Default::default())
    }

    pub fn font_get_first(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function font_get_first");
        Ok(Default::default())
    }

    pub fn font_get_last(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function font_get_last");
        Ok(Default::default())
    }

    pub fn font_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function font_add");
        Ok(Default::default())
    }

    pub fn font_replace(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        println!("Called unimplemented kernel function font_replace");
        Ok(Default::default())
    }

    pub fn font_add_sprite(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (sprite_id, first, prop, sep) = expect_args!(args, [int, int, bool, int])?;
        if let Some(sprite) = self.assets.sprites.get_asset(sprite_id) {
            let chars = asset::font::create_chars_from_sprite(sprite, prop, sep, &self.renderer);
            let font_id = self.assets.fonts.len();
            let first = first.max(0).min(255) as _;
            let last = (first as usize + chars.len() - 1).min(255) as _;
            self.assets.fonts.push(Some(Box::new(asset::Font {
                name: format!("__newfont{}", font_id).into(),
                sys_name: "".into(),
                charset: 1,
                size: 12,
                bold: false,
                italic: false,
                first,
                last,
                tallest_char_height: sprite.height,
                chars,
                own_graphics: false,
            })));
            Ok(font_id.into())
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_id))
        }
    }

    pub fn font_replace_sprite(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (font_id, sprite_id, first, prop, sep) = expect_args!(args, [int, int, int, bool, int])?;
        if let Some(font) = self.assets.fonts.get_asset_mut(font_id) {
            if let Some(sprite) = self.assets.sprites.get_asset(sprite_id) {
                if font.own_graphics {
                    // font_add isn't in yet but atm for ttfs all characters are on the same texture
                    if let Some(c) = font.get_char(font.first) {
                        self.renderer.delete_sprite(c.atlas_ref);
                    }
                }
                let chars = asset::font::create_chars_from_sprite(sprite, prop, sep, &self.renderer);
                font.sys_name = "".into();
                font.size = 12;
                font.bold = false;
                font.italic = false;
                font.first = first.max(0).min(255) as _;
                font.last = (first as usize + chars.len() - 1).min(255) as _;
                font.chars = chars;
                font.own_graphics = false;
                Ok(Default::default())
            } else {
                Err(gml::Error::NonexistentAsset(asset::Type::Sprite, sprite_id))
            }
        } else {
            Err(gml::Error::NonexistentAsset(asset::Type::Font, font_id))
        }
    }

    pub fn font_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function font_delete");
        Ok(Default::default())
    }

    pub fn script_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.script_get_name(context, args)
    }

    pub fn script_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let script_id = expect_args!(args, [int])?;
        Ok(self.assets.scripts.get_asset(script_id).is_some().into())
    }

    pub fn script_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let asset_id = expect_args!(args, [int])?;
        Ok(self.assets.scripts.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
    }

    pub fn script_get_text(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let script_id = expect_args!(args, [int])?;
        Ok(self.assets.scripts.get_asset(script_id).map(|s| s.source.clone().into()).unwrap_or("".into()))
    }

    pub fn script_execute(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        if let Some(script_id) = args.get(0) {
            let script_id = script_id.round();
            if let Some(script) = self.assets.scripts.get_asset(script_id) {
                let instructions = script.compiled.clone();
                let mut new_args: [Value; 16] = Default::default();
                for (src, dest) in args[1..].iter().zip(new_args.iter_mut()) {
                    *dest = src.clone();
                }
                let mut new_context = Context {
                    this: context.this,
                    other: context.other,
                    event_action: context.event_action,
                    relative: context.relative,
                    event_type: context.event_type,
                    event_number: context.event_number,
                    event_object: context.event_object,
                    arguments: new_args,
                    argument_count: args.len() - 1,
                    locals: DummyFieldHolder::new(),
                    return_value: Default::default(),
                };
                self.execute(&instructions, &mut new_context)?;
                Ok(new_context.return_value)
            } else {
                Err(gml::Error::NonexistentAsset(asset::Type::Script, script_id))
            }
        } else {
            Err(gml::runtime::Error::WrongArgumentCount(1, 0))
        }
    }

    pub fn path_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.path_get_name(context, args)
    }

    pub fn path_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        Ok(self.assets.paths.get_asset(path_id).is_some().into())
    }

    pub fn path_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let asset_id = expect_args!(args, [int])?;
        Ok(self.assets.paths.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
    }

    pub fn path_get_length(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.length.into()),
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_kind(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.curve.into()),
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_closed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.closed.into()),
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_precision(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.precision.into()),
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_number(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.points.len().into()),
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_point_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, point_id) = expect_args!(args, [int, int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => {
                if point_id < 0 || point_id >= path.points.len() as i32 {
                    Ok(0.into())
                } else {
                    Ok(path.points.get(point_id as usize).unwrap().x.into())
                }
            },
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_point_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, point_id) = expect_args!(args, [int, int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => {
                if point_id < 0 || point_id >= path.points.len() as i32 {
                    Ok(0.into())
                } else {
                    Ok(path.points.get(point_id as usize).unwrap().y.into())
                }
            },
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_point_speed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, point_id) = expect_args!(args, [int, int])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => {
                if point_id < 0 || point_id >= path.points.len() as i32 {
                    Ok(1.into())
                } else {
                    Ok(path.points.get(point_id as usize).unwrap().speed.into())
                }
            },
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, offset) = expect_args!(args, [int, real])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.get_point(offset).x.into()),
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, offset) = expect_args!(args, [int, real])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.get_point(offset).y.into()),
            None => Ok((-1).into()),
        }
    }

    pub fn path_get_speed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, offset) = expect_args!(args, [int, real])?;
        match self.assets.paths.get_asset(path_id) {
            Some(path) => Ok(path.get_point(offset).speed.into()),
            None => Ok((-1).into()),
        }
    }

    pub fn path_set_kind(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, kind) = expect_args!(args, [int, int])?;
        self.assets.paths.get_asset_mut(path_id).map(|path| {
            path.curve = kind == 1;
            path.update();
        });
        Ok(Default::default())
    }

    pub fn path_set_closed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, closed) = expect_args!(args, [int, int])?;
        self.assets.paths.get_asset_mut(path_id).map(|path| {
            path.closed = closed != 0;
            path.update();
        });
        Ok(Default::default())
    }

    pub fn path_set_precision(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, precision) = expect_args!(args, [int, int])?;
        self.assets.paths.get_asset_mut(path_id).map(|path| {
            path.precision = precision.min(8).max(0); // ghetto clamp
            path.update();
        });
        Ok(Default::default())
    }

    pub fn path_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let path_id = self.assets.paths.len();
        self.assets.paths.push(Some(Box::new(asset::Path {
            name: format!("__newpath{}", path_id).into(),
            points: Vec::new(),
            control_nodes: Default::default(),
            length: Default::default(),
            curve: false,
            closed: false,
            precision: 4,
            start: Default::default(),
            end: Default::default(),
        })));
        Ok(path_id.into())
    }

    pub fn path_duplicate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function path_duplicate");
        Ok(Default::default())
    }

    pub fn path_assign(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function path_assign");
        Ok(Default::default())
    }

    pub fn path_append(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function path_append");
        Ok(Default::default())
    }

    pub fn path_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let path_id = expect_args!(args, [int])?;
        if self.assets.paths.get_asset(path_id).is_some() {
            self.assets.paths[path_id as usize] = None;
        }
        Ok(Default::default())
    }

    pub fn path_add_point(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, x, y, speed) = expect_args!(args, [int, real, real, real])?;
        if let Some(path) = self.assets.paths.get_asset_mut(path_id) {
            path.points.push(asset::path::Point { x, y, speed });
            path.update();
        }
        Ok(Default::default())
    }

    pub fn path_insert_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function path_insert_point");
        Ok(Default::default())
    }

    pub fn path_change_point(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (path_id, n, x, y, speed) = expect_args!(args, [int, int, real, real, real])?;
        if n >= 0 {
            if let Some(path) = self.assets.paths.get_asset_mut(path_id) {
                if let Some(point) = path.points.get_mut(n as usize) {
                    point.x = x;
                    point.y = y;
                    point.speed = speed;
                    path.update();
                }
            }
        }
        Ok(Default::default())
    }

    pub fn path_delete_point(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function path_delete_point");
        Ok(Default::default())
    }

    pub fn path_clear_points(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function path_clear_points");
        Ok(Default::default())
    }

    pub fn path_reverse(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function path_reverse");
        Ok(Default::default())
    }

    pub fn path_mirror(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function path_mirror");
        Ok(Default::default())
    }

    pub fn path_flip(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function path_flip");
        Ok(Default::default())
    }

    pub fn path_rotate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function path_rotate");
        Ok(Default::default())
    }

    pub fn path_scale(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function path_scale");
        Ok(Default::default())
    }

    pub fn path_shift(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function path_shift");
        Ok(Default::default())
    }

    pub fn timeline_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.timeline_get_name(context, args)
    }

    pub fn timeline_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let timeline = expect_args!(args, [int])?;
        Ok(self.assets.timelines.get_asset(timeline).is_some().into())
    }

    pub fn timeline_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let asset_id = expect_args!(args, [int])?;
        Ok(self.assets.timelines.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
    }

    pub fn timeline_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function timeline_add");
        Ok(Default::default())
    }

    pub fn timeline_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function timeline_delete");
        Ok(Default::default())
    }

    pub fn timeline_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function timeline_clear");
        Ok(Default::default())
    }

    pub fn timeline_moment_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function timeline_moment_clear");
        Ok(Default::default())
    }

    pub fn timeline_moment_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function timeline_moment_add");
        Ok(Default::default())
    }

    pub fn object_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.object_get_name(context, args)
    }

    pub fn object_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object = expect_args!(args, [int])?;
        Ok(self.assets.objects.get_asset(object).is_some().into())
    }

    pub fn object_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let asset_id = expect_args!(args, [int])?;
        Ok(self.assets.objects.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
    }

    pub fn object_get_sprite(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            Ok(object.sprite_index.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn object_get_solid(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            Ok(object.solid.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn object_get_visible(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            Ok(object.visible.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn object_get_depth(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            Ok(object.depth.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn object_get_persistent(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            Ok(object.persistent.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn object_get_mask(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            Ok(object.mask_index.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn object_get_parent(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let object_id = expect_args!(args, [int])?;
        if let Some(Some(object)) = self.assets.objects.get(object_id as usize) {
            Ok(object.parent_index.into())
        } else {
            Ok((-1).into())
        }
    }

    pub fn object_is_ancestor(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (child_id, parent_id) = expect_args!(args, [int, int])?;
        if child_id != parent_id {
            if let Some(parent) = self.assets.objects.get_asset(parent_id) {
                return Ok(parent.children.borrow().contains(&child_id).into())
            }
        }
        Ok(false.into())
    }

    pub fn object_set_sprite(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, sprite_id) = expect_args!(args, [int, int])?;
        self.assets.objects.get_asset_mut(object_id).map(|o| o.sprite_index = sprite_id);
        Ok(Default::default())
    }

    pub fn object_set_solid(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, visible) = expect_args!(args, [int, bool])?;
        self.assets.objects.get_asset_mut(object_id).map(|o| o.visible = visible);
        Ok(Default::default())
    }

    pub fn object_set_visible(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, visible) = expect_args!(args, [int, bool])?;
        self.assets.objects.get_asset_mut(object_id).map(|o| o.visible = visible);
        Ok(Default::default())
    }

    pub fn object_set_depth(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, depth) = expect_args!(args, [int, int])?;
        self.assets.objects.get_asset_mut(object_id).map(|o| o.depth = depth);
        Ok(Default::default())
    }

    pub fn object_set_persistent(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, persistent) = expect_args!(args, [int, bool])?;
        self.assets.objects.get_asset_mut(object_id).map(|o| o.persistent = persistent);
        Ok(Default::default())
    }

    pub fn object_set_mask(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, mask_id) = expect_args!(args, [int, int])?;
        self.assets.objects.get_asset_mut(object_id).map(|o| o.mask_index = mask_id);
        Ok(Default::default())
    }

    pub fn object_set_parent(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_id, new_parent) = expect_args!(args, [int, int])?;
        if let Some(object) = self.assets.objects.get_asset(object_id) {
            // Remove object and all its children from old parents
            let children = object.children.borrow();
            let mut parent_index = object.parent_index;
            while let Some(parent) = self.assets.objects.get_asset(parent_index) {
                parent.children.borrow_mut().retain(|c| !children.contains(c));
                parent_index = parent.parent_index;
            }
            // Add object and all its children to new parents
            parent_index = new_parent;
            while let Some(parent) = self.assets.objects.get_asset(parent_index) {
                parent.children.borrow_mut().extend(children.iter());
                parent_index = parent.parent_index;
            }
        }

        self.assets.objects.get_asset_mut(object_id).map(|o| o.parent_index = new_parent);
        self.refresh_event_holders();
        Ok(Default::default())
    }

    pub fn object_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let id = self.assets.objects.len() as i32;
        let children = Default::default();
        let object = Box::new(asset::Object {
            name: format!("__newobject{}", id).into(),
            solid: false,
            visible: true,
            persistent: false,
            depth: 0,
            sprite_index: -1,
            mask_index: -1,
            parent_index: -1,
            events: Default::default(),
            children,
        });
        object.children.borrow_mut().insert(id);
        self.assets.objects.push(Some(object));
        Ok(id.into())
    }

    pub fn object_delete(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function object_delete");
        Ok(Default::default())
    }

    pub fn object_event_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_index, ev_type, ev_number) = expect_args!(args, [int, int, int])?;
        if let Some(object) = self.assets.objects.get_asset_mut(object_index) {
            object.events[ev_type as usize].remove(&(ev_number as u32));
            self.refresh_event_holders();
        }
        Ok(Default::default())
    }

    pub fn object_event_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (object_index, ev_type, ev_number, code) = expect_args!(args, [int, int, int, bytes])?;
        if let Some(object) = self.assets.objects.get_asset_mut(object_index) {
            let instrs = match self.compiler.compile(code.as_ref()) {
                Ok(instrs) => instrs,
                Err(e) => return Err(gml::Error::FunctionError("object_event_add".into(), e.message)),
            };
            let object_event_map = &mut object.events[ev_type as usize];
            match object_event_map.get_mut(&(ev_number as u32)) {
                Some(tree) => {
                    tree.borrow_mut().push_code(instrs);
                },
                None => {
                    object_event_map.insert(ev_number as u32, action::Tree::new_from_code(instrs));
                    self.refresh_event_holders();
                },
            }
        }
        Ok(Default::default())
    }

    pub fn room_name(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.room_get_name(context, args)
    }

    pub fn room_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let room_id = expect_args!(args, [int])?;
        Ok(self.assets.rooms.get_asset(room_id).is_some().into())
    }

    pub fn room_get_name(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let asset_id = expect_args!(args, [int])?;
        Ok(self.assets.rooms.get_asset(asset_id).map(|x| x.name.clone().into()).unwrap_or("<undefined>".into()))
    }

    pub fn room_set_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (room_id, width) = expect_args!(args, [int, int])?;
        if let Some(room) = self.assets.rooms.get_asset_mut(room_id) {
            room.width = width as _;
        }
        Ok(Default::default())
    }

    pub fn room_set_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (room_id, height) = expect_args!(args, [int, int])?;
        if let Some(room) = self.assets.rooms.get_asset_mut(room_id) {
            room.height = height as _;
        }
        Ok(Default::default())
    }

    pub fn room_set_caption(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (room_id, caption) = expect_args!(args, [int, bytes])?;
        if let Some(room) = self.assets.rooms.get_asset_mut(room_id) {
            room.caption = caption;
        }
        Ok(Default::default())
    }

    pub fn room_set_persistent(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function room_set_persistent");
        Ok(Default::default())
    }

    pub fn room_set_code(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function room_set_code");
        Ok(Default::default())
    }

    pub fn room_set_background_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (room_id, colour, show) = expect_args!(args, [int, int, bool])?;
        if let Some(room) = self.assets.rooms.get_asset_mut(room_id) {
            room.bg_colour = (colour as u32).into();
            room.clear_screen = show;
        }
        Ok(Default::default())
    }

    pub fn room_set_background(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (room_id, bg, visible, is_foreground, bg_id, x, y, htiled, vtiled, hspeed, vspeed, alpha) =
            expect_args!(args, [int, int, bool, bool, int, real, real, bool, bool, real, real, real])?;
        if bg >= 0 {
            if let Some(room) = self.assets.rooms.get_asset_mut(room_id) {
                if let Some(bg) = room.backgrounds.get_mut(bg as usize) {
                    bg.visible = visible;
                    bg.is_foreground = is_foreground;
                    bg.background_id = bg_id;
                    bg.x_offset = x;
                    bg.y_offset = y;
                    bg.tile_horizontal = htiled;
                    bg.tile_vertical = vtiled;
                    bg.hspeed = hspeed;
                    bg.vspeed = vspeed;
                    bg.alpha = alpha;
                }
            }
        }
        Ok(Default::default())
    }

    pub fn room_set_view(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (
            room_id,
            view_id,
            visible,
            source_x,
            source_y,
            source_w,
            source_h,
            port_x,
            port_y,
            port_w,
            port_h,
            follow_hborder,
            follow_vborder,
            follow_hspeed,
            follow_vspeed,
            follow_target,
        ) = expect_args!(args, [int, int, bool, int, int, int, int, int, int, int, int, int, int, int, int, int])?;
        let view_id = if view_id >= 0 { view_id as usize } else { return Ok(Default::default()) };
        if let Some(room) = self.assets.rooms.get_asset_mut(room_id) {
            if let Some(view) = room.views.get_mut(view_id) {
                *view = View {
                    visible: visible,
                    source_x,
                    source_y,
                    source_w: source_w as _,
                    source_h: source_h as _,
                    port_x,
                    port_y,
                    port_w: port_w as _,
                    port_h: port_h as _,
                    follow_hborder,
                    follow_vborder,
                    follow_hspeed,
                    follow_vspeed,
                    follow_target,
                    angle: view.angle,
                };
            }
        }
        Ok(Default::default())
    }

    pub fn room_set_view_enabled(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (room_id, enabled) = expect_args!(args, [int, bool])?;
        if let Some(room) = self.assets.rooms.get_asset_mut(room_id) {
            room.views_enabled = enabled;
        }
        Ok(Default::default())
    }

    pub fn room_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function room_add");
        Ok(Default::default())
    }

    pub fn room_duplicate(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function room_duplicate");
        Ok(Default::default())
    }

    pub fn room_assign(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function room_assign");
        Ok(Default::default())
    }

    pub fn room_instance_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function room_instance_add");
        Ok(Default::default())
    }

    pub fn room_instance_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function room_instance_clear");
        Ok(Default::default())
    }

    pub fn room_tile_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 9
        println!("Called unimplemented kernel function room_tile_add");
        Ok(Default::default())
    }

    pub fn room_tile_add_ext(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 12
        println!("Called unimplemented kernel function room_tile_add_ext");
        Ok(Default::default())
    }

    pub fn room_tile_clear(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function room_tile_clear");
        Ok(Default::default())
    }

    pub fn part_type_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.particles.create_type().into())
    }

    pub fn part_type_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        self.particles.destroy_type(id);
        Ok(Default::default())
    }

    pub fn part_type_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        Ok(self.particles.get_type(id).is_some().into())
    }

    pub fn part_type_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            *pt = Box::new(particle::ParticleType::new());
        }
        Ok(Default::default())
    }

    pub fn part_type_shape(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, shape) = expect_args!(args, [int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.graphic = particle::ParticleGraphic::Shape(shape);
        }
        Ok(Default::default())
    }

    pub fn part_type_sprite(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, sprite, animat, stretch, random) = expect_args!(args, [int, int, bool, bool, bool])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.graphic = particle::ParticleGraphic::Sprite { sprite, animat, stretch, random };
        }
        Ok(Default::default())
    }

    pub fn part_type_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, size_min, size_max, size_incr, size_wiggle) = expect_args!(args, [int, real, real, real, real])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.size_min = size_min;
            pt.size_max = size_max;
            pt.size_incr = size_incr;
            pt.size_wiggle = size_wiggle;
        }
        Ok(Default::default())
    }

    pub fn part_type_scale(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, xscale, yscale) = expect_args!(args, [int, real, real])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.xscale = xscale;
            pt.yscale = yscale;
        }
        Ok(Default::default())
    }

    pub fn part_type_life(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, life_min, life_max) = expect_args!(args, [int, int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.life_min = life_min;
            pt.life_max = life_max;
        }
        Ok(Default::default())
    }

    pub fn part_type_step(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, step_number, step_type) = expect_args!(args, [int, int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.step_number = step_number;
            pt.step_type = step_type;
        }
        Ok(Default::default())
    }

    pub fn part_type_death(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, death_number, death_type) = expect_args!(args, [int, int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.death_number = death_number;
            pt.death_type = death_type;
        }
        Ok(Default::default())
    }

    pub fn part_type_speed(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, speed_min, speed_max, speed_incr, speed_wiggle) = expect_args!(args, [int, real, real, real, real])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.speed_min = speed_min;
            pt.speed_max = speed_max;
            pt.speed_incr = speed_incr;
            pt.speed_wiggle = speed_wiggle;
        }
        Ok(Default::default())
    }

    pub fn part_type_direction(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, dir_min, dir_max, dir_incr, dir_wiggle) = expect_args!(args, [int, real, real, real, real])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.dir_min = dir_min;
            pt.dir_max = dir_max;
            pt.dir_incr = dir_incr;
            pt.dir_wiggle = dir_wiggle;
        }
        Ok(Default::default())
    }

    pub fn part_type_orientation(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, ang_min, ang_max, ang_incr, ang_wiggle, ang_relative) =
            expect_args!(args, [int, real, real, real, real, bool])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.ang_min = ang_min;
            pt.ang_max = ang_max;
            pt.ang_incr = ang_incr;
            pt.ang_wiggle = ang_wiggle;
            pt.ang_relative = ang_relative;
        }
        Ok(Default::default())
    }

    pub fn part_type_gravity(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, grav_amount, grav_dir) = expect_args!(args, [int, real, real])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.grav_amount = grav_amount;
            pt.grav_dir = grav_dir.rem_euclid(Real::from(360.0));
        }
        Ok(Default::default())
    }

    pub fn part_type_color_mix(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, c1, c2) = expect_args!(args, [int, int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.colour = particle::ParticleColour::Mix(c1, c2);
        }
        Ok(Default::default())
    }

    pub fn part_type_color_rgb(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, rmin, rmax, gmin, gmax, bmin, bmax) = expect_args!(args, [int, int, int, int, int, int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.colour = particle::ParticleColour::RGB { rmin, rmax, gmin, gmax, bmin, bmax };
        }
        Ok(Default::default())
    }

    pub fn part_type_color_hsv(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, hmin, hmax, smin, smax, vmin, vmax) = expect_args!(args, [int, int, int, int, int, int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.colour = particle::ParticleColour::HSV { hmin, hmax, smin, smax, vmin, vmax };
        }
        Ok(Default::default())
    }

    pub fn part_type_color1(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, col) = expect_args!(args, [int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.colour = particle::ParticleColour::One(col);
        }
        Ok(Default::default())
    }

    pub fn part_type_color2(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, c1, c2) = expect_args!(args, [int, int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.colour = particle::ParticleColour::Two(c1, c2);
        }
        Ok(Default::default())
    }

    pub fn part_type_color3(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, c1, c2, c3) = expect_args!(args, [int, int, int, int])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.colour = particle::ParticleColour::Three(c1, c2, c3);
        }
        Ok(Default::default())
    }

    pub fn part_type_color(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.part_type_color3(context, args)
    }

    pub fn part_type_alpha1(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, alpha) = expect_args!(args, [int, real])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.alpha1 = alpha;
            pt.alpha2 = alpha;
            pt.alpha3 = alpha;
        }
        Ok(Default::default())
    }

    pub fn part_type_alpha2(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, alpha1, alpha2) = expect_args!(args, [int, real, real])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.alpha1 = alpha1;
            pt.alpha2 = (alpha1 + alpha2) / Real::from(2.0);
            pt.alpha3 = alpha2;
        }
        Ok(Default::default())
    }

    pub fn part_type_alpha3(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, alpha1, alpha2, alpha3) = expect_args!(args, [int, real, real, real])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.alpha1 = alpha1;
            pt.alpha2 = alpha2;
            pt.alpha3 = alpha3;
        }
        Ok(Default::default())
    }

    pub fn part_type_alpha(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.part_type_alpha3(context, args)
    }

    pub fn part_type_blend(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, additive) = expect_args!(args, [int, bool])?;
        if let Some(pt) = self.particles.get_type_mut(id) {
            pt.additive_blending = additive;
        }
        Ok(Default::default())
    }

    pub fn part_system_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.particles.create_system().into())
    }

    pub fn part_system_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        self.particles.destroy_system(id);
        Ok(Default::default())
    }

    pub fn part_system_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        Ok(self.particles.get_system(id).is_some().into())
    }

    pub fn part_system_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            *ps = Box::new(particle::System::new());
        }
        Ok(Default::default())
    }

    pub fn part_system_draw_order(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, oldtonew) = expect_args!(args, [int, bool])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            ps.draw_old_to_new = oldtonew;
        }
        Ok(Default::default())
    }

    pub fn part_system_depth(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, depth) = expect_args!(args, [int, real])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            ps.depth = depth;
        }
        Ok(Default::default())
    }

    pub fn part_system_position(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, x, y) = expect_args!(args, [int, real, real])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            ps.x = x;
            ps.y = y;
        }
        Ok(Default::default())
    }

    pub fn part_system_automatic_update(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, automatic) = expect_args!(args, [int, bool])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            ps.auto_update = automatic;
        }
        Ok(Default::default())
    }

    pub fn part_system_automatic_draw(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, automatic) = expect_args!(args, [int, bool])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            ps.auto_draw = automatic;
        }
        Ok(Default::default())
    }

    pub fn part_system_update(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        self.particles.update_system(id, &mut self.rand);
        Ok(Default::default())
    }

    pub fn part_system_drawit(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        self.particles.draw_system(id, &mut self.renderer, &self.assets, false);
        Ok(Default::default())
    }

    pub fn part_particles_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, x, y, parttype, number) = expect_args!(args, [int, real, real, int, int])?;
        self.particles.system_create_particles(id, x, y, parttype, None, number, &mut self.rand);
        Ok(Default::default())
    }

    pub fn part_particles_create_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, x, y, parttype, colour, number) = expect_args!(args, [int, real, real, int, int, int])?;
        self.particles.system_create_particles(id, x, y, parttype, Some(colour), number, &mut self.rand);
        Ok(Default::default())
    }

    pub fn part_particles_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            ps.particles.clear();
        }
        Ok(Default::default())
    }

    pub fn part_particles_count(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system(id) {
            Ok(ps.particles.len().into())
        } else {
            Ok(Default::default())
        }
    }

    pub fn part_emitter_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            let em = particle::Emitter::new();
            if let Some(id) = ps.emitters.iter().position(|x| x.is_none()) {
                ps.emitters[id] = Some(em);
                Ok(id.into())
            } else {
                ps.emitters.push(Some(em));
                Ok((ps.emitters.len() - 1).into())
            }
        } else {
            Ok((-1).into())
        }
    }

    pub fn part_emitter_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if ps.emitters.get_asset(id).is_some() {
                ps.emitters[id as usize] = None;
            }
        }
        Ok(Default::default())
    }

    pub fn part_emitter_destroy_all(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let psid = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            ps.emitters.clear();
        }
        Ok(Default::default())
    }

    pub fn part_emitter_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system(psid) {
            Ok(ps.emitters.get_asset(id).is_some().into())
        } else {
            Ok(gml::FALSE.into())
        }
    }

    pub fn part_emitter_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(em) = ps.emitters.get_asset_mut(id) {
                *em = particle::Emitter::new();
            }
        }
        Ok(Default::default())
    }

    pub fn part_emitter_region(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, xmin, xmax, ymin, ymax, shape, distr) =
            expect_args!(args, [int, int, real, real, real, real, int, int])?;
        let shape = match shape {
            1 => particle::Shape::Ellipse,
            2 => particle::Shape::Diamond,
            3 => particle::Shape::Line,
            _ => particle::Shape::Rectangle,
        };
        let distr = match distr {
            1 => particle::Distribution::Gaussian,
            2 => particle::Distribution::InvGaussian,
            _ => particle::Distribution::Linear,
        };
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(em) = ps.emitters.get_asset_mut(id) {
                em.xmin = xmin;
                em.xmax = xmax;
                em.ymin = ymin;
                em.ymax = ymax;
                em.shape = shape;
                em.distribution = distr;
            }
        }
        Ok(Default::default())
    }

    pub fn part_emitter_burst(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, parttype, number) = expect_args!(args, [int, int, int, int])?;
        self.particles.emitter_burst(psid, id, parttype, number, &mut self.rand);
        Ok(Default::default())
    }

    pub fn part_emitter_stream(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, parttype, number) = expect_args!(args, [int, int, int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(em) = ps.emitters.get_asset_mut(id) {
                em.ptype = parttype;
                em.number = number;
            }
        }
        Ok(Default::default())
    }

    pub fn part_attractor_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            let at = particle::Attractor::new();
            if let Some(id) = ps.attractors.iter().position(|x| x.is_none()) {
                ps.attractors[id] = Some(at);
                Ok(id.into())
            } else {
                ps.attractors.push(Some(at));
                Ok((ps.attractors.len() - 1).into())
            }
        } else {
            Ok((-1).into())
        }
    }

    pub fn part_attractor_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if ps.attractors.get_asset(id).is_some() {
                ps.attractors[id as usize] = None;
            }
        }
        Ok(Default::default())
    }

    pub fn part_attractor_destroy_all(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let psid = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            ps.attractors.clear();
        }
        Ok(Default::default())
    }

    pub fn part_attractor_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system(psid) {
            Ok(ps.attractors.get_asset(id).is_some().into())
        } else {
            Ok(gml::FALSE.into())
        }
    }

    pub fn part_attractor_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(at) = ps.attractors.get_asset_mut(id) {
                *at = particle::Attractor::new();
            }
        }
        Ok(Default::default())
    }

    pub fn part_attractor_position(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, x, y) = expect_args!(args, [int, int, real, real])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(at) = ps.attractors.get_asset_mut(id) {
                at.x = x;
                at.y = y;
            }
        }
        Ok(Default::default())
    }

    pub fn part_attractor_force(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, force, dist, kind, additive) = expect_args!(args, [int, int, real, real, int, bool])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(at) = ps.attractors.get_asset_mut(id) {
                at.force = force;
                at.dist = dist;
                at.kind = match kind {
                    1 => particle::ForceKind::Linear,
                    2 => particle::ForceKind::Quadratic,
                    _ => particle::ForceKind::Constant,
                };
                at.additive = additive;
            }
        }
        Ok(Default::default())
    }

    pub fn part_destroyer_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            let de = particle::Destroyer::new();
            if let Some(id) = ps.destroyers.iter().position(|x| x.is_none()) {
                ps.destroyers[id] = Some(de);
                Ok(id.into())
            } else {
                ps.destroyers.push(Some(de));
                Ok((ps.destroyers.len() - 1).into())
            }
        } else {
            Ok((-1).into())
        }
    }

    pub fn part_destroyer_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if ps.destroyers.get_asset(id).is_some() {
                ps.destroyers[id as usize] = None;
            }
        }
        Ok(Default::default())
    }

    pub fn part_destroyer_destroy_all(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let psid = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            ps.destroyers.clear();
        }
        Ok(Default::default())
    }

    pub fn part_destroyer_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system(psid) {
            Ok(ps.destroyers.get_asset(id).is_some().into())
        } else {
            Ok(gml::FALSE.into())
        }
    }

    pub fn part_destroyer_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(de) = ps.destroyers.get_asset_mut(id) {
                *de = particle::Destroyer::new();
            }
        }
        Ok(Default::default())
    }

    pub fn part_destroyer_region(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, xmin, xmax, ymin, ymax, shape) = expect_args!(args, [int, int, real, real, real, real, int])?;
        let shape = match shape {
            1 => particle::Shape::Ellipse,
            2 => particle::Shape::Diamond,
            _ => particle::Shape::Rectangle,
        };
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(de) = ps.destroyers.get_asset_mut(id) {
                de.xmin = xmin;
                de.xmax = xmax;
                de.ymin = ymin;
                de.ymax = ymax;
                de.shape = shape;
            }
        }
        Ok(Default::default())
    }

    pub fn part_deflector_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            let de = particle::Deflector::new();
            if let Some(id) = ps.deflectors.iter().position(|x| x.is_none()) {
                ps.deflectors[id] = Some(de);
                Ok(id.into())
            } else {
                ps.deflectors.push(Some(de));
                Ok((ps.deflectors.len() - 1).into())
            }
        } else {
            Ok((-1).into())
        }
    }

    pub fn part_deflector_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if ps.deflectors.get_asset(id).is_some() {
                ps.deflectors[id as usize] = None;
            }
        }
        Ok(Default::default())
    }

    pub fn part_deflector_destroy_all(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let psid = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            ps.deflectors.clear();
        }
        Ok(Default::default())
    }

    pub fn part_deflector_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system(psid) {
            Ok(ps.deflectors.get_asset(id).is_some().into())
        } else {
            Ok(gml::FALSE.into())
        }
    }

    pub fn part_deflector_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(de) = ps.deflectors.get_asset_mut(id) {
                *de = particle::Deflector::new();
            }
        }
        Ok(Default::default())
    }

    pub fn part_deflector_region(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, xmin, xmax, ymin, ymax) = expect_args!(args, [int, int, real, real, real, real])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(de) = ps.deflectors.get_asset_mut(id) {
                de.xmin = xmin;
                de.xmax = xmax;
                de.ymin = ymin;
                de.ymax = ymax;
            }
        }
        Ok(Default::default())
    }

    pub fn part_deflector_kind(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, kind) = expect_args!(args, [int, int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(de) = ps.deflectors.get_asset_mut(id) {
                de.kind = match kind {
                    1 => particle::DeflectorKind::Horizontal,
                    _ => particle::DeflectorKind::Vertical,
                }
            }
        }
        Ok(Default::default())
    }

    pub fn part_deflector_friction(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, friction) = expect_args!(args, [int, int, real])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(de) = ps.deflectors.get_asset_mut(id) {
                de.friction = friction;
            }
        }
        Ok(Default::default())
    }

    pub fn part_changer_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(id) {
            let ch = particle::Changer::new();
            if let Some(id) = ps.changers.iter().position(|x| x.is_none()) {
                ps.changers[id] = Some(ch);
                Ok(id.into())
            } else {
                ps.changers.push(Some(ch));
                Ok((ps.changers.len() - 1).into())
            }
        } else {
            Ok((-1).into())
        }
    }

    pub fn part_changer_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if ps.changers.get_asset(id).is_some() {
                ps.changers[id as usize] = None;
            }
        }
        Ok(Default::default())
    }

    pub fn part_changer_destroy_all(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let psid = expect_args!(args, [int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            ps.changers.clear();
        }
        Ok(Default::default())
    }

    pub fn part_changer_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system(psid) {
            Ok(ps.changers.get_asset(id).is_some().into())
        } else {
            Ok(gml::FALSE.into())
        }
    }

    pub fn part_changer_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id) = expect_args!(args, [int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(ch) = ps.changers.get_asset_mut(id) {
                *ch = particle::Changer::new();
            }
        }
        Ok(Default::default())
    }

    pub fn part_changer_region(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, xmin, xmax, ymin, ymax, shape) = expect_args!(args, [int, int, real, real, real, real, int])?;
        let shape = match shape {
            1 => particle::Shape::Ellipse,
            2 => particle::Shape::Diamond,
            _ => particle::Shape::Rectangle,
        };
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(ch) = ps.changers.get_asset_mut(id) {
                ch.xmin = xmin;
                ch.xmax = xmax;
                ch.ymin = ymin;
                ch.ymax = ymax;
                ch.shape = shape;
            }
        }
        Ok(Default::default())
    }

    pub fn part_changer_kind(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, kind) = expect_args!(args, [int, int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(ch) = ps.changers.get_asset_mut(id) {
                ch.kind = match kind {
                    0 => particle::ChangerKind::All,
                    1 => particle::ChangerKind::Shape,
                    _ => particle::ChangerKind::Motion,
                };
            }
        }
        Ok(Default::default())
    }

    pub fn part_changer_types(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (psid, id, parttype1, parttype2) = expect_args!(args, [int, int, int, int])?;
        if let Some(ps) = self.particles.get_system_mut(psid) {
            if let Some(ch) = ps.changers.get_asset_mut(id) {
                ch.parttype1 = parttype1;
                ch.parttype2 = parttype2;
            }
        }
        Ok(Default::default())
    }

    pub fn effect_create_below(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (kind, x, y, size, colour) = expect_args!(args, [any, any, any, any, any])?;
        self.action_effect(context, &[kind, x, y, size, colour, gml::TRUE.into()])
    }

    pub fn effect_create_above(&mut self, context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (kind, x, y, size, colour) = expect_args!(args, [any, any, any, any, any])?;
        self.action_effect(context, &[kind, x, y, size, colour, gml::FALSE.into()])
    }

    pub fn effect_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.particles.effect_clear();
        Ok(Default::default())
    }

    pub fn ds_set_precision(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        self.ds_precision = expect_args!(args, [real])?;
        Ok(Default::default())
    }

    pub fn ds_stack_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.stacks.put(ds::Stack::new()).into())
    }

    pub fn ds_stack_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if self.stacks.delete(id) {
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("ds_stack_destroy".into(), ds::Error::NonexistentStructure(id).into()))
        }
    }

    pub fn ds_stack_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.stacks.get_mut(id) {
            Some(stack) => {
                stack.clear();
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_stack_clear".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_stack_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, src_id) = expect_args!(args, [int, int])?;
        let src = match self.stacks.get(src_id) {
            Some(stack) => stack.clone(),
            None => {
                return Err(gml::Error::FunctionError(
                    "ds_stack_copy".into(),
                    ds::Error::NonexistentStructure(src_id).into(),
                ))
            },
        };
        match self.stacks.get_mut(id) {
            Some(stack) => {
                *stack = src;
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_stack_copy".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_stack_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.stacks.get(id) {
            Some(stack) => Ok(stack.len().into()),
            None => Err(gml::Error::FunctionError("ds_stack_size".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_stack_empty(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.stacks.get(id) {
            Some(stack) => Ok(stack.is_empty().into()),
            None => Err(gml::Error::FunctionError("ds_stack_empty".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_stack_push(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val) = expect_args!(args, [int, any])?;
        match self.stacks.get_mut(id) {
            Some(stack) => {
                stack.push(val);
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_stack_push".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_stack_pop(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.stacks.get_mut(id) {
            Some(stack) => Ok(stack.pop().unwrap_or_default()),
            None => Err(gml::Error::FunctionError("ds_stack_pop".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_stack_top(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.stacks.get(id) {
            Some(stack) => Ok(stack.last().map(Value::clone).unwrap_or_default()),
            None => Err(gml::Error::FunctionError("ds_stack_top".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_stack_write(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.stacks.get_mut(id) {
            Some(stack) => {
                let mut output = "65000000".to_string();
                output.push_str(&hex::encode_upper((stack.len() as u32).to_le_bytes()));
                output.extend(stack.iter().map(|v| hex::encode_upper(v.as_bytes())));
                Ok(output.into())
            },
            None => Err(gml::Error::FunctionError("ds_stack_write".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_stack_read(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, hex_data) = expect_args!(args, [int, string])?;
        match self.stacks.get_mut(id) {
            Some(old_stack) => {
                match hex::decode(hex_data.as_ref()) {
                    Ok(data) => {
                        let mut reader = data.as_slice();
                        // Read header and size
                        let mut buf = [0u8; 4];
                        if reader.read_exact(&mut buf).is_ok()
                            && u32::from_le_bytes(buf) == 0x65
                            && reader.read_exact(&mut buf).is_ok()
                        {
                            let size = u32::from_le_bytes(buf) as usize;
                            // Read each item
                            let mut stack = ds::Stack::with_capacity(size);
                            for _ in 0..size {
                                if let Some(val) = Value::from_reader(&mut reader) {
                                    stack.push(val);
                                } else {
                                    return Ok(Default::default())
                                }
                            }
                            *old_stack = stack;
                        }
                    },
                    Err(e) => eprintln!("Warning (ds_stack_read): {}", e),
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_stack_read".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_queue_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.queues.put(ds::Queue::new()).into())
    }

    pub fn ds_queue_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if self.queues.delete(id) {
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("ds_queue_destroy".into(), ds::Error::NonexistentStructure(id).into()))
        }
    }

    pub fn ds_queue_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.queues.get_mut(id) {
            Some(queue) => {
                queue.clear();
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_queue_clear".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_queue_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, src_id) = expect_args!(args, [int, int])?;
        let src = match self.queues.get(src_id) {
            Some(queue) => queue.clone(),
            None => {
                return Err(gml::Error::FunctionError(
                    "ds_queue_copy".into(),
                    ds::Error::NonexistentStructure(src_id).into(),
                ))
            },
        };
        match self.queues.get_mut(id) {
            Some(queue) => {
                *queue = src;
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_queue_copy".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_queue_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.queues.get(id) {
            Some(queue) => Ok(queue.len().into()),
            None => Err(gml::Error::FunctionError("ds_queue_size".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_queue_empty(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.queues.get(id) {
            Some(queue) => Ok(queue.is_empty().into()),
            None => Err(gml::Error::FunctionError("ds_queue_empty".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_queue_enqueue(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val) = expect_args!(args, [int, any])?;
        match self.queues.get_mut(id) {
            Some(queue) => {
                queue.push_back(val);
                Ok(Default::default())
            },
            None => {
                Err(gml::Error::FunctionError("ds_queue_enqueue".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_queue_dequeue(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.queues.get_mut(id) {
            Some(queue) => Ok(queue.pop_front().unwrap_or_default()),
            None => {
                Err(gml::Error::FunctionError("ds_queue_dequeue".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_queue_head(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.queues.get(id) {
            Some(queue) => Ok(queue.front().map(Value::clone).unwrap_or_default()),
            None => Err(gml::Error::FunctionError("ds_queue_head".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_queue_tail(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.queues.get(id) {
            Some(queue) => Ok(queue.back().map(Value::clone).unwrap_or_default()),
            None => Err(gml::Error::FunctionError("ds_queue_tail".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_queue_write(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function ds_queue_write");
        Ok(Default::default())
    }

    pub fn ds_queue_read(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function ds_queue_read");
        Ok(Default::default())
    }

    pub fn ds_list_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.lists.put(ds::List::new()).into())
    }

    pub fn ds_list_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if self.lists.delete(id) {
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("ds_list_destroy".into(), ds::Error::NonexistentStructure(id).into()))
        }
    }

    pub fn ds_list_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.lists.get_mut(id) {
            Some(list) => {
                list.clear();
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_list_clear".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, src_id) = expect_args!(args, [int, int])?;
        let src = match self.lists.get(src_id) {
            Some(list) => list.clone(),
            None => {
                return Err(gml::Error::FunctionError(
                    "ds_list_copy".into(),
                    ds::Error::NonexistentStructure(src_id).into(),
                ))
            },
        };
        match self.lists.get_mut(id) {
            Some(list) => {
                *list = src;
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_list_copy".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.lists.get(id) {
            Some(list) => Ok(list.len().into()),
            None => Err(gml::Error::FunctionError("ds_list_size".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_empty(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.lists.get(id) {
            Some(list) => Ok(list.is_empty().into()),
            None => Err(gml::Error::FunctionError("ds_list_empty".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val) = expect_args!(args, [int, any])?;
        match self.lists.get_mut(id) {
            Some(list) => {
                list.push(val);
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_list_add".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_insert(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, index, val) = expect_args!(args, [int, int, any])?;
        match self.lists.get_mut(id) {
            Some(list) => {
                if index >= 0 && (index as usize) <= list.len() {
                    list.insert(index as usize, val);
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_list_insert".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_replace(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, index, val) = expect_args!(args, [int, int, any])?;
        match self.lists.get_mut(id) {
            Some(list) => {
                if index >= 0 && (index as usize) < list.len() {
                    list[index as usize] = val;
                }
                Ok(Default::default())
            },
            None => {
                Err(gml::Error::FunctionError("ds_list_replace".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_list_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, index) = expect_args!(args, [int, int])?;
        match self.lists.get_mut(id) {
            Some(list) => {
                if index >= 0 && (index as usize) < list.len() {
                    list.remove(index as usize);
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_list_delete".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_find_index(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val) = expect_args!(args, [int, any])?;
        match self.lists.get(id) {
            Some(list) => Ok(list
                .iter()
                .enumerate()
                .find(|(_, x)| ds::eq(x, &val, self.ds_precision))
                .map(|(i, _)| i as i32)
                .unwrap_or(-1)
                .into()),
            None => {
                Err(gml::Error::FunctionError("ds_list_find_index".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_list_find_value(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, index) = expect_args!(args, [int, int])?;
        match self.lists.get(id) {
            Some(list) => {
                if index >= 0 && (index as usize) < list.len() {
                    Ok(list[index as usize].clone())
                } else {
                    Ok(Default::default())
                }
            },
            None => {
                Err(gml::Error::FunctionError("ds_list_find_value".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_list_sort(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, asc) = expect_args!(args, [int, bool])?;
        match self.lists.get_mut(id) {
            Some(list) => {
                let precision = self.ds_precision; // otherwise we get borrowing issues
                if asc {
                    list.sort_by(|x, y| ds::cmp(x, y, precision));
                } else {
                    list.sort_by(|x, y| ds::cmp(y, x, precision));
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_list_sort".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_shuffle(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.lists.get_mut(id) {
            Some(list) => {
                for _ in 1..list.len() {
                    let id1 = self.rand.next_int(list.len() as u32 - 1);
                    let id2 = self.rand.next_int(list.len() as u32 - 1);
                    list.swap(id1 as usize, id2 as usize);
                }
                Ok(Default::default())
            },
            None => {
                Err(gml::Error::FunctionError("ds_list_shuffle".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_list_write(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.lists.get_mut(id) {
            Some(list) => {
                let mut output = "2D010000".to_string();
                output.push_str(&hex::encode_upper((list.len() as u32).to_le_bytes()));
                output.extend(list.iter().map(|v| hex::encode_upper(v.as_bytes())));
                Ok(output.into())
            },
            None => Err(gml::Error::FunctionError("ds_list_write".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_list_read(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, hex_data) = expect_args!(args, [int, string])?;
        fn read_list(mut reader: &[u8]) -> Option<ds::List> {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf).ok()?;
            if u32::from_le_bytes(buf) != 0x12d {
                return None
            }
            reader.read_exact(&mut buf).ok()?;
            let size = u32::from_le_bytes(buf) as usize;
            let mut list = ds::List::with_capacity(size);
            for _ in 0..size {
                list.push(Value::from_reader(&mut reader)?);
            }
            Some(list)
        }
        match self.lists.get_mut(id) {
            Some(old_list) => {
                match hex::decode(hex_data.as_ref()) {
                    Ok(data) => {
                        if let Some(list) = read_list(data.as_slice()) {
                            *old_list = list;
                        }
                    },
                    Err(e) => eprintln!("Warning (ds_list_read): {}", e),
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_list_read".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.maps.put(ds::Map { keys: Vec::new(), values: Vec::new() }).into())
    }

    pub fn ds_map_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if self.maps.delete(id) {
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("ds_map_destroy".into(), ds::Error::NonexistentStructure(id).into()))
        }
    }

    pub fn ds_map_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.maps.get_mut(id) {
            Some(map) => {
                map.keys.clear();
                map.values.clear();
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_map_clear".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, src_id) = expect_args!(args, [int, int])?;
        let src = match self.maps.get(src_id) {
            Some(map) => map.clone(),
            None => {
                return Err(gml::Error::FunctionError(
                    "ds_map_copy".into(),
                    ds::Error::NonexistentStructure(src_id).into(),
                ))
            },
        };
        match self.maps.get_mut(id) {
            Some(map) => {
                *map = src;
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_map_copy".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.maps.get(id) {
            Some(map) => Ok(map.keys.len().into()),
            None => Err(gml::Error::FunctionError("ds_map_size".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_empty(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.maps.get(id) {
            Some(map) => Ok(map.keys.is_empty().into()),
            None => Err(gml::Error::FunctionError("ds_map_empty".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, key, val) = expect_args!(args, [int, any, any])?;
        match self.maps.get_mut(id) {
            Some(map) => {
                let index = map.get_next_index(&key, self.ds_precision);
                map.keys.insert(index, key);
                map.values.insert(index, val);
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_map_add".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_replace(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, key, val) = expect_args!(args, [int, any, any])?;
        match self.maps.get_mut(id) {
            Some(map) => {
                if let Some(index) = map.get_index(&key, self.ds_precision) {
                    map.values[index] = val;
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_map_replace".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_delete(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, key) = expect_args!(args, [int, any])?;
        match self.maps.get_mut(id) {
            Some(map) => {
                if let Some(index) = map.get_index(&key, self.ds_precision) {
                    map.keys.remove(index);
                    map.values.remove(index);
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_map_delete".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_exists(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, key) = expect_args!(args, [int, any])?;
        match self.maps.get(id) {
            Some(map) => Ok(map.contains_key(&key, self.ds_precision).into()),
            None => Err(gml::Error::FunctionError("ds_map_exists".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_find_value(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, key) = expect_args!(args, [int, any])?;
        match self.maps.get(id) {
            Some(map) => Ok(map.get_index(&key, self.ds_precision).map_or(0.into(), |i| map.values[i].clone())),
            None => {
                Err(gml::Error::FunctionError("ds_map_find_value".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_map_find_previous(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, key) = expect_args!(args, [int, any])?;
        match self.maps.get(id) {
            Some(map) => {
                let index = map.get_index_unchecked(&key, self.ds_precision);
                if index > 0 { Ok(map.keys[index - 1].clone()) } else { Ok(Default::default()) }
            },
            None => Err(gml::Error::FunctionError(
                "ds_map_find_previous".into(),
                ds::Error::NonexistentStructure(id).into(),
            )),
        }
    }

    pub fn ds_map_find_next(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, key) = expect_args!(args, [int, any])?;
        match self.maps.get(id) {
            Some(map) => {
                let index = map.get_next_index(&key, self.ds_precision);
                if index < map.keys.len() { Ok(map.keys[index].clone()) } else { Ok(Default::default()) }
            },
            None => {
                Err(gml::Error::FunctionError("ds_map_find_next".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_map_find_first(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.maps.get(id) {
            Some(map) => Ok(map.keys.first().map(Value::clone).unwrap_or_default()),
            None => {
                Err(gml::Error::FunctionError("ds_map_find_first".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_map_find_last(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.maps.get(id) {
            Some(map) => Ok(map.keys.last().map(Value::clone).unwrap_or_default()),
            None => {
                Err(gml::Error::FunctionError("ds_map_find_last".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_map_write(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.maps.get_mut(id) {
            Some(map) => {
                let mut output = "91010000".to_string();
                output.push_str(&hex::encode_upper((map.keys.len() as u32).to_le_bytes()));
                output.extend(map.keys.iter().map(|v| hex::encode_upper(v.as_bytes())));
                output.extend(map.values.iter().map(|v| hex::encode_upper(v.as_bytes())));
                Ok(output.into())
            },
            None => Err(gml::Error::FunctionError("ds_map_write".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_map_read(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, hex_data) = expect_args!(args, [int, string])?;
        fn read_map(mut reader: &[u8]) -> Option<ds::Map> {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf).ok()?;
            if u32::from_le_bytes(buf) != 0x191 {
                return None
            }
            reader.read_exact(&mut buf).ok()?;
            let size = u32::from_le_bytes(buf) as usize;
            let mut keys = Vec::with_capacity(size);
            let mut values = Vec::with_capacity(size);
            for _ in 0..size {
                keys.push(Value::from_reader(&mut reader)?);
            }
            for _ in 0..size {
                values.push(Value::from_reader(&mut reader)?);
            }
            Some(ds::Map { keys, values })
        }
        match self.maps.get_mut(id) {
            Some(old_map) => {
                match hex::decode(hex_data.as_ref()) {
                    Ok(data) => {
                        if let Some(map) = read_map(data.as_slice()) {
                            *old_map = map;
                        }
                    },
                    Err(e) => eprintln!("Warning (ds_map_read): {}", e),
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_map_read".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_priority_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.priority_queues.put(ds::Priority { priorities: Vec::new(), values: Vec::new() }).into())
    }

    pub fn ds_priority_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if self.priority_queues.delete(id) {
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("ds_priority_destroy".into(), ds::Error::NonexistentStructure(id).into()))
        }
    }

    pub fn ds_priority_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.priority_queues.get_mut(id) {
            Some(pq) => {
                pq.priorities.clear();
                pq.values.clear();
                Ok(Default::default())
            },
            None => {
                Err(gml::Error::FunctionError("ds_priority_clear".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_priority_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, src_id) = expect_args!(args, [int, int])?;
        let src = match self.priority_queues.get(src_id) {
            Some(queue) => queue.clone(),
            None => {
                return Err(gml::Error::FunctionError(
                    "ds_priority_copy".into(),
                    ds::Error::NonexistentStructure(src_id).into(),
                ))
            },
        };
        match self.priority_queues.get_mut(id) {
            Some(queue) => {
                *queue = src;
                Ok(Default::default())
            },
            None => {
                Err(gml::Error::FunctionError("ds_priority_copy".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_priority_size(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.priority_queues.get(id) {
            Some(pq) => Ok(pq.priorities.len().into()),
            None => {
                Err(gml::Error::FunctionError("ds_priority_size".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_priority_empty(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.priority_queues.get(id) {
            Some(pq) => Ok(pq.priorities.is_empty().into()),
            None => {
                Err(gml::Error::FunctionError("ds_priority_empty".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_priority_add(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val, prio) = expect_args!(args, [int, any, any])?;
        match self.priority_queues.get_mut(id) {
            Some(pq) => {
                pq.priorities.push(prio);
                pq.values.push(val);
                Ok(Default::default())
            },
            None => {
                Err(gml::Error::FunctionError("ds_priority_add".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_priority_change_priority(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val, prio) = expect_args!(args, [int, any, any])?;
        match self.priority_queues.get_mut(id) {
            Some(pq) => {
                let precision = self.ds_precision;
                if let Some(pos) = pq.values.iter().position(|x| ds::eq(x, &val, precision)) {
                    pq.priorities[pos] = prio;
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError(
                "ds_priority_change_priority".into(),
                ds::Error::NonexistentStructure(id).into(),
            )),
        }
    }

    pub fn ds_priority_find_priority(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val) = expect_args!(args, [int, any])?;
        match self.priority_queues.get(id) {
            Some(pq) => {
                let precision = self.ds_precision;
                if let Some(pos) = pq.values.iter().position(|x| ds::eq(x, &val, precision)) {
                    Ok(pq.priorities[pos].clone())
                } else {
                    Ok(Default::default())
                }
            },
            None => Err(gml::Error::FunctionError(
                "ds_priority_find_priority".into(),
                ds::Error::NonexistentStructure(id).into(),
            )),
        }
    }

    pub fn ds_priority_delete_value(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val) = expect_args!(args, [int, any])?;
        match self.priority_queues.get_mut(id) {
            Some(pq) => {
                let precision = self.ds_precision;
                if let Some(pos) = pq.values.iter().position(|x| ds::eq(x, &val, precision)) {
                    pq.priorities.remove(pos);
                    pq.values.remove(pos);
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError(
                "ds_priority_delete_value".into(),
                ds::Error::NonexistentStructure(id).into(),
            )),
        }
    }

    pub fn ds_priority_delete_min(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.priority_queues.get_mut(id) {
            Some(pq) => {
                if let Some(min) = pq.min_id(self.ds_precision) {
                    pq.priorities.remove(min);
                    Ok(pq.values.remove(min))
                } else {
                    Ok(Default::default())
                }
            },
            None => Err(gml::Error::FunctionError(
                "ds_priority_delete_min".into(),
                ds::Error::NonexistentStructure(id).into(),
            )),
        }
    }

    pub fn ds_priority_find_min(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.priority_queues.get(id) {
            Some(pq) => {
                if let Some(min) = pq.min_id(self.ds_precision) {
                    Ok(pq.values[min].clone())
                } else {
                    Ok(Default::default())
                }
            },
            None => Err(gml::Error::FunctionError(
                "ds_priority_find_min".into(),
                ds::Error::NonexistentStructure(id).into(),
            )),
        }
    }

    pub fn ds_priority_delete_max(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.priority_queues.get_mut(id) {
            Some(pq) => {
                if let Some(max) = pq.max_id(self.ds_precision) {
                    pq.priorities.remove(max);
                    Ok(pq.values.remove(max))
                } else {
                    Ok(Default::default())
                }
            },
            None => Err(gml::Error::FunctionError(
                "ds_priority_delete_max".into(),
                ds::Error::NonexistentStructure(id).into(),
            )),
        }
    }

    pub fn ds_priority_find_max(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.priority_queues.get(id) {
            Some(pq) => {
                if let Some(max) = pq.max_id(self.ds_precision) {
                    Ok(pq.values[max].clone())
                } else {
                    Ok(Default::default())
                }
            },
            None => Err(gml::Error::FunctionError(
                "ds_priority_find_max".into(),
                ds::Error::NonexistentStructure(id).into(),
            )),
        }
    }

    pub fn ds_priority_write(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.priority_queues.get_mut(id) {
            Some(pq) => {
                let mut output = "F5010000".to_string();
                output.push_str(&hex::encode_upper((pq.priorities.len() as u32).to_le_bytes()));
                output.extend(pq.priorities.iter().map(|v| hex::encode_upper(v.as_bytes())));
                output.extend(pq.values.iter().map(|v| hex::encode_upper(v.as_bytes())));
                Ok(output.into())
            },
            None => {
                Err(gml::Error::FunctionError("ds_priority_write".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_priority_read(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, hex_data) = expect_args!(args, [int, string])?;
        fn read_priority(mut reader: &[u8]) -> Option<ds::Priority> {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf).ok()?;
            if u32::from_le_bytes(buf) != 0x1f5 {
                return None
            }
            reader.read_exact(&mut buf).ok()?;
            let size = u32::from_le_bytes(buf) as usize;
            let mut priorities = Vec::with_capacity(size);
            let mut values = Vec::with_capacity(size);
            for _ in 0..size {
                priorities.push(Value::from_reader(&mut reader)?);
            }
            for _ in 0..size {
                values.push(Value::from_reader(&mut reader)?);
            }
            Some(ds::Priority { priorities, values })
        }
        match self.priority_queues.get_mut(id) {
            Some(old_pq) => {
                match hex::decode(hex_data.as_ref()) {
                    Ok(data) => {
                        if let Some(pq) = read_priority(data.as_slice()) {
                            *old_pq = pq;
                        }
                    },
                    Err(e) => eprintln!("Warning (ds_priority_read): {}", e),
                }
                Ok(Default::default())
            },
            None => {
                Err(gml::Error::FunctionError("ds_priority_read".into(), ds::Error::NonexistentStructure(id).into()))
            },
        }
    }

    pub fn ds_grid_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (width, height) = expect_args!(args, [int, int])?;
        if width < 0 || height < 0 {
            return Err(gml::Error::FunctionError(
                "ds_grid_create".into(),
                "grids cannot have negative dimensions".to_string(),
            ))
        }
        Ok(self.grids.put(ds::Grid::new(width as usize, height as usize)).into())
    }

    pub fn ds_grid_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        if self.grids.delete(id) {
            Ok(Default::default())
        } else {
            Err(gml::Error::FunctionError("ds_grid_destroy".into(), ds::Error::NonexistentStructure(id).into()))
        }
    }

    pub fn ds_grid_copy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, src_id) = expect_args!(args, [int, int])?;
        let src_grid = match self.grids.get(src_id) {
            Some(grid) => grid.clone(),
            None => {
                return Err(gml::Error::FunctionError(
                    "ds_grid_copy".into(),
                    ds::Error::NonexistentStructure(src_id).into(),
                ))
            },
        };
        match self.grids.get_mut(id) {
            Some(grid) => {
                *grid = src_grid;
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_grid_copy".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_grid_resize(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, width, height) = expect_args!(args, [int, int, int])?;
        match self.grids.get_mut(id) {
            Some(grid) => {
                if width < 0 || height < 0 {
                    return Err(gml::Error::FunctionError(
                        "ds_grid_resize".into(),
                        "grids cannot have negative dimensions".to_string(),
                    ))
                }
                grid.resize(width as usize, height as usize);
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_grid_resize".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_grid_width(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.grids.get(id) {
            Some(grid) => Ok(grid.width().into()),
            None => Err(gml::Error::FunctionError("ds_grid_width".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_grid_height(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.grids.get(id) {
            Some(grid) => Ok(grid.height().into()),
            None => Err(gml::Error::FunctionError("ds_grid_height".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_grid_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, val) = expect_args!(args, [int, any])?;
        match self.grids.get_mut(id) {
            Some(grid) => {
                for x in 0..grid.width() {
                    for y in 0..grid.height() {
                        grid.set(x, y, val.clone());
                    }
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_grid_clear".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_grid_set(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, x, y, val) = expect_args!(args, [int, int, int, any])?;
        match self.grids.get_mut(id) {
            Some(grid) => {
                if x >= 0 && y >= 0 && (x as usize) < grid.width() && (y as usize) < grid.height() {
                    grid.set(x as usize, y as usize, val);
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_grid_set".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_grid_add(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function ds_grid_add");
        Ok(Default::default())
    }

    pub fn ds_grid_multiply(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function ds_grid_multiply");
        Ok(Default::default())
    }

    pub fn ds_grid_set_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function ds_grid_set_region");
        Ok(Default::default())
    }

    pub fn ds_grid_add_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function ds_grid_add_region");
        Ok(Default::default())
    }

    pub fn ds_grid_multiply_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function ds_grid_multiply_region");
        Ok(Default::default())
    }

    pub fn ds_grid_set_disk(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_set_disk");
        Ok(Default::default())
    }

    pub fn ds_grid_add_disk(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_add_disk");
        Ok(Default::default())
    }

    pub fn ds_grid_multiply_disk(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_multiply_disk");
        Ok(Default::default())
    }

    pub fn ds_grid_set_grid_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        println!("Called unimplemented kernel function ds_grid_set_grid_region");
        Ok(Default::default())
    }

    pub fn ds_grid_add_grid_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        println!("Called unimplemented kernel function ds_grid_add_grid_region");
        Ok(Default::default())
    }

    pub fn ds_grid_multiply_grid_region(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        println!("Called unimplemented kernel function ds_grid_multiply_grid_region");
        Ok(Default::default())
    }

    pub fn ds_grid_get(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, x, y) = expect_args!(args, [int, int, int])?;
        match self.grids.get(id) {
            Some(grid) => {
                if x >= 0 && y >= 0 && (x as usize) < grid.width() && (y as usize) < grid.height() {
                    Ok(grid.get(x as usize, y as usize).clone())
                } else {
                    Ok(Default::default())
                }
            },
            None => Err(gml::Error::FunctionError("ds_grid_get".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_grid_get_sum(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_get_sum");
        Ok(Default::default())
    }

    pub fn ds_grid_get_max(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_get_max");
        Ok(Default::default())
    }

    pub fn ds_grid_get_min(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_get_min");
        Ok(Default::default())
    }

    pub fn ds_grid_get_mean(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_get_mean");
        Ok(Default::default())
    }

    pub fn ds_grid_get_disk_sum(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function ds_grid_get_disk_sum");
        Ok(Default::default())
    }

    pub fn ds_grid_get_disk_max(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function ds_grid_get_disk_max");
        Ok(Default::default())
    }

    pub fn ds_grid_get_disk_min(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function ds_grid_get_disk_min");
        Ok(Default::default())
    }

    pub fn ds_grid_get_disk_mean(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function ds_grid_get_disk_mean");
        Ok(Default::default())
    }

    pub fn ds_grid_value_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function ds_grid_value_exists");
        Ok(Default::default())
    }

    pub fn ds_grid_value_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function ds_grid_value_x");
        Ok(Default::default())
    }

    pub fn ds_grid_value_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function ds_grid_value_y");
        Ok(Default::default())
    }

    pub fn ds_grid_value_disk_exists(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_value_disk_exists");
        Ok(Default::default())
    }

    pub fn ds_grid_value_disk_x(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_value_disk_x");
        Ok(Default::default())
    }

    pub fn ds_grid_value_disk_y(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function ds_grid_value_disk_y");
        Ok(Default::default())
    }

    pub fn ds_grid_shuffle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function ds_grid_shuffle");
        Ok(Default::default())
    }

    pub fn ds_grid_write(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let id = expect_args!(args, [int])?;
        match self.grids.get_mut(id) {
            Some(grid) => {
                let mut output = "59020000".to_string();
                output.push_str(&hex::encode_upper((grid.width() as u32).to_le_bytes()));
                output.push_str(&hex::encode_upper((grid.height() as u32).to_le_bytes()));
                for x in 0..grid.width() {
                    for y in 0..grid.height() {
                        output.push_str(&hex::encode_upper(grid.get(x, y).as_bytes()));
                    }
                }
                Ok(output.into())
            },
            None => Err(gml::Error::FunctionError("ds_grid_write".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn ds_grid_read(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, hex_data) = expect_args!(args, [int, string])?;
        fn read_grid(mut reader: &[u8]) -> Option<ds::Grid> {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf).ok()?;
            if u32::from_le_bytes(buf) != 0x259 {
                return None
            }
            reader.read_exact(&mut buf).ok()?;
            let width = u32::from_le_bytes(buf) as usize;
            reader.read_exact(&mut buf).ok()?;
            let height = u32::from_le_bytes(buf) as usize;
            let mut grid = ds::Grid::new(width, height);
            for x in 0..width {
                for y in 0..height {
                    grid.set(x, y, Value::from_reader(&mut reader)?);
                }
            }
            Some(grid)
        }
        match self.grids.get_mut(id) {
            Some(old_grid) => {
                match hex::decode(hex_data.as_ref()) {
                    Ok(data) => {
                        if let Some(grid) = read_grid(data.as_slice()) {
                            *old_grid = grid;
                        }
                    },
                    Err(e) => eprintln!("Warning (ds_grid_read): {}", e),
                }
                Ok(Default::default())
            },
            None => Err(gml::Error::FunctionError("ds_grid_read".into(), ds::Error::NonexistentStructure(id).into())),
        }
    }

    pub fn sound_play(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sound_id = expect_args!(args, [int])?;
        if let Some(sound) = self.assets.sounds.get_asset(sound_id) {
            let kind = sound.kind;
            if let Some(source) = &sound.source {
                let cloned = source.clone();
                let reader = BufReader::new(std::io::Cursor::new(cloned));
                match rodio::Decoder::new(reader) {
                    Ok(source) => self.play_sound(source, sound_id, kind == asset::sound::Kind::Multimedia),
                    _ => {
                        return Err(gml::Error::FunctionError(
                            "sound_play".into(),
                            format!("Decoder error for sound {}. Data may be malformed.", sound_id),
                        ))
                    },
                }
            }
        }
        Ok(Default::default())
    }

    pub fn sound_loop(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sound_id = expect_args!(args, [int])?;
        if let Some(sound) = self.assets.sounds.get_asset(sound_id) {
            let kind = sound.kind;
            if let Some(source) = &sound.source {
                use rodio::Source;
                let cloned = source.clone();
                let reader = BufReader::new(std::io::Cursor::new(cloned));
                match rodio::Decoder::new(reader) {
                    Ok(source) => {
                        self.play_sound(source.repeat_infinite(), sound_id, kind == asset::sound::Kind::Multimedia)
                    },
                    _ => {
                        return Err(gml::Error::FunctionError(
                            "sound_loop".into(),
                            format!("Decoder error for sound {}. Data may be malformed.", sound_id),
                        ))
                    },
                }
            }
        }
        Ok(Default::default())
    }

    pub fn sound_stop(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sound_id = expect_args!(args, [int])?;
        self.rodio_sinks.retain(|(_, id, _)| *id != sound_id);
        Ok(Default::default())
    }

    pub fn sound_stop_all(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        self.rodio_sinks.clear();
        Ok(Default::default())
    }

    pub fn sound_isplaying(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let sound_id = expect_args!(args, [int])?;
        Ok(self.rodio_sinks.iter().any(|(sink, id, _)| *id == sound_id && !sink.empty()).into())
    }

    pub fn sound_volume(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        //unimplemented!("Called unimplemented kernel function sound_volume")
        // TODO
        Ok(Default::default())
    }

    pub fn sound_fade(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        //unimplemented!("Called unimplemented kernel function sound_fade")
        Ok(Default::default())
    }

    pub fn sound_pan(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function sound_pan");
        Ok(Default::default())
    }

    pub fn sound_background_tempo(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function sound_background_tempo")
        Ok(Default::default())
    }

    pub fn sound_global_volume(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        //unimplemented!("Called unimplemented kernel function sound_global_volume")
        // TODO
        Ok(Default::default())
    }

    pub fn sound_set_search_directory(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function sound_set_search_directory");
        Ok(Default::default())
    }

    pub fn sound_effect_set(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function sound_effect_set");
        Ok(Default::default())
    }

    pub fn sound_effect_chorus(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        println!("Called unimplemented kernel function sound_effect_chorus");
        Ok(Default::default())
    }

    pub fn sound_effect_compressor(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        println!("Called unimplemented kernel function sound_effect_compressor");
        Ok(Default::default())
    }

    pub fn sound_effect_echo(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 6
        println!("Called unimplemented kernel function sound_effect_echo");
        Ok(Default::default())
    }

    pub fn sound_effect_flanger(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 8
        println!("Called unimplemented kernel function sound_effect_flanger");
        Ok(Default::default())
    }

    pub fn sound_effect_gargle(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function sound_effect_gargle");
        Ok(Default::default())
    }

    pub fn sound_effect_equalizer(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function sound_effect_equalizer");
        Ok(Default::default())
    }

    pub fn sound_effect_reverb(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 5
        println!("Called unimplemented kernel function sound_effect_reverb");
        Ok(Default::default())
    }

    pub fn sound_3d_set_sound_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function sound_3d_set_sound_position");
        Ok(Default::default())
    }

    pub fn sound_3d_set_sound_velocity(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 4
        println!("Called unimplemented kernel function sound_3d_set_sound_velocity");
        Ok(Default::default())
    }

    pub fn sound_3d_set_sound_distance(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 3
        println!("Called unimplemented kernel function sound_3d_set_sound_distance");
        Ok(Default::default())
    }

    pub fn sound_3d_set_sound_cone(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 7
        println!("Called unimplemented kernel function sound_3d_set_sound_cone");
        Ok(Default::default())
    }

    pub fn cd_init(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_init");
        Ok(Default::default())
    }

    pub fn cd_present(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_present");
        Ok(Default::default())
    }

    pub fn cd_number(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_number");
        Ok(Default::default())
    }

    pub fn cd_playing(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_playing");
        Ok(Default::default())
    }

    pub fn cd_paused(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_paused");
        Ok(Default::default())
    }

    pub fn cd_track(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_track");
        Ok(Default::default())
    }

    pub fn cd_length(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_length");
        Ok(Default::default())
    }

    pub fn cd_track_length(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function cd_track_length");
        Ok(Default::default())
    }

    pub fn cd_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_position");
        Ok(Default::default())
    }

    pub fn cd_track_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_track_position");
        Ok(Default::default())
    }

    pub fn cd_play(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 2
        println!("Called unimplemented kernel function cd_play");
        Ok(Default::default())
    }

    pub fn cd_stop(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_stop");
        Ok(Default::default())
    }

    pub fn cd_pause(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_pause");
        Ok(Default::default())
    }

    pub fn cd_resume(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_resume");
        Ok(Default::default())
    }

    pub fn cd_set_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function cd_set_position");
        Ok(Default::default())
    }

    pub fn cd_set_track_position(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function cd_set_track_position");
        Ok(Default::default())
    }

    pub fn cd_open_door(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_open_door");
        Ok(Default::default())
    }

    pub fn cd_close_door(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 0
        println!("Called unimplemented kernel function cd_close_door");
        Ok(Default::default())
    }

    pub fn mci_command(&mut self, _context: &mut Context, _args: &[Value]) -> gml::Result<Value> {
        // Expected arg count: 1
        println!("Called unimplemented kernel function MCI_command");
        Ok(Default::default())
    }

    pub fn d3d_start(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.renderer.set_3d(true);
        Ok(1.into())
    }

    pub fn d3d_end(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.renderer.set_3d(false);
        Ok(1.into())
    }

    pub fn d3d_set_perspective(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let perspective = expect_args!(args, [bool])?;
        self.renderer.set_perspective(perspective);
        Ok(Default::default())
    }

    pub fn d3d_set_hidden(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let hidden = expect_args!(args, [bool])?;
        self.renderer.set_depth_test(hidden);
        Ok(Default::default())
    }

    pub fn d3d_set_depth(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let depth = expect_args!(args, [real])?;
        self.renderer.set_depth(depth.into_inner() as f32);
        Ok(Default::default())
    }

    pub fn d3d_set_zwriteenable(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let write_depth = expect_args!(args, [bool])?;
        if self.renderer.get_3d() {
            self.renderer.set_write_depth(write_depth);
        }
        Ok(Default::default())
    }

    pub fn d3d_set_lighting(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let enabled = expect_args!(args, [bool])?;
        self.renderer.set_lighting_enabled(enabled);
        Ok(Default::default())
    }

    pub fn d3d_set_shading(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let gouraud = expect_args!(args, [bool])?;
        self.renderer.set_gouraud(gouraud);
        Ok(Default::default())
    }

    pub fn d3d_set_fog(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (enabled, colour, begin, end) = expect_args!(args, [bool, int, real, real])?;
        let fog = if enabled {
            Some(Fog { colour, begin: begin.into_inner() as f32, end: end.into_inner() as f32 })
        } else {
            None
        };
        self.renderer.set_fog(fog);
        Ok(Default::default())
    }

    pub fn d3d_set_culling(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let cull = expect_args!(args, [bool])?;
        self.renderer.set_culling(cull);
        Ok(Default::default())
    }

    pub fn d3d_primitive_begin(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let kind = expect_args!(args, [int])?;
        self.renderer.reset_primitive_3d(kind.into(), None);
        Ok(Default::default())
    }

    pub fn d3d_primitive_begin_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (kind, texture) = expect_args!(args, [int, int])?;
        self.renderer.reset_primitive_3d(kind.into(), self.renderer.get_texture_from_id(texture as _).copied());
        Ok(Default::default())
    }

    pub fn d3d_primitive_end(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.renderer.draw_primitive_3d();
        Ok(Default::default())
    }

    pub fn d3d_vertex(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, z) = expect_args!(args, [real, real, real])?;
        // And here we see the really weird GM8 colour rules where when drawing 3D vertices,
        // the LSB of the blue component is set to 0 when the colour isn't specified, and 1 when it is.
        let col = u32::from(self.draw_colour) as i32 & 0xfeffff;
        self.renderer.vertex_3d(x.into(), y.into(), z.into(), 0.0, 0.0, 0.0, 0.0, 0.0, col, self.draw_alpha.into());
        Ok(Default::default())
    }

    pub fn d3d_vertex_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, z, col, alpha) = expect_args!(args, [real, real, real, int, real])?;
        let col = col | 0x010000;
        self.renderer.vertex_3d(x.into(), y.into(), z.into(), 0.0, 0.0, 0.0, 0.0, 0.0, col, alpha.into());
        Ok(Default::default())
    }

    pub fn d3d_vertex_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, z, xtex, ytex) = expect_args!(args, [real, real, real, real, real])?;
        let col = u32::from(self.draw_colour) as i32 & 0xfeffff;
        self.renderer.vertex_3d(
            x.into(),
            y.into(),
            z.into(),
            0.0,
            0.0,
            0.0,
            xtex.into(),
            ytex.into(),
            col,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_vertex_texture_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, z, xtex, ytex, col, alpha) = expect_args!(args, [real, real, real, real, real, int, real])?;
        let col = col | 0x010000;
        self.renderer.vertex_3d(
            x.into(),
            y.into(),
            z.into(),
            0.0,
            0.0,
            0.0,
            xtex.into(),
            ytex.into(),
            col,
            alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_vertex_normal(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, z, nx, ny, nz) = expect_args!(args, [real, real, real, real, real, real])?;
        let col = u32::from(self.draw_colour) as i32 & 0xfeffff;
        self.renderer.vertex_3d(
            x.into(),
            y.into(),
            z.into(),
            nx.into(),
            ny.into(),
            nz.into(),
            0.0,
            0.0,
            col,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_vertex_normal_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, z, nx, ny, nz, col, alpha) = expect_args!(args, [real, real, real, real, real, real, int, real])?;
        let col = col | 0x010000;
        self.renderer.vertex_3d(
            x.into(),
            y.into(),
            z.into(),
            nx.into(),
            ny.into(),
            nz.into(),
            0.0,
            0.0,
            col,
            alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_vertex_normal_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, z, nx, ny, nz, xtex, ytex) = expect_args!(args, [real, real, real, real, real, real, real, real])?;
        let col = u32::from(self.draw_colour) as i32 & 0xfeffff;
        self.renderer.vertex_3d(
            x.into(),
            y.into(),
            z.into(),
            nx.into(),
            ny.into(),
            nz.into(),
            xtex.into(),
            ytex.into(),
            col,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_vertex_normal_texture_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, z, nx, ny, nz, xtex, ytex, col, alpha) =
            expect_args!(args, [real, real, real, real, real, real, real, real, int, real])?;
        let col = col | 0x010000;
        self.renderer.vertex_3d(
            x.into(),
            y.into(),
            z.into(),
            nx.into(),
            ny.into(),
            nz.into(),
            xtex.into(),
            ytex.into(),
            col,
            alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_draw_block(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2, tex_id, hrepeat, vrepeat) =
            expect_args!(args, [real, real, real, real, real, real, int, real, real])?;
        let atlas_ref = self.renderer.get_texture_from_id(tex_id as _).copied();
        model::draw_block(
            &mut self.renderer,
            atlas_ref,
            &mut |r: &mut Renderer| r.draw_primitive_3d(),
            x1.into(),
            y1.into(),
            z1.into(),
            x2.into(),
            y2.into(),
            z2.into(),
            hrepeat.into(),
            vrepeat.into(),
            u32::from(self.draw_colour) as i32 & 0xfeffff,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_draw_cylinder(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2, tex_id, hrepeat, vrepeat, closed, steps) =
            expect_args!(args, [real, real, real, real, real, real, int, real, real, bool, int])?;
        let atlas_ref = self.renderer.get_texture_from_id(tex_id as _).copied();
        model::draw_cylinder(
            &mut self.renderer,
            atlas_ref,
            &mut |r: &mut Renderer| r.draw_primitive_3d(),
            x1.into_inner(),
            y1.into_inner(),
            z1.into_inner(),
            x2.into_inner(),
            y2.into_inner(),
            z2.into_inner(),
            hrepeat.into_inner(),
            vrepeat.into_inner(),
            closed,
            steps,
            u32::from(self.draw_colour) as i32 & 0xfeffff,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_draw_cone(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2, tex_id, hrepeat, vrepeat, closed, steps) =
            expect_args!(args, [real, real, real, real, real, real, int, real, real, bool, int])?;
        let atlas_ref = self.renderer.get_texture_from_id(tex_id as _).copied();
        model::draw_cone(
            &mut self.renderer,
            atlas_ref,
            &mut |r: &mut Renderer| r.draw_primitive_3d(),
            x1.into_inner(),
            y1.into_inner(),
            z1.into_inner(),
            x2.into_inner(),
            y2.into_inner(),
            z2.into_inner(),
            hrepeat.into_inner(),
            vrepeat.into_inner(),
            closed,
            steps,
            u32::from(self.draw_colour) as i32 & 0xfeffff,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_draw_ellipsoid(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2, tex_id, hrepeat, vrepeat, steps) =
            expect_args!(args, [real, real, real, real, real, real, int, real, real, int])?;
        let atlas_ref = self.renderer.get_texture_from_id(tex_id as _).copied();
        model::draw_ellipsoid(
            &mut self.renderer,
            atlas_ref,
            &mut |r: &mut Renderer| r.draw_primitive_3d(),
            x1.into_inner(),
            y1.into_inner(),
            z1.into_inner(),
            x2.into_inner(),
            y2.into_inner(),
            z2.into_inner(),
            hrepeat.into_inner(),
            vrepeat.into_inner(),
            steps,
            u32::from(self.draw_colour) as i32 & 0xfeffff,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_draw_wall(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2, tex_id, hrepeat, vrepeat) =
            expect_args!(args, [real, real, real, real, real, real, int, real, real])?;
        let atlas_ref = self.renderer.get_texture_from_id(tex_id as _).copied();
        model::draw_wall(
            &mut self.renderer,
            atlas_ref,
            &mut |r: &mut Renderer| r.draw_primitive_3d(),
            x1.into(),
            y1.into(),
            z1.into(),
            x2.into(),
            y2.into(),
            z2.into(),
            hrepeat.into(),
            vrepeat.into(),
            u32::from(self.draw_colour) as i32 & 0xfeffff,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_draw_floor(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x1, y1, z1, x2, y2, z2, tex_id, hrepeat, vrepeat) =
            expect_args!(args, [real, real, real, real, real, real, int, real, real])?;
        let atlas_ref = self.renderer.get_texture_from_id(tex_id as _).copied();
        model::draw_floor(
            &mut self.renderer,
            atlas_ref,
            &mut |r: &mut Renderer| r.draw_primitive_3d(),
            x1.into(),
            y1.into(),
            z1.into(),
            x2.into(),
            y2.into(),
            z2.into(),
            hrepeat.into(),
            vrepeat.into(),
            u32::from(self.draw_colour) as i32 & 0xfeffff,
            self.draw_alpha.into(),
        );
        Ok(Default::default())
    }

    pub fn d3d_set_projection(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (eye_x, eye_y, eye_z, at_x, at_y, at_z, up_x, up_y, up_z) =
            expect_args!(args, [real, real, real, real, real, real, real, real, real])?;

        // zaxis = normal(at - eye)
        let (za_x, za_y, za_z) = (at_x - eye_x, at_y - eye_y, at_z - eye_z);
        let za_len = (za_x * za_x + za_y * za_y + za_z * za_z).sqrt();
        let (za_x, za_y, za_z) = (za_x / za_len, za_y / za_len, za_z / za_len);
        // xaxis = normal(cross(up, zaxis))
        let (xa_x, xa_y, xa_z) = (up_y * za_z - up_z * za_y, up_z * za_x - up_x * za_z, up_x * za_y - up_y * za_x);
        let xa_len = (xa_x * xa_x + xa_y * xa_y + xa_z * xa_z).sqrt();
        let (xa_x, xa_y, xa_z) = (xa_x / xa_len, xa_y / xa_len, xa_z / xa_len);
        // yaxis = cross(zaxis, xaxis)
        let (ya_x, ya_y, ya_z) = (za_y * xa_z - za_z * xa_y, za_z * xa_x - za_x * xa_z, za_x * xa_y - za_y * xa_x);
        // bottom row
        let (xa_w, ya_w, za_w) = (
            -(xa_x * eye_x + xa_y * eye_y + xa_z * eye_z),
            -(ya_x * eye_x + ya_y * eye_y + ya_z * eye_z),
            -(za_x * eye_x + za_y * eye_y + za_z * eye_z),
        );

        #[rustfmt::skip]
        let view_matrix: [f32; 16] = [
            xa_x.into_inner() as f32, ya_x.into_inner() as f32, za_x.into_inner() as f32, 0.0,
            xa_y.into_inner() as f32, ya_y.into_inner() as f32, za_y.into_inner() as f32, 0.0,
            xa_z.into_inner() as f32, ya_z.into_inner() as f32, za_z.into_inner() as f32, 0.0,
            xa_w.into_inner() as f32, ya_w.into_inner() as f32, za_w.into_inner() as f32, 1.0,
        ];

        self.renderer.set_view_matrix(view_matrix);
        Ok(Default::default())
    }

    pub fn d3d_set_projection_ext(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (eye_x, eye_y, eye_z, at_x, at_y, at_z, up_x, up_y, up_z, angle, aspect, znear, zfar) =
            expect_args!(args, [real, real, real, real, real, real, real, real, real, real, real, real, real])?;

        // zaxis = normal(at - eye)
        let (za_x, za_y, za_z) = (at_x - eye_x, at_y - eye_y, at_z - eye_z);
        let za_len = (za_x * za_x + za_y * za_y + za_z * za_z).sqrt();
        let (za_x, za_y, za_z) = (za_x / za_len, za_y / za_len, za_z / za_len);
        // xaxis = normal(cross(up, zaxis))
        let (xa_x, xa_y, xa_z) = (up_y * za_z - up_z * za_y, up_z * za_x - up_x * za_z, up_x * za_y - up_y * za_x);
        let xa_len = (xa_x * xa_x + xa_y * xa_y + xa_z * xa_z).sqrt();
        let (xa_x, xa_y, xa_z) = (xa_x / xa_len, xa_y / xa_len, xa_z / xa_len);
        // yaxis = cross(zaxis, xaxis)
        let (ya_x, ya_y, ya_z) = (za_y * xa_z - za_z * xa_y, za_z * xa_x - za_x * xa_z, za_x * xa_y - za_y * xa_x);
        // bottom row
        let (xa_w, ya_w, za_w) = (
            -(xa_x * eye_x + xa_y * eye_y + xa_z * eye_z),
            -(ya_x * eye_x + ya_y * eye_y + ya_z * eye_z),
            -(za_x * eye_x + za_y * eye_y + za_z * eye_z),
        );

        #[rustfmt::skip]
        let view_matrix: [f32; 16] = [
            xa_x.into_inner() as f32, ya_x.into_inner() as f32, za_x.into_inner() as f32, 0.0,
            xa_y.into_inner() as f32, ya_y.into_inner() as f32, za_y.into_inner() as f32, 0.0,
            xa_z.into_inner() as f32, ya_z.into_inner() as f32, za_z.into_inner() as f32, 0.0,
            xa_w.into_inner() as f32, ya_w.into_inner() as f32, za_w.into_inner() as f32, 1.0,
        ];

        let half_angle = angle.to_radians() / 2.into();
        let yscale = half_angle.cos() / half_angle.sin();
        let xscale = (yscale / aspect).into_inner() as f32;
        let yscale = yscale.into_inner() as f32;
        let upper_z = (zfar / (zfar - znear)).into_inner() as f32;
        let lower_z = (-znear * zfar / (zfar - znear)).into_inner() as f32;
        #[rustfmt::skip]
        let proj_matrix: [f32; 16] = [
            xscale, 0.0,    0.0,     0.0,
            0.0,    yscale, 0.0,     0.0,
            0.0,    0.0,    upper_z, 1.0,
            0.0,    0.0,    lower_z, 0.0,
        ];

        self.renderer.set_viewproj_matrix(view_matrix, proj_matrix);
        Ok(Default::default())
    }

    pub fn d3d_set_projection_ortho(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, w, h, angle) = expect_args!(args, [real, real, real, real, real])?;
        self.renderer.set_projection_ortho(x.into(), y.into(), w.into(), h.into(), angle.into());
        Ok(Default::default())
    }

    pub fn d3d_set_projection_perspective(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (x, y, w, h, angle) = expect_args!(args, [real, real, real, real, real])?;
        self.renderer.set_projection_perspective(x.into(), y.into(), w.into(), h.into(), angle.into());
        Ok(Default::default())
    }

    pub fn d3d_transform_set_identity(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        self.renderer.set_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_set_translation(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (xt, yt, zt) = expect_args!(args, [real, real, real])?;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            1.0,                    0.0,                    0.0,                    0.0,
            0.0,                    1.0,                    0.0,                    0.0,
            0.0,                    0.0,                    1.0,                    0.0,
            xt.into_inner() as f32, yt.into_inner() as f32, zt.into_inner() as f32, 1.0,
        ];
        self.renderer.set_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_set_scaling(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (xs, ys, zs) = expect_args!(args, [real, real, real])?;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            xs.into_inner() as f32, 0.0,                    0.0,                    0.0,
            0.0,                    ys.into_inner() as f32, 0.0,                    0.0,
            0.0,                    0.0,                    zs.into_inner() as f32, 0.0,
            0.0,                    0.0,                    0.0,                    1.0,
        ];
        self.renderer.set_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_set_rotation_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let angle = expect_args!(args, [real])?.to_radians();
        let sin = -angle.sin().into_inner() as f32;
        let cos = angle.cos().into_inner() as f32;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            1.0, 0.0,  0.0, 0.0,
            0.0, cos,  sin, 0.0,
            0.0, -sin, cos, 0.0,
            0.0, 0.0,  0.0, 1.0,
        ];
        self.renderer.set_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_set_rotation_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let angle = expect_args!(args, [real])?.to_radians();
        let sin = -angle.sin().into_inner() as f32;
        let cos = angle.cos().into_inner() as f32;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0,  0.0,
            sin, 0.0, cos,  0.0,
            0.0, 0.0, 0.0,  1.0,
        ];
        self.renderer.set_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_set_rotation_z(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let angle = expect_args!(args, [real])?.to_radians();
        let sin = -angle.sin().into_inner() as f32;
        let cos = angle.cos().into_inner() as f32;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            cos,  sin, 0.0, 0.0,
            -sin, cos, 0.0, 0.0,
            0.0,  0.0, 1.0, 0.0,
            0.0,  0.0, 0.0, 1.0,
        ];
        self.renderer.set_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_set_rotation_axis(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (xa, ya, za, angle) = expect_args!(args, [real, real, real, real])?;
        let axis_len = (xa * xa + ya * ya + za * za).sqrt();
        let (xa, ya, za) = (xa / axis_len, ya / axis_len, za / axis_len);
        let angle = angle.to_radians();
        let sin = -angle.sin();
        let cos = angle.cos();
        let anticos = Real::from(1.0) - cos;
        let m00 = (cos + xa * xa * anticos).into_inner() as f32;
        let m01 = (xa * ya * anticos - za * sin).into_inner() as f32;
        let m02 = (xa * za * anticos + ya * sin).into_inner() as f32;
        let m10 = (ya * xa * anticos + za * sin).into_inner() as f32;
        let m11 = (cos + ya * ya * anticos).into_inner() as f32;
        let m12 = (ya * za * anticos - xa * sin).into_inner() as f32;
        let m20 = (za * za * anticos - ya * sin).into_inner() as f32;
        let m21 = (za * ya * anticos + xa * sin).into_inner() as f32;
        let m22 = (cos + za * za * anticos).into_inner() as f32;
        #[rustfmt::skip]
        let model_matrix = [
            m00, m10, m20, 0.0,
            m01, m11, m21, 0.0,
            m02, m12, m22, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        self.renderer.set_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_add_translation(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (xt, yt, zt) = expect_args!(args, [real, real, real])?;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            1.0,                    0.0,                    0.0,                    0.0,
            0.0,                    1.0,                    0.0,                    0.0,
            0.0,                    0.0,                    1.0,                    0.0,
            xt.into_inner() as f32, yt.into_inner() as f32, zt.into_inner() as f32, 1.0,
        ];
        self.renderer.mult_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_add_scaling(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (xs, ys, zs) = expect_args!(args, [real, real, real])?;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            xs.into_inner() as f32, 0.0,                    0.0,                    0.0,
            0.0,                    ys.into_inner() as f32, 0.0,                    0.0,
            0.0,                    0.0,                    zs.into_inner() as f32, 0.0,
            0.0,                    0.0,                    0.0,                    1.0,
        ];
        self.renderer.mult_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_add_rotation_x(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let angle = expect_args!(args, [real])?.to_radians();
        let sin = -angle.sin().into_inner() as f32;
        let cos = angle.cos().into_inner() as f32;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            1.0, 0.0,  0.0, 0.0,
            0.0, cos,  sin, 0.0,
            0.0, -sin, cos, 0.0,
            0.0, 0.0,  0.0, 1.0,
        ];
        self.renderer.mult_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_add_rotation_y(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let angle = expect_args!(args, [real])?.to_radians();
        let sin = -angle.sin().into_inner() as f32;
        let cos = angle.cos().into_inner() as f32;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0,  0.0,
            sin, 0.0, cos,  0.0,
            0.0, 0.0, 0.0,  1.0,
        ];
        self.renderer.mult_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_add_rotation_z(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let angle = expect_args!(args, [real])?.to_radians();
        let sin = -angle.sin().into_inner() as f32;
        let cos = angle.cos().into_inner() as f32;
        #[rustfmt::skip]
        let model_matrix: [f32; 16] = [
            cos,  sin, 0.0, 0.0,
            -sin, cos, 0.0, 0.0,
            0.0,  0.0, 1.0, 0.0,
            0.0,  0.0, 0.0, 1.0,
        ];
        self.renderer.mult_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_add_rotation_axis(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (xa, ya, za, angle) = expect_args!(args, [real, real, real, real])?;
        let axis_len = (xa * xa + ya * ya + za * za).sqrt();
        let (xa, ya, za) = (xa / axis_len, ya / axis_len, za / axis_len);
        let angle = angle.to_radians();
        let sin = -angle.sin();
        let cos = angle.cos();
        let anticos = Real::from(1.0) - cos;
        let m00 = (cos + xa * xa * anticos).into_inner() as f32;
        let m01 = (xa * ya * anticos - za * sin).into_inner() as f32;
        let m02 = (xa * za * anticos + ya * sin).into_inner() as f32;
        let m10 = (ya * xa * anticos + za * sin).into_inner() as f32;
        let m11 = (cos + ya * ya * anticos).into_inner() as f32;
        let m12 = (ya * za * anticos - xa * sin).into_inner() as f32;
        let m20 = (za * za * anticos - ya * sin).into_inner() as f32;
        let m21 = (za * ya * anticos + xa * sin).into_inner() as f32;
        let m22 = (cos + za * za * anticos).into_inner() as f32;
        #[rustfmt::skip]
        let model_matrix = [
            m00, m10, m20, 0.0,
            m01, m11, m21, 0.0,
            m02, m12, m22, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        self.renderer.mult_model_matrix(model_matrix);
        Ok(Default::default())
    }

    pub fn d3d_transform_stack_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        self.model_matrix_stack.clear();
        Ok(Default::default())
    }

    pub fn d3d_transform_stack_empty(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.model_matrix_stack.is_empty().into())
    }

    pub fn d3d_transform_stack_push(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        if self.model_matrix_stack.len() <= 1000 {
            self.model_matrix_stack.push(self.renderer.get_model_matrix());
            Ok(true.into())
        } else {
            Ok(false.into())
        }
    }

    pub fn d3d_transform_stack_pop(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        if let Some(mat) = self.model_matrix_stack.pop() {
            self.renderer.set_model_matrix(mat);
            Ok(true.into())
        } else {
            Ok(false.into())
        }
    }

    pub fn d3d_transform_stack_top(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        if let Some(mat) = self.model_matrix_stack.last() {
            self.renderer.set_model_matrix(*mat);
            Ok(true.into())
        } else {
            Ok(false.into())
        }
    }

    pub fn d3d_transform_stack_discard(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        Ok(self.model_matrix_stack.pop().is_some().into())
    }

    pub fn d3d_light_define_ambient(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let colour = expect_args!(args, [int])?;
        self.renderer.set_ambient_colour(colour);
        Ok(Default::default())
    }

    pub fn d3d_light_define_direction(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, dx, dy, dz, colour) = expect_args!(args, [int, real, real, real, int])?;
        if (0..8).contains(&id) {
            self.renderer.set_light(id as usize, Light::Directional {
                direction: [dx.into_inner() as f32, dy.into_inner() as f32, dz.into_inner() as f32],
                colour,
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_light_define_point(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, x, y, z, range, colour) = expect_args!(args, [int, real, real, real, real, int])?;
        if (0..8).contains(&id) {
            self.renderer.set_light(id as usize, Light::Point {
                position: [x.into_inner() as f32, y.into_inner() as f32, z.into_inner() as f32],
                range: range.into_inner() as f32,
                colour,
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_light_enable(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (id, enabled) = expect_args!(args, [int, bool])?;
        if (0..8).contains(&id) {
            self.renderer.set_light_enabled(id as usize, enabled);
        }
        Ok(Default::default())
    }

    pub fn d3d_model_create(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        expect_args!(args, [])?;
        let model = Default::default();
        if let Some(id) = self.models.iter().position(|x| x.is_none()) {
            self.models[id] = Some(model);
            Ok(id.into())
        } else {
            self.models.push(Some(model));
            Ok((self.models.len() - 1).into())
        }
    }

    pub fn d3d_model_destroy(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let model_id = expect_args!(args, [int])?;
        if self.models.get_asset(model_id).is_some() {
            self.models[model_id as usize] = None;
        }
        Ok(Default::default())
    }

    pub fn d3d_model_clear(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let model_id = expect_args!(args, [int])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            *model = Default::default();
        }
        Ok(Default::default())
    }

    pub fn d3d_model_load(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, fname) = expect_args!(args, [int, string])?;
        fn load_model(fname: &str) -> Result<model::Model, Box<dyn std::error::Error>> {
            let mut file = std::io::BufReader::new(std::fs::File::open(fname)?);
            let version = file::read_real(&mut file)?;
            if version != 100.0 {
                return Err("invalid version".into())
            };
            file::skip_line(&mut file)?;
            let command_count = match file::read_real(&mut file)? as i32 {
                x if x < 0 => return Err("negative command count".into()),
                x => x as usize,
            };
            file::skip_line(&mut file)?;
            let mut commands = Vec::with_capacity(command_count);
            for _ in 0..command_count {
                let cmd = file::read_real(&mut file)?.round() as i32;
                let mut args = [0.0; 10];
                for x in &mut args {
                    *x = file::read_real(&mut file)?;
                }
                file::skip_line(&mut file)?;
                commands.push(match cmd {
                    0 => model::Command::Begin((args[0] as i32).into()),
                    1 => model::Command::End,
                    2 => model::Command::Vertex {
                        pos: [args[0].into(), args[1].into(), args[2].into()],
                        normal: [0.into(); 3],
                        tex_coord: [0.into(); 2],
                    },
                    3 => model::Command::VertexColour {
                        pos: [args[0].into(), args[1].into(), args[2].into()],
                        normal: [0.into(); 3],
                        tex_coord: [0.into(); 2],
                        col: (args[3] as i32, args[4].into()),
                    },
                    4 => model::Command::Vertex {
                        pos: [args[0].into(), args[1].into(), args[2].into()],
                        normal: [0.into(); 3],
                        tex_coord: [args[3].into(), args[4].into()],
                    },
                    5 => model::Command::VertexColour {
                        pos: [args[0].into(), args[1].into(), args[2].into()],
                        normal: [0.into(); 3],
                        tex_coord: [args[3].into(), args[4].into()],
                        col: (args[5] as i32, args[6].into()),
                    },
                    6 => model::Command::Vertex {
                        pos: [args[0].into(), args[1].into(), args[2].into()],
                        normal: [args[3].into(), args[4].into(), args[5].into()],
                        tex_coord: [0.into(); 2],
                    },
                    7 => model::Command::VertexColour {
                        pos: [args[0].into(), args[1].into(), args[2].into()],
                        normal: [args[3].into(), args[4].into(), args[5].into()],
                        tex_coord: [0.into(); 2],
                        col: (args[6] as i32, args[7].into()),
                    },
                    8 => model::Command::Vertex {
                        pos: [args[0].into(), args[1].into(), args[2].into()],
                        normal: [args[3].into(), args[4].into(), args[5].into()],
                        tex_coord: [args[6].into(), args[7].into()],
                    },
                    9 => model::Command::VertexColour {
                        pos: [args[0].into(), args[1].into(), args[2].into()],
                        normal: [args[3].into(), args[4].into(), args[5].into()],
                        tex_coord: [args[6].into(), args[7].into()],
                        col: (args[8] as i32, args[9].into()),
                    },
                    10 => model::Command::Block {
                        pos1: [args[0].into(), args[1].into(), args[2].into()],
                        pos2: [args[3].into(), args[4].into(), args[5].into()],
                        tex_repeat: [args[6].into(), args[7].into()],
                    },
                    11 => model::Command::Cylinder {
                        pos1: [args[0].into(), args[1].into(), args[2].into()],
                        pos2: [args[3].into(), args[4].into(), args[5].into()],
                        tex_repeat: [args[6].into(), args[7].into()],
                        closed: args[8] >= 0.5,
                        steps: args[9] as _,
                    },
                    12 => model::Command::Cone {
                        pos1: [args[0].into(), args[1].into(), args[2].into()],
                        pos2: [args[3].into(), args[4].into(), args[5].into()],
                        tex_repeat: [args[6].into(), args[7].into()],
                        closed: args[8] >= 0.5,
                        steps: args[9] as _,
                    },
                    13 => model::Command::Ellipsoid {
                        pos1: [args[0].into(), args[1].into(), args[2].into()],
                        pos2: [args[3].into(), args[4].into(), args[5].into()],
                        tex_repeat: [args[6].into(), args[7].into()],
                        steps: args[8] as _,
                    },
                    14 => model::Command::Wall {
                        pos1: [args[0].into(), args[1].into(), args[2].into()],
                        pos2: [args[3].into(), args[4].into(), args[5].into()],
                        tex_repeat: [args[6].into(), args[7].into()],
                    },
                    15 => model::Command::Floor {
                        pos1: [args[0].into(), args[1].into(), args[2].into()],
                        pos2: [args[3].into(), args[4].into(), args[5].into()],
                        tex_repeat: [args[6].into(), args[7].into()],
                    },
                    _ => continue,
                });
            }
            Ok(model::Model { old_draw_colour: None, commands, cache: None })
        }
        if let Some(model) = self.models.get_asset_mut(model_id) {
            match load_model(&fname) {
                Ok(new_model) => *model = new_model,
                Err(e) => return Err(gml::Error::FunctionError("d3d_model_load".into(), e.to_string())),
            }
        }
        Ok(Default::default())
    }

    pub fn d3d_model_save(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, fname) = expect_args!(args, [int, string])?;
        fn save_model(model: &model::Model, fname: &str) -> std::io::Result<()> {
            let mut file = std::io::BufWriter::new(std::fs::File::create(fname)?);
            writeln!(&mut file, "100\r\n{}\r", model.commands.len())?;
            for cmd in &model.commands {
                let (cmd, args) = cmd.to_line();
                write!(&mut file, "{}", cmd)?;
                for arg in args.iter() {
                    write!(&mut file, " {:.4}", arg)?;
                }
                writeln!(&mut file, "\r")?;
            }
            file.flush()?;
            Ok(())
        }
        if let Some(model) = self.models.get_asset(model_id) {
            if let Err(e) = save_model(model, &fname) {
                return Err(gml::Error::FunctionError("d3d_model_save".into(), format!("{}", e)))
            }
        }
        Ok(Default::default())
    }

    pub fn d3d_model_draw(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x, y, z, tex_id) = expect_args!(args, [int, real, real, real, int])?;
        let atlas_ref = self.renderer.get_texture_from_id(tex_id as _).copied();
        if let Some(model) = self.models.get_asset_mut(model_id) {
            // translate according to given position
            let old_model_matrix = self.renderer.get_model_matrix();
            #[rustfmt::skip]
                let translation: [f32; 16] = [
                1.0,                    0.0,                    0.0,                    0.0,
                0.0,                    1.0,                    0.0,                    0.0,
                0.0,                    0.0,                    1.0,                    0.0,
                x.into_inner() as f32,  y.into_inner() as f32,  z.into_inner() as f32,  1.0,
            ];
            self.renderer.mult_model_matrix(translation);

            let draw_colour = (u32::from(self.draw_colour) as i32 & 0xfeffff, self.draw_alpha.into_inner());
            if model.cache.is_none() || self.gm_version == Version::GameMaker8_0 {
                // GM8.0 does not use model caching.
                // GM8.1 draws the model semi-normally once, then caches that and redraws.
                let mut buffers = Default::default();
                let mut primitive_draw: Box<dyn FnMut(&mut Renderer)> = match self.gm_version {
                    Version::GameMaker8_0 => Box::new(|r| r.draw_primitive_3d()),
                    Version::GameMaker8_1 => Box::new(|r| r.extend_buffers(&mut buffers)),
                };
                let mut uses_draw_colour = false;
                for command in &model.commands {
                    match command {
                        model::Command::Begin(ptype) => self.renderer.reset_primitive_3d(*ptype, atlas_ref),
                        model::Command::Vertex { pos: [x, y, z], normal: [nx, ny, nz], tex_coord: [xtex, ytex] } => {
                            self.renderer.vertex_3d(
                                x.into_inner(),
                                y.into_inner(),
                                z.into_inner(),
                                nx.into_inner(),
                                ny.into_inner(),
                                nz.into_inner(),
                                xtex.into_inner(),
                                ytex.into_inner(),
                                draw_colour.0,
                                draw_colour.1,
                            );
                            uses_draw_colour = true;
                        },
                        model::Command::VertexColour {
                            pos: [x, y, z],
                            normal: [nx, ny, nz],
                            tex_coord: [xtex, ytex],
                            col: (col, alpha),
                        } => {
                            self.renderer.vertex_3d(
                                x.into_inner(),
                                y.into_inner(),
                                z.into_inner(),
                                nx.into_inner(),
                                ny.into_inner(),
                                nz.into_inner(),
                                xtex.into_inner(),
                                ytex.into_inner(),
                                *col | 1,
                                alpha.into_inner(),
                            );
                        },
                        model::Command::Block { pos1: [x1, y1, z1], pos2: [x2, y2, z2], tex_repeat: [hr, vr] } => {
                            model::draw_block(
                                &mut self.renderer,
                                atlas_ref,
                                &mut primitive_draw,
                                x1.into_inner(),
                                y1.into_inner(),
                                z1.into_inner(),
                                x2.into_inner(),
                                y2.into_inner(),
                                z2.into_inner(),
                                hr.into_inner(),
                                vr.into_inner(),
                                draw_colour.0,
                                draw_colour.1,
                            );
                            uses_draw_colour = true;
                        },
                        model::Command::Cylinder {
                            pos1: [x1, y1, z1],
                            pos2: [x2, y2, z2],
                            tex_repeat: [hr, vr],
                            closed,
                            steps,
                        } => {
                            model::draw_cylinder(
                                &mut self.renderer,
                                atlas_ref,
                                &mut primitive_draw,
                                x1.into_inner(),
                                y1.into_inner(),
                                z1.into_inner(),
                                x2.into_inner(),
                                y2.into_inner(),
                                z2.into_inner(),
                                hr.into_inner(),
                                vr.into_inner(),
                                *closed,
                                *steps,
                                draw_colour.0,
                                draw_colour.1,
                            );
                            uses_draw_colour = true;
                        },
                        model::Command::Cone {
                            pos1: [x1, y1, z1],
                            pos2: [x2, y2, z2],
                            tex_repeat: [hr, vr],
                            closed,
                            steps,
                        } => {
                            let x1 = match self.gm_version {
                                Version::GameMaker8_0 => *x1,
                                Version::GameMaker8_1 => x + *x1, // why is gm8 like this
                            };
                            model::draw_cone(
                                &mut self.renderer,
                                atlas_ref,
                                &mut primitive_draw,
                                x1.into_inner(),
                                y1.into_inner(),
                                z1.into_inner(),
                                x2.into_inner(),
                                y2.into_inner(),
                                z2.into_inner(),
                                hr.into_inner(),
                                vr.into_inner(),
                                *closed,
                                *steps,
                                draw_colour.0,
                                draw_colour.1,
                            );
                            uses_draw_colour = true;
                        },
                        model::Command::Ellipsoid {
                            pos1: [x1, y1, z1],
                            pos2: [x2, y2, z2],
                            tex_repeat: [hr, vr],
                            steps,
                        } => {
                            model::draw_ellipsoid(
                                &mut self.renderer,
                                atlas_ref,
                                &mut primitive_draw,
                                x1.into_inner(),
                                y1.into_inner(),
                                z1.into_inner(),
                                x2.into_inner(),
                                y2.into_inner(),
                                z2.into_inner(),
                                hr.into_inner(),
                                vr.into_inner(),
                                *steps,
                                draw_colour.0,
                                draw_colour.1,
                            );
                            uses_draw_colour = true;
                        },
                        model::Command::Wall { pos1: [x1, y1, z1], pos2: [x2, y2, z2], tex_repeat: [hr, vr] } => {
                            model::draw_wall(
                                &mut self.renderer,
                                atlas_ref,
                                &mut primitive_draw,
                                x1.into_inner(),
                                y1.into_inner(),
                                z1.into_inner(),
                                x2.into_inner(),
                                y2.into_inner(),
                                z2.into_inner(),
                                hr.into_inner(),
                                vr.into_inner(),
                                draw_colour.0,
                                draw_colour.1,
                            );
                            uses_draw_colour = true;
                        },
                        model::Command::Floor { pos1: [x1, y1, z1], pos2: [x2, y2, z2], tex_repeat: [hr, vr] } => {
                            model::draw_floor(
                                &mut self.renderer,
                                atlas_ref,
                                &mut primitive_draw,
                                x1.into_inner(),
                                y1.into_inner(),
                                z1.into_inner(),
                                x2.into_inner(),
                                y2.into_inner(),
                                z2.into_inner(),
                                hr.into_inner(),
                                vr.into_inner(),
                                draw_colour.0,
                                draw_colour.1,
                            );
                            uses_draw_colour = true;
                        },
                        model::Command::End => primitive_draw(&mut self.renderer),
                    }
                }
                if uses_draw_colour {
                    model.old_draw_colour = Some(draw_colour);
                }
                drop(primitive_draw);
                model.cache = Some(buffers);
            }
            if self.gm_version == Version::GameMaker8_1 {
                let cache = model.cache.as_mut().unwrap();
                if let Some(old_col) = model.old_draw_colour {
                    cache.swap_colour(old_col, draw_colour);
                    model.old_draw_colour = Some(draw_colour);
                }
                self.renderer.draw_buffers(atlas_ref, cache);
            }

            self.renderer.set_model_matrix(old_model_matrix);
        }
        Ok(Default::default())
    }

    pub fn d3d_model_primitive_begin(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, kind) = expect_args!(args, [int, int])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Begin(kind.into()));
        }
        Ok(Default::default())
    }

    pub fn d3d_model_primitive_end(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let model_id = expect_args!(args, [int])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::End);
        }
        Ok(Default::default())
    }

    pub fn d3d_model_vertex(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x, y, z) = expect_args!(args, [int, real, real, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Vertex {
                pos: [x, y, z],
                normal: [0.into(); 3],
                tex_coord: [0.into(); 2],
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_vertex_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x, y, z, col, alpha) = expect_args!(args, [int, real, real, real, int, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::VertexColour {
                pos: [x, y, z],
                normal: [0.into(); 3],
                tex_coord: [0.into(); 2],
                col: (col, alpha),
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_vertex_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x, y, z, xtex, ytex) = expect_args!(args, [int, real, real, real, real, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Vertex {
                pos: [x, y, z],
                normal: [0.into(); 3],
                tex_coord: [xtex, ytex],
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_vertex_texture_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x, y, z, xtex, ytex, col, alpha) =
            expect_args!(args, [int, real, real, real, real, real, int, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::VertexColour {
                pos: [x, y, z],
                normal: [0.into(); 3],
                tex_coord: [xtex, ytex],
                col: (col, alpha),
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_vertex_normal(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x, y, z, nx, ny, nz) = expect_args!(args, [int, real, real, real, real, real, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Vertex {
                pos: [x, y, z],
                normal: [nx, ny, nz],
                tex_coord: [0.into(); 2],
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_vertex_normal_color(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x, y, z, nx, ny, nz, col, alpha) =
            expect_args!(args, [int, real, real, real, real, real, real, int, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::VertexColour {
                pos: [x, y, z],
                normal: [nx, ny, nz],
                tex_coord: [0.into(); 2],
                col: (col, alpha),
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_vertex_normal_texture(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x, y, z, nx, ny, nz, xtex, ytex) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Vertex {
                pos: [x, y, z],
                normal: [nx, ny, nz],
                tex_coord: [xtex, ytex],
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_vertex_normal_texture_color(
        &mut self,
        _context: &mut Context,
        args: &[Value],
    ) -> gml::Result<Value> {
        let (model_id, x, y, z, nx, ny, nz, xtex, ytex, col, alpha) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, int, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::VertexColour {
                pos: [x, y, z],
                normal: [nx, ny, nz],
                tex_coord: [xtex, ytex],
                col: (col, alpha),
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_block(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x1, y1, z1, x2, y2, z2, hrepeat, vrepeat) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Block {
                pos1: [x1, y1, z1],
                pos2: [x2, y2, z2],
                tex_repeat: [hrepeat, vrepeat],
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_cylinder(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x1, y1, z1, x2, y2, z2, hrepeat, vrepeat, closed, steps) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, bool, int])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Cylinder {
                pos1: [x1, y1, z1],
                pos2: [x2, y2, z2],
                tex_repeat: [hrepeat, vrepeat],
                closed,
                steps,
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_cone(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x1, y1, z1, x2, y2, z2, hrepeat, vrepeat, closed, steps) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, bool, int])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Cone {
                pos1: [x1, y1, z1],
                pos2: [x2, y2, z2],
                tex_repeat: [hrepeat, vrepeat],
                closed,
                steps,
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_ellipsoid(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x1, y1, z1, x2, y2, z2, hrepeat, vrepeat, steps) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real, int])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Ellipsoid {
                pos1: [x1, y1, z1],
                pos2: [x2, y2, z2],
                tex_repeat: [hrepeat, vrepeat],
                steps,
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_wall(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x1, y1, z1, x2, y2, z2, hrepeat, vrepeat) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Wall {
                pos1: [x1, y1, z1],
                pos2: [x2, y2, z2],
                tex_repeat: [hrepeat, vrepeat],
            });
        }
        Ok(Default::default())
    }

    pub fn d3d_model_floor(&mut self, _context: &mut Context, args: &[Value]) -> gml::Result<Value> {
        let (model_id, x1, y1, z1, x2, y2, z2, hrepeat, vrepeat) =
            expect_args!(args, [int, real, real, real, real, real, real, real, real])?;
        if let Some(model) = self.models.get_asset_mut(model_id) {
            model.commands.push(model::Command::Floor {
                pos1: [x1, y1, z1],
                pos2: [x2, y2, z2],
                tex_repeat: [hrepeat, vrepeat],
            });
        }
        Ok(Default::default())
    }
}
