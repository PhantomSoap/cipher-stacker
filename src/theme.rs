use ratatui::style::Color;

#[must_use]
pub const fn resolve_rgb(color: Color) -> Option<(u8, u8, u8)> {
    match color {
        Color::Rgb(r, g, b) => Some((r, g, b)),
        Color::Black => Some((0, 0, 0)),
        Color::Red => Some((128, 0, 0)),
        Color::Green => Some((0, 128, 0)),
        Color::Yellow => Some((128, 128, 0)),
        Color::Blue => Some((0, 0, 128)),
        Color::Magenta => Some((128, 0, 128)),
        Color::Cyan => Some((0, 128, 128)),
        Color::Gray => Some((192, 192, 192)),
        Color::DarkGray => Some((128, 128, 128)),
        Color::LightRed => Some((255, 0, 0)),
        Color::LightGreen => Some((0, 255, 0)),
        Color::LightYellow => Some((255, 255, 0)),
        Color::LightBlue => Some((0, 0, 255)),
        Color::LightMagenta => Some((255, 0, 255)),
        Color::LightCyan => Some((0, 255, 255)),
        Color::White => Some((255, 255, 255)),
        Color::Indexed(_) | Color::Reset => None,
    }
}

#[must_use]
#[allow(
    clippy::cast_possible_truncation,
    reason = "each channel is a weighted average of two u8s, so it stays within 0..=255 and the `as u8` cast cannot truncate"
)]
pub const fn dim(color: Color, toward: Color, amount: u16) -> Color {
    match (resolve_rgb(color), resolve_rgb(toward)) {
        (Some((r, g, b)), Some((tr, tg, tb))) => {
            let amount = if amount > 100 { 100 } else { amount };
            let keep = 100 - amount;
            // `+ 50` rounds to nearest rather than flooring.
            Color::Rgb(
                ((r as u16 * keep + tr as u16 * amount + 50) / 100) as u8,
                ((g as u16 * keep + tg as u16 * amount + 50) / 100) as u8,
                ((b as u16 * keep + tb as u16 * amount + 50) / 100) as u8,
            )
        }
        _ => color,
    }
}

//Theme Implementation from ratcn
pub const PRESETS: [Theme; 7] = [
    Theme::default_dark(),
    Theme::terminal(),
    Theme::catppuccin(),
    Theme::gruvbox(),
    Theme::nord(),
    Theme::tokyo_night(),
    Theme::solarized(),
];

/// Preset focus rings are derived, not stored: the preset's `primary` dimmed
/// this much toward its `background`.
///
/// The softening is small because a ring is a boundary — it has to clear 3:1
/// against both the background it is drawn on and the surface it frames, and
/// dimming toward the background is the ring spending exactly the contrast it
/// exists to have. A mid-luminance accent (Solarized's blue) runs out first.
const RING_DIM: u16 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Theme {
    /// Display name, for a theme picker.
    pub name: &'static str,

    // Text
    /// Ordinary text.
    pub foreground: Color,
    /// De-emphasized text: placeholders, descriptions, disabled labels.
    pub muted_foreground: Color,

    // Surfaces, back to front
    /// The furthest-back surface, behind everything else.
    pub background: Color,
    /// Raised surfaces that sit above the background — dialogs and toasts.
    pub surface: Color,
    /// Control surfaces — lists, select panels, and chart backdrops.
    pub field: Color,

    // Accents
    /// The workhorse accent: default buttons, selected rows, bars.
    pub primary: Color,
    /// Text drawn on top of `primary`, so it must contrast with it.
    pub primary_foreground: Color,
    /// The quieter accent, for secondary buttons and unselected tabs.
    pub secondary: Color,
    /// Text drawn on top of `secondary`.
    pub secondary_foreground: Color,
    /// The palette's vivid signature color, held back for occasional emphasis
    /// rather than used as a second `primary`.
    pub accent: Color,
    /// Destructive actions and error states.
    pub destructive: Color,
    /// Text drawn on top of `destructive`.
    pub destructive_foreground: Color,
    /// Warnings — less severe than `destructive`.
    pub warning: Color,

    // Lines and caret
    /// Borders and separators at rest.
    pub border: Color,
    /// The focus accent around a container — a focused pane's border, the
    /// modal dialog's frame. The shadcn `ring` role. Presets derive it from
    /// `primary`, dimmed 10% toward `background`.
    pub ring: Color,
    /// A text caret. No component ships with one today; the role is kept so
    /// text-entry components can be themed consistently when they land.
    pub cursor: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_dark()
    }
}

impl Theme {
    /// A dark default theme inspired by shadcn/ui's default OKLCH palette,
    /// translated to opaque sRGB terminal colors.
    #[must_use]
    pub const fn default_dark() -> Self {
        let primary = Color::Rgb(229, 229, 229);
        let background = Color::Rgb(10, 10, 10);
        Self {
            name: "Default",
            foreground: Color::Rgb(250, 250, 250),
            muted_foreground: Color::Rgb(161, 161, 161),
            background,
            surface: Color::Rgb(18, 18, 18),
            field: Color::Rgb(31, 31, 31),
            primary,
            primary_foreground: Color::Rgb(23, 23, 23),
            secondary: Color::Rgb(38, 38, 38),
            secondary_foreground: Color::Rgb(250, 250, 250),
            accent: Color::Rgb(139, 92, 246),
            destructive: Color::Rgb(255, 100, 103),
            // The same tone `primary_foreground` uses, and for the same reason:
            // both are labels on a bright fill, where only a dark label holds
            // the floor across all three fill states.
            destructive_foreground: Color::Rgb(23, 23, 23),
            warning: Color::Rgb(250, 204, 21),
            border: Color::Rgb(94, 94, 94),
            ring: dim(primary, background, RING_DIM),
            cursor: Color::Rgb(115, 115, 115),
        }
    }

    /// The terminal's own palette: [`Color::Reset`] for the outer background
    /// and body text, named colors for the rest, whose derived states resolve
    /// through [`resolve_rgb`]'s fixed VGA approximation.
    ///
    /// `primary` is a neutral rather than a hue, because that approximation
    /// replaces a named accent with a fixed rgb the moment a state derives from
    /// it, and a neutral degrades into a neutral.
    ///
    /// The well surfaces are concrete dark rgb rather than [`Color::Reset`],
    /// which carries no channels for a focus, hover, or cursor-row fill to
    /// lighten from — so a light terminal gets dark wells.
    #[must_use]
    pub const fn terminal() -> Self {
        let primary = Color::White;
        let background = Color::Reset;
        Self {
            name: "Terminal",
            foreground: Color::Reset,
            muted_foreground: Color::Gray,
            background,
            surface: Color::Rgb(18, 18, 18),
            field: Color::Rgb(31, 31, 31),
            primary,
            primary_foreground: Color::Black,
            secondary: Color::Rgb(38, 38, 38),
            secondary_foreground: Color::Reset,
            accent: Color::LightMagenta,
            destructive: Color::LightRed,
            destructive_foreground: Color::Black,
            warning: Color::Yellow,
            border: Color::DarkGray,
            ring: dim(primary, background, RING_DIM),
            cursor: Color::LightBlue,
        }
    }

    /// The Catppuccin Mocha palette: soft pastels on a deep indigo background,
    /// from `catppuccin/palette`'s `palette.json`.
    ///
    /// `surface` is off-palette, 30% of the way from `crust` to `base`; the
    /// flavor's own middle tone, `mantle`, leaves too little between a dialog
    /// and a well drawn inside it.
    #[must_use]
    pub const fn catppuccin() -> Self {
        let primary = Color::Rgb(137, 180, 250); // blue #89b4fa
        let background = Color::Rgb(17, 17, 27); // crust #11111b
        let field = Color::Rgb(30, 30, 46); // base #1e1e2e
        Self {
            name: "Catppuccin",
            foreground: Color::Rgb(205, 214, 244), // text #cdd6f4
            muted_foreground: Color::Rgb(166, 173, 200), // subtext0 #a6adc8
            background,
            surface: dim(background, field, 30),
            field,
            primary,
            primary_foreground: Color::Rgb(17, 17, 27), // crust #11111b
            secondary: Color::Rgb(49, 50, 68),          // surface0 #313244
            secondary_foreground: Color::Rgb(205, 214, 244), // text #cdd6f4
            accent: Color::Rgb(203, 166, 247),          // mauve #cba6f7
            destructive: Color::Rgb(243, 139, 168),     // red #f38ba8
            destructive_foreground: Color::Rgb(17, 17, 27), // crust #11111b
            warning: Color::Rgb(250, 179, 135),         // peach #fab387
            border: Color::Rgb(108, 112, 134),          // overlay0 #6c7086
            ring: dim(primary, background, RING_DIM),
            cursor: Color::Rgb(137, 180, 250), // blue #89b4fa
        }
    }

    /// The Gruvbox dark palette: warm, low-contrast retro tones, from
    /// `morhetz/gruvbox`'s `colors/gruvbox.vim`, using the *bright* accent
    /// variants dark mode calls for.
    ///
    /// `muted_foreground` is `light2` rather than the dimmer `light4`, which
    /// does not read on the upper rungs of the well ladder.
    ///
    /// `destructive_foreground` is `dark0_hard` rather than `dark0`: no tone
    /// labels the mid-luminance `bright_red` across a button's states, so the
    /// pair takes a documented contrast exception on the palette's darkest
    /// tone.
    #[must_use]
    pub const fn gruvbox() -> Self {
        let primary = Color::Rgb(250, 189, 47); // bright yellow #fabd2f
        let background = Color::Rgb(40, 40, 40); // dark0 #282828
        Self {
            name: "Gruvbox",
            foreground: Color::Rgb(235, 219, 178), // light1 #ebdbb2
            muted_foreground: Color::Rgb(213, 196, 161), // light2 #d5c4a1
            background,
            surface: Color::Rgb(50, 48, 47), // dark0_soft #32302f
            field: Color::Rgb(60, 56, 54),   // dark1 #3c3836
            primary,
            primary_foreground: Color::Rgb(40, 40, 40), // dark0 #282828
            secondary: Color::Rgb(80, 73, 69),          // dark2 #504945
            secondary_foreground: Color::Rgb(235, 219, 178), // light1 #ebdbb2
            accent: Color::Rgb(254, 128, 25),           // bright orange #fe8019
            destructive: Color::Rgb(251, 73, 52),       // bright red #fb4934
            destructive_foreground: Color::Rgb(29, 32, 33), // dark0_hard #1d2021
            warning: Color::Rgb(250, 189, 47),          // bright yellow #fabd2f
            border: Color::Rgb(124, 111, 100),          // dark4 #7c6f64
            ring: dim(primary, background, RING_DIM),
            cursor: Color::Rgb(250, 189, 47), // bright yellow #fabd2f
        }
    }

    /// The Nord palette: cool desaturated blues, from `nordtheme/nord`'s
    /// `src/nord.css`.
    ///
    /// Text is `nord6`/`nord4`, both from Snow Storm, because Nord ships no dim
    /// neutral between its two ramps.
    ///
    /// `surface` is off-palette, the midpoint of `nord0` and `nord1`, which
    /// Nord has no tone between.
    ///
    /// `border` is off-palette: no Nord tone lands in the window a boundary
    /// needs on `nord0`.
    ///
    /// `destructive_foreground` is the light side of `nord11`, the one red Nord
    /// ships, and a documented contrast exception — the light side is the one
    /// that improves as the fill darkens.
    #[must_use]
    pub const fn nord() -> Self {
        let primary = Color::Rgb(136, 192, 208); // nord8 #88c0d0
        let background = Color::Rgb(46, 52, 64); // nord0 #2e3440
        let field = Color::Rgb(59, 66, 82); // nord1 #3b4252
        Self {
            name: "Nord",
            foreground: Color::Rgb(236, 239, 244), // nord6 #eceff4
            muted_foreground: Color::Rgb(216, 222, 233), // nord4 #d8dee9
            background,
            surface: dim(background, field, 50),
            field,
            primary,
            primary_foreground: Color::Rgb(46, 52, 64), // nord0 #2e3440
            secondary: Color::Rgb(76, 86, 106),         // nord3 #4c566a
            secondary_foreground: Color::Rgb(236, 239, 244), // nord6 #eceff4
            accent: Color::Rgb(180, 142, 173),          // nord15 #b48ead
            destructive: Color::Rgb(191, 97, 106),      // nord11 #bf616a
            destructive_foreground: Color::Rgb(236, 239, 244), // nord6 #eceff4
            warning: Color::Rgb(235, 203, 139),         // nord13 #ebcb8b
            // Off-palette: nord3, the palette's own line color, does not clear
            // the boundary floor on nord0, and the next tone up is text.
            border: Color::Rgb(112, 126, 155),
            ring: dim(primary, background, RING_DIM),
            cursor: Color::Rgb(136, 192, 208), // nord8 #88c0d0
        }
    }

    /// The Tokyo Night palette: saturated blues and purples on near-black, from
    /// `folke/tokyonight.nvim`'s `night` variant
    /// (`lua/tokyonight/colors/storm.lua` with the `night.lua` overrides).
    ///
    /// `surface` is off-palette, the midpoint of `bg` and `bg_highlight`: the
    /// variant's own `bg_float` is *darker* than the editor background, and a
    /// dialog would sink into the backdrop it floats over.
    ///
    /// `secondary` is `bg_visual`, the selection fill, rather than
    /// `terminal_black` — the only background tone that keeps a hovered
    /// secondary label readable while still reading as a raised control.
    #[must_use]
    pub const fn tokyo_night() -> Self {
        let primary = Color::Rgb(122, 162, 247); // blue #7aa2f7
        let background = Color::Rgb(26, 27, 38); // bg #1a1b26
        let field = Color::Rgb(41, 46, 66); // bg_highlight #292e42
        Self {
            name: "Tokyo Night",
            foreground: Color::Rgb(192, 202, 245), // fg #c0caf5
            muted_foreground: Color::Rgb(169, 177, 214), // fg_dark #a9b1d6
            background,
            surface: dim(background, field, 50),
            field,
            primary,
            primary_foreground: Color::Rgb(26, 27, 38), // bg #1a1b26
            secondary: Color::Rgb(40, 52, 87),          // bg_visual #283457
            secondary_foreground: Color::Rgb(192, 202, 245), // fg #c0caf5
            accent: Color::Rgb(187, 154, 247),          // magenta #bb9af7
            destructive: Color::Rgb(247, 118, 142),     // red #f7768e
            destructive_foreground: Color::Rgb(26, 27, 38), // bg #1a1b26
            warning: Color::Rgb(224, 175, 104),         // yellow #e0af68
            border: Color::Rgb(115, 122, 162),          // dark5 #737aa2
            ring: dim(primary, background, RING_DIM),
            cursor: Color::Rgb(122, 162, 247), // blue #7aa2f7
        }
    }

    /// The Solarized Dark palette: low-contrast, carefully balanced, from Ethan
    /// Schoonover's `altercation/solarized` README.
    ///
    /// `surface` is off-palette, 30% of the way from `base03` to `base02`,
    /// because the palette ships no third background tone.
    ///
    /// Text is one rung up the ramp from Schoonover's own dark mapping
    /// (`base2`/`base1` where he uses `base1`/`base0`), which leaves no
    /// headroom for the well ladder to spend.
    ///
    /// `primary_foreground` is `base3`, the light-mode background: this blue
    /// takes a documented contrast exception, and `base3` is the side that
    /// improves as the fill darkens.
    #[must_use]
    pub const fn solarized() -> Self {
        let primary = Color::Rgb(38, 139, 210); // blue #268bd2
        let background = Color::Rgb(0, 43, 54); // base03 #002b36
        let field = Color::Rgb(7, 54, 66); // base02 #073642
        Self {
            name: "Solarized",
            foreground: Color::Rgb(238, 232, 213), // base2 #eee8d5
            muted_foreground: Color::Rgb(147, 161, 161), // base1 #93a1a1
            background,
            surface: dim(background, field, 30),
            field,
            primary,
            primary_foreground: Color::Rgb(253, 246, 227), // base3 #fdf6e3
            secondary: Color::Rgb(88, 110, 117),           // base01 #586e75
            secondary_foreground: Color::Rgb(253, 246, 227), // base3 #fdf6e3
            accent: Color::Rgb(211, 54, 130),              // magenta #d33682
            destructive: Color::Rgb(220, 50, 47),          // red #dc322f
            destructive_foreground: Color::Rgb(253, 246, 227), // base3 #fdf6e3
            warning: Color::Rgb(181, 137, 0),              // yellow #b58900
            border: Color::Rgb(101, 123, 131),             // base00 #657b83
            ring: dim(primary, background, RING_DIM),
            cursor: Color::Rgb(42, 161, 152), // cyan #2aa198
        }
    }
}