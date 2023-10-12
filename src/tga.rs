#![allow(dead_code)]
// Using
use std::mem;

enum TgaFormatType {
    R8 = 1,
    RGB5A1 = 2,
    RGB8 = 3,
    RGBA8 = 4,
}

enum TgaColorType {
    R = 1,
    RGB = 3,
    RGBA = 4,
}

enum TgaImageType {
    None = 0,
    Indexed = 1,
    RGB = 2,
    Grey = 3,
    IndexedRLE = 9,
    RGBRLE = 10,
    GreyRLE = 11,
}

#[repr(packed)]
struct TGAHeader {
    m_identsize: u8,
    m_colourmaptype: u8,
    m_imagetype: u8,
    m_colourmapstart: i16,
    m_colourmaplength: i16,
    m_colourmapbits: u8,
    m_xstart: i16,
    m_ystart: i16,
    m_width: i16,
    m_height: i16,
    m_bits: u8,
    m_descriptor: u8,
}

const VERTICAL_FLIP_MASK: u8 = 0b00100000;
const HORIZONTAL_FLIP_MASK: u8 = 0b00010000;

fn vertical_flip(tga_h: &TGAHeader) -> bool {
    tga_h.m_descriptor & VERTICAL_FLIP_MASK != 0
}

fn horizontal_flip(tga_h: &TGAHeader) -> bool {
    tga_h.m_descriptor & HORIZONTAL_FLIP_MASK != 0
}

fn decoder_rle(width: usize, height: usize, image_bytes_pixel: usize, buffer_in: &[u8]) -> Vec<u8> {
    let mut out_image = vec![0u8; width * height * image_bytes_pixel];
    let mut i = 0;
    let mut idx = 0;

    while i < out_image.len() {
        // Read header
        let header_byte = buffer_in[idx];
        let is_rle_chunk = header_byte & 0x80 != 0;
        // Next
        idx += 1;
        // RLE or RAW?
        if is_rle_chunk {
			// 1XXX XXXX RLE chunk [ LEN | PIXEL ]
            let length_chunk = header_byte as usize - 127;
            // Pixel
            let pixel_data = &buffer_in[idx..idx + image_bytes_pixel];
            // Next
            idx += image_bytes_pixel;
            // Copy n times
            for _ in 0..length_chunk {
                out_image[i..i + image_bytes_pixel].copy_from_slice(pixel_data);
                i += image_bytes_pixel;
            }
        } else {
			// RAW chunk  [ LEN | PIXEL | PIXEL |... ]
            let length_chunk = header_byte as usize + 1;
            // Copy all pixels
            for _ in 0..length_chunk {
                let pixel_data = &buffer_in[idx..idx + image_bytes_pixel];
                out_image[i..i + image_bytes_pixel].copy_from_slice(pixel_data);
                i += image_bytes_pixel;
                idx += image_bytes_pixel;
            }
        };
    }
    return out_image;
}

fn read_rgba5551(data: &[u8]) -> (u8, u8, u8, u8) {
    let r = (data[0] & 0xf8) >> 3;
    let g = ((data[0] & 0x07) << 2) | ((data[1] & 0xfb) >> 6);
    let b = (data[1] & 0x3e) >> 1;
    let a = data[1] & 0x01;
    (r, g, b, a)
}

fn write_rgba5551(data: &mut [u8], r: u8, g: u8, b: u8, a: u8) {
    data[0] = (r << 3) | (data[0] & 0x07);
    data[0] = ((g & 0x1c) >> 2) | (data[0] & 0xf8);
    data[1] = ((g & 0x03) << 6) | (data[1] & 0xf8);
    data[1] = (b << 1) | (data[1] & 0xc1);
    data[1] = a | (data[1] & 0xfe);
}

pub fn from_rgba5551_to_rgba_32(data: &[u8], width: usize, height: usize) -> Option< Vec<u8> > {
    // Alloc
    const IN_BYTES_PER_PIXEL: usize = 2;
    const OUT_BYTES_PER_PIXEL: usize = 4;
    let mut output: Vec<u8> = Vec::with_capacity(OUT_BYTES_PER_PIXEL * width * height);
    // Iterator of pixels
    let mut chk_data = data.chunks(IN_BYTES_PER_PIXEL);
    // Convert
    for _ in 0..( width * height ) {
        let chk = match chk_data.next() {
            Some(value) => value,
            None => return None
        };
        let (r,g,b,a) = read_rgba5551(chk);
        output.extend_from_slice(&[((r as u16 * 255) / 31) as u8, ((g as u16 * 255) / 31) as u8, ((b as u16 * 255) / 31) as u8, a * 255]);
    }

    return Some(output);
}

fn rga_swap_r_and_b_16(bytes: &mut [u8], width: usize, height: usize) {
    let image_size = width * height;
    for i in 0..image_size {
        let (mut r, g, mut b, a) = read_rgba5551(&bytes[i * 2..]);
        mem::swap(&mut r, &mut b);
        write_rgba5551(&mut bytes[i * 2..], r, g, b, a);
    }
}

fn rga_swap_r_and_b_24_32(bytes: &mut [u8], image_bytes_pixel: usize, width: usize, height: usize) {
    let image_size = width * height;
    for i in 0..image_size {
        let tmp = bytes[i * image_bytes_pixel];
        bytes[i * image_bytes_pixel] = bytes[i * image_bytes_pixel + 2];
        bytes[i * image_bytes_pixel + 2] = tmp;
    }
}

fn image_y_flip(
    bytes: &mut Vec<u8>,
    image_bytes_pixel: usize,
    image_width: usize,
    image_height: usize,
) {
    let line_len = image_width * image_bytes_pixel * mem::size_of::<u8>();
    let half_height = image_height / 2;
    let mut temp_line = vec![0u8; line_len];
    let mut temp_line_flip = vec![0u8; line_len];

    for y in 0..half_height {
        let line_start = line_len * y;
        let line_end = line_len * (y + 1);
        let flipped_start = line_len * (image_height - y - 1);
        let flipped_end = line_len * image_height;
        temp_line.copy_from_slice(&bytes[line_start..line_end]);
        temp_line_flip.copy_from_slice(&bytes[flipped_start..flipped_end]);
        bytes[line_start..line_end].copy_from_slice(&temp_line_flip);
        bytes[flipped_start..flipped_end].copy_from_slice(&temp_line);
    }
}

fn image_x_flip(
    bytes: &mut Vec<u8>,
    image_bytes_pixel: usize,
    image_width: usize,
    image_height: usize,
) {
    assert!(image_bytes_pixel <= 4);
    let line_width = image_width * image_bytes_pixel;
    let half_width: usize = image_width / 2;
    let mut temp_pixel_left = [0u8; 4];
    let mut temp_pixel_right = [0u8; 4];

    for y in 0..image_height {
        for x in 0..half_width {
            temp_pixel_left.copy_from_slice(&bytes[x + line_width * y..(x + 1) + line_width * y]);
            temp_pixel_right.copy_from_slice( &bytes[(line_width - x - 1) + line_width * y..line_width * (y + 1)]);
            bytes[x + line_width * y..(x + 1) + line_width * y].copy_from_slice(&temp_pixel_right);
            bytes[(line_width - x - 1) + line_width * y..line_width * (y + 1)].copy_from_slice(&temp_pixel_left);
        }
    }
}

pub fn decode_tga(
    out_image: &mut Vec<u8>,
    image_width: &mut usize,
    image_height: &mut usize,
    image_format: &mut u8,
    image_type: &mut u8,
    in_tga: &[u8],
) -> bool {
    let header: &TGAHeader = unsafe { mem::transmute(&in_tga[0]) };
    let data = &in_tga[mem::size_of::<TGAHeader>()..];

    if header.m_imagetype != TgaImageType::RGB as u8
        && header.m_imagetype != TgaImageType::RGBRLE as u8
    {
        return false;
    }

    *image_width = header.m_width as usize;
    *image_height = header.m_height as usize;

    match header.m_bits {
        8 => {
            *image_format = TgaFormatType::R8 as u8;
            *image_type = TgaColorType::R as u8;
        }
        16 => {
            *image_format = TgaFormatType::RGB5A1 as u8;
            *image_type = TgaColorType::RGBA as u8;
        }
        24 => {
            *image_format = TgaFormatType::RGB8 as u8;
            *image_type = TgaColorType::RGB as u8;
        }
        32 => {
            *image_format = TgaFormatType::RGBA8 as u8;
            *image_type = TgaColorType::RGBA as u8;
        }
        _ => return false,
    }

    if header.m_imagetype == TgaImageType::RGBRLE as u8 {
        *out_image = decoder_rle(
            *image_width,
            *image_height,
            header.m_bits as usize / 8,
            data,
        );
    } else {
        out_image.resize(data.len(), 0);
        out_image.copy_from_slice(data);
    }

    match header.m_bits {
        16 => rga_swap_r_and_b_16(out_image, *image_width, *image_height),
        24 | 32 => rga_swap_r_and_b_24_32(
            out_image,
            header.m_bits as usize / 8,
            *image_width,
            *image_height,
        ),
        _ => {}
    }

    if vertical_flip(header) {
        image_y_flip(
            out_image,
            header.m_bits as usize / 8,
            *image_width,
            *image_height,
        );
    }
    if horizontal_flip(header) {
        image_x_flip(
            out_image,
            header.m_bits as usize / 8,
            *image_width,
            *image_height,
        );
    }

    true
}
