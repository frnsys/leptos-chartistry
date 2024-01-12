mod colourmaps;

pub use colourmaps::*;

use leptos::signal_prelude::*;

/*
Colours are an important part of charts. Our aim is to avoid less readable and misleading colour schemes. So we rely on the scientific colour maps developed by Fabio Crameri. These are perceptually uniform, colour blind friendly, and monochrome friendly.

Reading material:
- Summary poster: https://www.fabiocrameri.ch/ws/media-library/a17d02961b3a4544961416de2d7900a4/posterscientificcolourmaps_crameri.pdf
- Article "The misuse of colour in science communication" https://www.nature.com/articles/s41467-020-19160-7
- Homepage https://www.fabiocrameri.ch/colourmaps/
- Picking a colour scheme: https://s-ink.org/colour-map-guideline
*/

pub const GREY_LAYOUT: [Colour; 3] = [
    Colour::new(0x9A, 0x9A, 0x9A), // Light grey
    Colour::new(0xD2, 0xD2, 0xD2), // Lighter grey
    Colour::new(0xEF, 0xF2, 0xFA), // Lightest grey
];

/// Arbitrary colours for a brighter palette
pub const ARBITRARY: [Colour; 10] = [
    Colour::new(0x12, 0xA5, 0xED), // Blue
    Colour::new(0xF5, 0x32, 0x5B), // Red
    Colour::new(0x71, 0xc6, 0x14), // Green
    Colour::new(0xFF, 0x84, 0x00), // Orange
    Colour::new(0x7b, 0x4d, 0xff), // Purple
    Colour::new(0xdb, 0x4c, 0xb2), // Magenta
    Colour::new(0x92, 0xb4, 0x2c), // Darker green
    Colour::new(0xFF, 0xCA, 0x00), // Yellow
    Colour::new(0x22, 0xd2, 0xba), // Turquoise
    Colour::new(0xea, 0x60, 0xdf), // Pink
];

#[derive(Clone, Debug, PartialEq)]
pub struct ColourScheme {
    swatches: Vec<Colour>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColourScheme {
    fn new(swatches: Vec<Colour>) -> Self {
        Self { swatches }
    }

    pub fn by_index(&self, index: usize) -> Colour {
        let index = index.rem_euclid(self.swatches.len());
        self.swatches[index]
    }
}

impl Colour {
    const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn signal_option(
        colour: MaybeSignal<Option<Colour>>,
        layout: Memo<ColourScheme>,
        index: usize,
    ) -> Signal<Colour> {
        Signal::derive(move || colour.get().unwrap_or_else(|| layout.get().by_index(index)))
    }
}

impl From<&[Colour]> for ColourScheme {
    fn from(colours: &[Colour]) -> Self {
        Self::new(colours.to_vec())
    }
}

impl From<[Colour; 3]> for ColourScheme {
    fn from(colours: [Colour; 3]) -> Self {
        colours.as_ref().into()
    }
}

impl From<[Colour; 10]> for ColourScheme {
    fn from(colours: [Colour; 10]) -> Self {
        colours.as_ref().into()
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
