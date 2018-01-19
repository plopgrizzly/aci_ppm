// Aldaron's Codec Interface / PPM
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Aldaron's Codec Interface / PPM is a small library developed by Plop Grizzly
//! for decoding ppm image files.

#![warn(missing_docs)]
#![doc(html_logo_url = "http://plopgrizzly.com/icon.png",
	html_favicon_url = "http://plopgrizzly.com/icon.png",
	html_root_url = "http://plopgrizzly.com/aci_png/")]

extern crate afi;

use afi::GraphicBuilder;
pub use afi::{ Graphic, GraphicDecodeErr };

fn skip_line(ppm: &[u8], index: &mut usize) {
	while ppm[*index] != b'\n' {
		*index += 1;
	}
	*index += 1;
}

fn utf8_to_u32(ppm: &[u8], index: &mut usize, until: u8)
	-> Result<u32, GraphicDecodeErr>
{
	let zero = b'0';
	let mut number = 0;

	while ppm[*index] != until {
		let digit = ppm[*index];

		number *= 10;
		if digit != zero {
			if digit < zero || digit > zero + 9 {
				return Err(GraphicDecodeErr::BadNum);
			}
			number += (digit - zero) as u32;
		}
		*index += 1;
	}
	*index += 1;
	Ok(number)
}

/// Decode PPM data.  On success, returns image as a `Graphic`.
pub fn decode(ppm: &[u8]) -> Result<Graphic, GraphicDecodeErr> {
	let mut index = 3;

	// Header
	if ppm[0] != b'P' || ppm[1] != b'6' {
		return Err(GraphicDecodeErr::IncorrectFormat);
	}

	// Optional Comment
	if ppm[index] == b'#' {
		skip_line(ppm, &mut index);
	}

	// Width & Height
	let width = utf8_to_u32(ppm, &mut index, b' ')?;
	let height = utf8_to_u32(ppm, &mut index, b'\n')?;

	// Allocate RGBA data.
	let size = (width * height) as usize;
	let mut out: Vec<u32> = Vec::with_capacity(size + 2);

	// We don't care about this.  In ppm format 255 is normally here
	skip_line(ppm, &mut index);

	// Build Graphic
	let graphic = {
		let buf = &ppm[index..];
		let mut pixel : [u8;4] = [0xFF, 0xFF, 0xFF, 0xFF];

		for i in 0..size {
			pixel[0] = buf[i * 3 + 0];
			pixel[1] = buf[i * 3 + 1];
			pixel[2] = buf[i * 3 + 2];
			pixel[3] = 255;

			out.push(unsafe { ::std::mem::transmute(pixel) });
		}

		GraphicBuilder::new().rgba(width, height, out)
	};

	Ok(graphic)
}
