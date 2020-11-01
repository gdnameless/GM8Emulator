use crate::{asset::Sprite, game::string::RCStr};
use encoding_rs::Encoding;
use gmio::{
    atlas::AtlasBuilder,
    render::{AtlasRef, Renderer},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Font {
    pub name: RCStr,
    pub sys_name: RCStr,
    pub charset: u32,
    pub size: u32,
    pub bold: bool,
    pub italic: bool,
    pub first: u32,
    pub last: u32,
    pub tallest_char_height: u32,
    pub chars: Box<[Character]>,
    pub own_graphics: bool, // Does this Font own the graphics associated with it?
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Character {
    pub offset: i32,
    pub distance: i32,
    pub atlas_ref: AtlasRef,
}

impl Font {
    pub fn get_char(&self, index: u32) -> Option<Character> {
        if let Some(index) = index.checked_sub(self.first) { self.chars.get(index as usize).copied() } else { None }
    }

    pub fn get_encoding(&self, default: &'static Encoding) -> &'static Encoding {
        match self.charset {
            0x00 => encoding_rs::WINDOWS_1252, // ANSI_CHARSET
            0x80 => encoding_rs::SHIFT_JIS,    // SHIFTJIS_CHARSET
            0x81 => encoding_rs::EUC_KR,       // HANGUL_CHARSET
            0x82 => default,                   // JOHAB_CHARSET
            0x86 => encoding_rs::GBK,          // GB2312_CHARSET
            0x88 => encoding_rs::BIG5,         // CHINESEBIG5_CHARSET
            0xA1 => encoding_rs::WINDOWS_1253, // GREEK_CHARSET
            0xA2 => encoding_rs::WINDOWS_1254, // TURKISH_CHARSET
            0xA3 => encoding_rs::WINDOWS_1258, // VIETNAMESE_CHARSET
            0xB1 => encoding_rs::WINDOWS_1255, // HEBREW_CHARSET
            0xB2 => encoding_rs::WINDOWS_1256, // ARABIC_CHARSET
            0xBA => encoding_rs::WINDOWS_1257, // BALTIC_CHARSET
            0xCC => encoding_rs::WINDOWS_1251, // RUSSIAN_CHARSET
            0xDE => encoding_rs::WINDOWS_874,  // THAI_CHARSET
            0xEE => encoding_rs::WINDOWS_1250, // EASTEUROPE_CHARSET
            _ => default,
        }
    }
}

pub fn create_chars_from_ttf(
    data: &[u8],
    scale: f32,
    first: u8,
    last: u8,
    atlases: &mut AtlasBuilder,
) -> Result<(Box<[Character]>, u32), String> {
    // TODO: figure out runtime font loading
    let font = rusttype::Font::try_from_bytes(data).ok_or("Couldn't load font")?;
    let v_offset = (scale * 4.0 / 3.0) as i32;
    let scale = rusttype::Scale::uniform(scale * 1.5);
    let mut max_height = 0;
    (first..=last)
        .map(|i| {
            // TODO: use the relevant encoding
            let glyph = font.glyph(char::from(i)).scaled(scale).positioned(rusttype::Point { x: 0.0, y: 0.0 });
            let (x, y, w, h) = match glyph.pixel_bounding_box() {
                Some(bbox) => (-bbox.min.x, -bbox.min.y, bbox.max.x - bbox.min.x, bbox.max.y - bbox.min.y),
                None => (0, 0, 0, 0),
            };
            let y = y - v_offset;
            if h > max_height {
                max_height = h;
            }
            let mut data: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);
            glyph.draw(|_, _, a| {
                data.push(0xFF);
                data.push(0xFF);
                data.push(0xFF);
                data.push((a * 255.0) as u8);
            });
            let atlas_ref = atlases.texture(w, h, x, y, data.into_boxed_slice()).ok_or("Couldn't pack font")?;
            let hmetrics = glyph.unpositioned().h_metrics();
            Ok(Character { offset: hmetrics.advance_width as _, distance: hmetrics.left_side_bearing as _, atlas_ref })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|v| (v.into_boxed_slice(), max_height as _))
}

pub fn create_chars_from_sprite(sprite: &Sprite, prop: bool, sep: i32, renderer: &Renderer) -> Box<[Character]> {
    let mut chars = Vec::with_capacity(sprite.frames.len());
    if prop {
        // proportional font, get the left and right bounds of each character
        for frame in &sprite.frames {
            let data = renderer.dump_sprite(&frame.atlas_ref);
            let column_empty =
                |&x: &u32| (0..sprite.height).any(|y| data[(y * sprite.width + x) as usize * 4 + 3] != 0);
            let left_edge = (0..sprite.width).find(column_empty).map(|x| x as i32).unwrap_or(sprite.width as i32 - 1);
            let right_edge = (0..sprite.width).rfind(column_empty).unwrap_or(0) as i32;
            chars.push(Character {
                offset: right_edge + sep - left_edge,
                distance: -left_edge,
                atlas_ref: frame.atlas_ref.clone(),
            });
        }
    } else {
        // non-proportional font, just add them whole
        chars.extend(sprite.frames.iter().map(|f| Character {
            offset: f.width as i32 + sep,
            distance: 0,
            atlas_ref: f.atlas_ref.clone(),
        }));
    }
    chars.into_boxed_slice()
}
