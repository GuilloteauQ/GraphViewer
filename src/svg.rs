//! # SVG
//!
//! A library for creating simple SVG files

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

/// # The Svg Struct
/// The struct representing a svg file
/// It contains the width, the height and the file
pub struct Svg {
    /// Width of the image
    pub w: u32,
    /// Height of the image
    pub h: u32,
    /// The file itself
    pub file: File,
}

impl Svg {
    /// Opens the file with the name 'filename' and returns
    /// an Svg object with the height and the width of the file
    pub fn new(filename: String, w: u32, h: u32) -> Svg {
        Svg {
            w: w,
            h: h,
            file: File::create(filename).unwrap(),
        }
    }

    /// Writes the header of the file
    pub fn header(&self) {
        let mut writer = BufWriter::new(&self.file);
        write!(
            &mut writer,
            "<?xml version='1.0'?>\n
<svg viewBox='0 0 {} {}' version='1.1' xmlns='http://www.w3.org/2000/svg'>\n",
            self.w, self.h
        ).unwrap();
    }

    /// Writes the footer of the file
    pub fn footer(&self) {
        let mut writer = BufWriter::new(&self.file);
        write!(&mut writer, "</svg>\n").unwrap();
    }

    /// Draws a rectangle at (x, y) with height:h and weidth:w and color
    pub fn rectangle(&self, x: u32, y: u32, h: u32, w: u32, color: String) {
        let mut writer = BufWriter::new(&self.file);
        write!(
            &mut writer,
            "<rect x='{}' y='{}' width='{}' height='{}' fill='{}'/>\n",
            x, y, w, h, color
        ).unwrap();
    }

    /// Draws an ellipse at (cx, cy) with the radius (rx, ry) filled in color
    pub fn ellipse(&self, cx: u32, cy: u32, rx: u32, ry: u32, color: String) {
        let mut writer = BufWriter::new(&self.file);
        write!(
            &mut writer,
            "<ellipse cx='{}' cy='{}' rx='{}' ry='{}' fill='{}'/>\n",
            cx, cy, rx, ry, color
        ).unwrap();
    }

    /// Draws a circle at (cx, cy) with the radius r
    pub fn circle(&self, cx: u32, cy: u32, r: u32, color: String) {
        self.ellipse(cx, cy, r, r, color);
    }

    /// Draws a line from (x1, y1) to (x2, y2)
    pub fn line(&self, x1: u32, y1: u32, x2: u32, y2: u32, color: String) {
        let mut writer = BufWriter::new(&self.file);
        write!(
            &mut writer,
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{}'/>\n",
            x1, y1, x2, y2, color
        ).unwrap();
    }

    /// Draws a line from (x1, y1) to (x2, y2)
    pub fn line_animated(&self, x1: u32, y1: u32, x2: u32, y2: u32, color: String, begin: u32) {
        let mut writer = BufWriter::new(&self.file);
        write!(
            &mut writer,
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{}'>\n<set attributeName='x2' attributeType='XML' to='{}' begin='{}ms' />\n<set attributeName='y2' attributeType='XML' to='{}' begin='{}ms'/>\n</line>\n",
            x1, y1, x1, y1, color, x2, begin, y2, begin
        ).unwrap();
    }

    ///  Write text in the SVG file
    pub fn text(&self, x: u32, y: u32, text: String, color: String) {
        let mut writer = BufWriter::new(&self.file);
        write!(
            &mut writer,
            "<text x='{}' y='{}' fill='{}'>{}</text>\n",
            x, y, color, text
        ).unwrap();
    }
}
