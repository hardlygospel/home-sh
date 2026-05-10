use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub id: &'static str,
    pub name: &'static str,
    pub bg: Color,
    pub fg: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub dim: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub border: Color,
}

impl Theme {
    const fn new(
        id: &'static str,
        name: &'static str,
        bg: Color,
        fg: Color,
        primary: Color,
        secondary: Color,
        accent: Color,
        dim: Color,
        success: Color,
        warning: Color,
        error: Color,
        border: Color,
    ) -> Self {
        Theme { id, name, bg, fg, primary, secondary, accent, dim, success, warning, error, border }
    }
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        Color::Rgb($r, $g, $b)
    };
}

pub static ALL_THEMES: &[Theme] = &[
    // 0: Late
    Theme::new("late", "Late", rgb!(15,15,20), rgb!(220,220,230), rgb!(130,170,255), rgb!(80,120,200), rgb!(255,180,80), rgb!(80,80,100), rgb!(100,220,120), rgb!(255,200,80), rgb!(220,80,80), rgb!(40,40,60)),
    // 1: Contrast
    Theme::new("contrast", "Contrast", rgb!(0,0,0), rgb!(255,255,255), rgb!(255,255,0), rgb!(200,200,0), rgb!(0,255,255), rgb!(100,100,100), rgb!(0,255,0), rgb!(255,165,0), rgb!(255,0,0), rgb!(80,80,80)),
    // 2: Purple
    Theme::new("purple", "Purple", rgb!(20,10,35), rgb!(220,210,240), rgb!(180,100,255), rgb!(140,70,220), rgb!(255,100,180), rgb!(70,50,100), rgb!(120,220,150), rgb!(255,200,80), rgb!(220,80,80), rgb!(50,30,80)),
    // 3: Catppuccin Mocha
    Theme::new("catppuccin-mocha", "Catppuccin Mocha", rgb!(30,30,46), rgb!(205,214,244), rgb!(137,180,250), rgb!(116,199,236), rgb!(245,194,231), rgb!(88,91,112), rgb!(166,227,161), rgb!(249,226,175), rgb!(243,139,168), rgb!(54,58,79)),
    // 4: Catppuccin Macchiato
    Theme::new("catppuccin-macchiato", "Catppuccin Macchiato", rgb!(24,25,38), rgb!(202,211,245), rgb!(138,173,244), rgb!(125,196,228), rgb!(240,198,198), rgb!(91,96,120), rgb!(166,218,149), rgb!(238,212,159), rgb!(237,135,150), rgb!(36,39,58)),
    // 5: Catppuccin Frappe
    Theme::new("catppuccin-frappe", "Catppuccin Frappe", rgb!(48,52,70), rgb!(198,208,245), rgb!(140,170,238), rgb!(153,209,219), rgb!(242,213,207), rgb!(99,109,134), rgb!(166,209,137), rgb!(229,200,144), rgb!(231,130,132), rgb!(41,44,60)),
    // 6: Catppuccin Latte
    Theme::new("catppuccin-latte", "Catppuccin Latte", rgb!(239,241,245), rgb!(76,79,105), rgb!(30,102,245), rgb!(4,165,229), rgb!(234,118,203), rgb!(172,176,190), rgb!(64,160,43), rgb!(223,142,29), rgb!(210,15,57), rgb!(188,192,204)),
    // 7: Gruvbox
    Theme::new("gruvbox", "Gruvbox", rgb!(40,40,40), rgb!(235,219,178), rgb!(250,189,47), rgb!(184,187,38), rgb!(254,128,25), rgb!(102,92,84), rgb!(184,187,38), rgb!(250,189,47), rgb!(251,73,52), rgb!(80,73,69)),
    // 8: OneDarkPro
    Theme::new("onedark", "One Dark Pro", rgb!(40,44,52), rgb!(171,178,191), rgb!(97,175,239), rgb!(86,182,194), rgb!(198,120,221), rgb!(92,99,112), rgb!(152,195,121), rgb!(229,192,123), rgb!(224,108,117), rgb!(59,64,72)),
    // 9: Rose Pine
    Theme::new("rose-pine", "Rose Pine", rgb!(25,23,36), rgb!(224,222,244), rgb!(196,167,231), rgb!(235,188,186), rgb!(246,193,119), rgb!(87,82,121), rgb!(156,207,216), rgb!(246,193,119), rgb!(235,111,146), rgb!(38,35,58)),
    // 10: TokyoNight
    Theme::new("tokyo-night", "Tokyo Night", rgb!(26,27,38), rgb!(169,177,214), rgb!(122,162,247), rgb!(42,195,222), rgb!(187,154,247), rgb!(86,95,137), rgb!(158,206,106), rgb!(224,175,104), rgb!(247,118,142), rgb!(36,40,59)),
    // 11: Kanagawa
    Theme::new("kanagawa", "Kanagawa", rgb!(31,31,40), rgb!(220,215,186), rgb!(127,180,202), rgb!(149,190,162), rgb!(210,126,153), rgb!(84,84,109), rgb!(118,148,106), rgb!(192,156,111), rgb!(196,98,98), rgb!(54,54,68)),
    // 12: Dracula
    Theme::new("dracula", "Dracula", rgb!(40,42,54), rgb!(248,248,242), rgb!(139,233,253), rgb!(80,250,123), rgb!(255,121,198), rgb!(98,114,164), rgb!(80,250,123), rgb!(241,250,140), rgb!(255,85,85), rgb!(68,71,90)),
    // 13: Oxocarbon
    Theme::new("oxocarbon", "Oxocarbon", rgb!(18,18,18), rgb!(244,244,244), rgb!(120,165,255), rgb!(88,165,165), rgb!(200,130,255), rgb!(80,80,80), rgb!(65,200,100), rgb!(250,200,80), rgb!(250,100,80), rgb!(40,40,40)),
    // 14: CopperFresh
    Theme::new("copper-fresh", "CopperFresh", rgb!(15,20,15), rgb!(200,220,190), rgb!(184,115,51), rgb!(150,90,40), rgb!(100,200,100), rgb!(60,70,60), rgb!(100,200,100), rgb!(255,200,80), rgb!(220,80,80), rgb!(30,40,30)),
    // 15: ExposedCopper
    Theme::new("exposed-copper", "ExposedCopper", rgb!(30,15,10), rgb!(230,210,190), rgb!(205,127,50), rgb!(180,100,40), rgb!(255,180,100), rgb!(80,50,40), rgb!(100,200,100), rgb!(255,200,80), rgb!(220,80,80), rgb!(50,30,20)),
    // 16: WeatheredCopper
    Theme::new("weathered-copper", "WeatheredCopper", rgb!(18,30,28), rgb!(190,220,210), rgb!(100,180,160), rgb!(80,140,130), rgb!(205,127,50), rgb!(50,80,75), rgb!(80,200,120), rgb!(255,200,80), rgb!(220,80,80), rgb!(30,50,48)),
    // 17: OxidizedCopper
    Theme::new("oxidized-copper", "OxidizedCopper", rgb!(10,25,20), rgb!(180,230,210), rgb!(68,163,145), rgb!(50,130,115), rgb!(180,100,40), rgb!(40,70,65), rgb!(80,200,120), rgb!(255,200,80), rgb!(220,80,80), rgb!(20,45,40)),
    // 18: MARATHON Arachne
    Theme::new("marathon-arachne", "MARATHON Arachne", rgb!(10,5,20), rgb!(200,190,220), rgb!(150,80,255), rgb!(100,50,200), rgb!(255,80,150), rgb!(50,40,80), rgb!(80,200,120), rgb!(255,200,80), rgb!(220,80,80), rgb!(25,15,45)),
    // 19: MARATHON CyberAcme
    Theme::new("marathon-cyberacme", "MARATHON CyberAcme", rgb!(5,10,20), rgb!(180,210,240), rgb!(0,200,255), rgb!(0,150,200), rgb!(255,200,0), rgb!(30,50,80), rgb!(0,255,150), rgb!(255,220,0), rgb!(255,50,50), rgb!(15,25,45)),
    // 20: MARATHON NuCaloric
    Theme::new("marathon-nucaloric", "MARATHON NuCaloric", rgb!(20,15,5), rgb!(240,230,190), rgb!(255,180,0), rgb!(220,140,0), rgb!(200,100,255), rgb!(80,65,30), rgb!(150,255,100), rgb!(255,210,50), rgb!(255,80,80), rgb!(45,35,10)),
    // 21: MARATHON Sekiguchi
    Theme::new("marathon-sekiguchi", "MARATHON Sekiguchi", rgb!(20,5,5), rgb!(240,200,200), rgb!(255,80,80), rgb!(200,50,50), rgb!(80,200,255), rgb!(80,30,30), rgb!(100,255,150), rgb!(255,200,80), rgb!(255,50,50), rgb!(45,10,10)),
    // 22: MARATHON Traxus
    Theme::new("marathon-traxus", "MARATHON Traxus", rgb!(5,15,5), rgb!(190,230,190), rgb!(0,255,80), rgb!(0,200,60), rgb!(255,100,0), rgb!(25,60,25), rgb!(0,255,120), rgb!(255,220,0), rgb!(220,80,80), rgb!(10,35,10)),
    // 23: MARATHON Mida
    Theme::new("marathon-mida", "MARATHON Mida", rgb!(5,5,25), rgb!(190,190,240), rgb!(80,100,255), rgb!(60,70,200), rgb!(255,150,0), rgb!(25,25,80), rgb!(100,255,200), rgb!(255,220,80), rgb!(255,80,80), rgb!(10,10,50)),
    // 24: ENA
    Theme::new("ena", "ENA", rgb!(30,25,15), rgb!(235,225,190), rgb!(200,160,80), rgb!(160,120,50), rgb!(100,180,255), rgb!(80,70,40), rgb!(120,220,100), rgb!(255,200,80), rgb!(220,80,80), rgb!(55,45,25)),
    // 25: ENADreamBbq
    Theme::new("ena-dream-bbq", "ENADreamBbq", rgb!(25,10,10), rgb!(240,210,195), rgb!(255,120,80), rgb!(200,90,60), rgb!(255,220,100), rgb!(80,40,35), rgb!(120,220,100), rgb!(255,200,80), rgb!(220,80,80), rgb!(50,20,18)),
    // 26: Kirii
    Theme::new("kirii", "Kirii", rgb!(240,235,255), rgb!(50,40,80), rgb!(120,80,220), rgb!(90,60,180), rgb!(255,120,180), rgb!(160,150,200), rgb!(60,180,100), rgb!(200,150,50), rgb!(220,60,80), rgb!(200,195,230)),
    // 27: Discord Dark
    Theme::new("discord-dark", "Discord Dark", rgb!(54,57,63), rgb!(220,221,222), rgb!(88,101,242), rgb!(71,82,196), rgb!(235,69,158), rgb!(114,118,125), rgb!(87,242,135), rgb!(250,168,26), rgb!(237,66,69), rgb!(79,84,92)),
    // 28: Discord Light
    Theme::new("discord-light", "Discord Light", rgb!(255,255,255), rgb!(35,39,42), rgb!(88,101,242), rgb!(71,82,196), rgb!(235,69,158), rgb!(142,146,151), rgb!(87,242,135), rgb!(250,168,26), rgb!(237,66,69), rgb!(210,215,220)),
    // 29: Facebook Dark
    Theme::new("facebook-dark", "Facebook Dark", rgb!(24,25,26), rgb!(225,227,229), rgb!(24,119,242), rgb!(16,92,196), rgb!(66,183,221), rgb!(70,73,75), rgb!(67,183,99), rgb!(255,200,0), rgb!(250,50,50), rgb!(48,49,50)),
    // 30: Facebook Light
    Theme::new("facebook-light", "Facebook Light", rgb!(240,242,245), rgb!(5,5,5), rgb!(24,119,242), rgb!(16,92,196), rgb!(66,183,221), rgb!(150,153,158), rgb!(67,183,99), rgb!(255,200,0), rgb!(250,50,50), rgb!(200,204,209)),
    // 31: Twitter Dark
    Theme::new("twitter-dark", "Twitter Dark", rgb!(21,32,43), rgb!(255,255,255), rgb!(29,161,242), rgb!(20,130,200), rgb!(94,179,240), rgb!(71,89,103), rgb!(23,191,99), rgb!(255,173,31), rgb!(224,36,94), rgb!(38,55,68)),
    // 32: Twitter Light
    Theme::new("twitter-light", "Twitter Light", rgb!(255,255,255), rgb!(15,20,25), rgb!(29,161,242), rgb!(20,130,200), rgb!(94,179,240), rgb!(140,152,164), rgb!(23,191,99), rgb!(255,173,31), rgb!(224,36,94), rgb!(200,207,214)),
    // 33: Telegram Dark
    Theme::new("telegram-dark", "Telegram Dark", rgb!(23,33,43), rgb!(255,255,255), rgb!(42,174,245), rgb!(33,145,205), rgb!(100,200,255), rgb!(64,83,100), rgb!(78,208,100), rgb!(255,200,50), rgb!(255,80,80), rgb!(36,52,67)),
    // 34: Telegram Light
    Theme::new("telegram-light", "Telegram Light", rgb!(255,255,255), rgb!(0,0,0), rgb!(42,174,245), rgb!(33,145,205), rgb!(100,200,255), rgb!(130,150,170), rgb!(78,208,100), rgb!(255,200,50), rgb!(255,80,80), rgb!(200,215,228)),
    // 35: GitHub Dark
    Theme::new("github-dark", "GitHub Dark", rgb!(13,17,23), rgb!(230,237,243), rgb!(88,166,255), rgb!(58,129,200), rgb!(210,153,34), rgb!(48,54,61), rgb!(63,185,80), rgb!(210,153,34), rgb!(248,81,73), rgb!(33,38,45)),
    // 36: GitHub Light
    Theme::new("github-light", "GitHub Light", rgb!(255,255,255), rgb!(36,41,47), rgb!(9,105,218), rgb!(0,75,160), rgb!(130,80,0), rgb!(140,149,159), rgb!(26,127,55), rgb!(154,103,0), rgb!(207,34,46), rgb!(208,215,222)),
    // 37: Matcha
    Theme::new("matcha", "Matcha", rgb!(20,30,20), rgb!(200,230,200), rgb!(80,180,80), rgb!(60,140,60), rgb!(180,230,100), rgb!(50,80,50), rgb!(100,220,100), rgb!(220,200,80), rgb!(200,80,80), rgb!(35,55,35)),
    // 38: EarlGrey
    Theme::new("earl-grey", "EarlGrey", rgb!(45,42,38), rgb!(220,215,205), rgb!(180,160,130), rgb!(150,130,100), rgb!(200,180,150), rgb!(90,85,78), rgb!(130,180,100), rgb!(210,190,100), rgb!(200,90,90), rgb!(65,62,58)),
    // 39: Hibiscus
    Theme::new("hibiscus", "Hibiscus", rgb!(30,15,20), rgb!(240,210,220), rgb!(220,80,120), rgb!(180,60,100), rgb!(255,180,200), rgb!(80,45,55), rgb!(100,200,120), rgb!(255,200,80), rgb!(220,60,80), rgb!(55,30,38)),
    // 40: Oolong
    Theme::new("oolong", "Oolong", rgb!(28,24,20), rgb!(220,210,195), rgb!(160,130,90), rgb!(130,100,65), rgb!(200,170,120), rgb!(75,65,55), rgb!(110,180,100), rgb!(210,180,90), rgb!(200,90,80), rgb!(50,42,36)),
    // 41: SilverNeedle
    Theme::new("silver-needle", "SilverNeedle", rgb!(235,238,240), rgb!(40,45,50), rgb!(80,120,160), rgb!(60,95,130), rgb!(100,160,200), rgb!(150,160,170), rgb!(70,150,80), rgb!(170,140,60), rgb!(190,60,60), rgb!(190,198,205)),
    // 42: CRT Amber
    Theme::new("crt-amber", "CRT Amber", rgb!(15,10,0), rgb!(255,180,0), rgb!(255,200,50), rgb!(220,160,0), rgb!(255,220,100), rgb!(80,55,0), rgb!(200,220,0), rgb!(255,200,50), rgb!(255,80,0), rgb!(30,20,0)),
    // 43: CRT Green
    Theme::new("crt-green", "CRT Green", rgb!(0,15,0), rgb!(0,255,0), rgb!(50,255,50), rgb!(0,200,0), rgb!(100,255,100), rgb!(0,80,0), rgb!(0,255,100), rgb!(200,255,0), rgb!(255,80,0), rgb!(0,30,0)),
    // 44: CRT Cyan
    Theme::new("crt-cyan", "CRT Cyan", rgb!(0,10,15), rgb!(0,220,255), rgb!(50,230,255), rgb!(0,180,220), rgb!(100,240,255), rgb!(0,60,80), rgb!(0,255,180), rgb!(200,255,0), rgb!(255,80,80), rgb!(0,20,30)),
    // 45: CRT C64
    Theme::new("crt-c64", "CRT C64", rgb!(64,64,185), rgb!(162,162,255), rgb!(140,140,255), rgb!(100,100,200), rgb!(255,120,120), rgb!(80,80,140), rgb!(100,255,100), rgb!(255,255,100), rgb!(255,80,80), rgb!(80,80,140)),
    // 46: CRT Blood
    Theme::new("crt-blood", "CRT Blood", rgb!(15,0,0), rgb!(220,0,0), rgb!(255,50,50), rgb!(200,0,0), rgb!(255,100,100), rgb!(80,0,0), rgb!(180,100,0), rgb!(255,180,0), rgb!(255,50,50), rgb!(30,0,0)),
    // 47: CRT AppleII
    Theme::new("crt-apple2", "CRT AppleII", rgb!(0,0,0), rgb!(255,110,0), rgb!(255,140,30), rgb!(220,90,0), rgb!(255,180,80), rgb!(80,40,0), rgb!(180,220,0), rgb!(255,200,0), rgb!(255,50,50), rgb!(20,10,0)),
    // 48: CRT Plasma
    Theme::new("crt-plasma", "CRT Plasma", rgb!(10,0,20), rgb!(200,0,255), rgb!(220,50,255), rgb!(180,0,220), rgb!(255,100,255), rgb!(60,0,80), rgb!(100,255,200), rgb!(255,200,80), rgb!(255,80,80), rgb!(20,0,40)),
    // 49: CRT Paper
    Theme::new("crt-paper", "CRT Paper", rgb!(240,235,220), rgb!(30,25,15), rgb!(60,50,30), rgb!(80,70,50), rgb!(40,80,60), rgb!(160,155,140), rgb!(50,120,60), rgb!(160,120,20), rgb!(180,50,40), rgb!(200,196,184)),
    // 50: CRT Mauve
    Theme::new("crt-mauve", "CRT Mauve", rgb!(20,10,25), rgb!(220,180,240), rgb!(180,100,240), rgb!(150,70,210), rgb!(240,150,255), rgb!(70,45,80), rgb!(120,220,150), rgb!(255,200,80), rgb!(220,80,80), rgb!(40,20,50)),
    // 51: CRT PipboyMojave
    Theme::new("crt-pipboy", "CRT PipboyMojave", rgb!(5,15,5), rgb!(100,200,50), rgb!(120,220,70), rgb!(90,180,40), rgb!(160,255,100), rgb!(30,60,20), rgb!(80,255,80), rgb!(220,200,0), rgb!(255,80,0), rgb!(10,30,10)),
    // 52: Nordic
    Theme::new("nordic", "Nordic", rgb!(36,39,49), rgb!(216,222,233), rgb!(129,161,193), rgb!(94,129,172), rgb!(191,97,106), rgb!(76,86,106), rgb!(163,190,140), rgb!(235,203,139), rgb!(191,97,106), rgb!(59,66,82)),
    // 53: Bamboo
    Theme::new("bamboo", "Bamboo", rgb!(20,28,18), rgb!(195,225,185), rgb!(100,180,70), rgb!(75,150,50), rgb!(200,180,80), rgb!(55,75,48), rgb!(100,220,100), rgb!(220,200,80), rgb!(200,80,80), rgb!(35,50,30)),
    // 54: Spring
    Theme::new("spring", "Spring", rgb!(245,245,255), rgb!(60,60,90), rgb!(180,140,220), rgb!(140,100,180), rgb!(255,180,200), rgb!(180,170,210), rgb!(100,200,130), rgb!(220,200,80), rgb!(220,80,80), rgb!(200,198,225)),
    // 55: Summer
    Theme::new("summer", "Summer", rgb!(255,250,230), rgb!(60,50,20), rgb!(255,180,0), rgb!(220,140,0), rgb!(255,100,50), rgb!(190,180,150), rgb!(80,200,100), rgb!(255,200,50), rgb!(220,80,80), rgb!(210,205,180)),
    // 56: Autumn
    Theme::new("autumn", "Autumn", rgb!(35,20,10), rgb!(240,215,185), rgb!(210,120,50), rgb!(170,90,30), rgb!(190,60,40), rgb!(90,60,35), rgb!(120,180,80), rgb!(220,180,60), rgb!(200,70,50), rgb!(60,38,20)),
    // 57: Winter
    Theme::new("winter", "Winter", rgb!(230,240,255), rgb!(30,40,80), rgb!(80,120,200), rgb!(60,95,170), rgb!(150,200,255), rgb!(160,175,210), rgb!(80,180,120), rgb!(200,180,80), rgb!(200,80,80), rgb!(190,200,230)),
    // 58: Cyberpunk2077
    Theme::new("cyberpunk", "Cyberpunk 2077", rgb!(10,5,20), rgb!(240,220,50), rgb!(240,220,50), rgb!(200,180,0), rgb!(255,0,200), rgb!(60,50,20), rgb!(0,255,200), rgb!(255,220,0), rgb!(255,50,50), rgb!(25,15,50)),
    // 59: Monokai
    Theme::new("monokai", "Monokai", rgb!(39,40,34), rgb!(248,248,242), rgb!(102,217,239), rgb!(78,201,176), rgb!(249,38,114), rgb!(117,113,94), rgb!(166,226,46), rgb!(230,219,116), rgb!(249,38,114), rgb!(62,64,56)),
    // 60: Late (alias for slot 60)
    Theme::new("late-60", "Late (alt)", rgb!(12,12,18), rgb!(215,215,228), rgb!(120,165,255), rgb!(75,115,195), rgb!(250,175,75), rgb!(75,75,98), rgb!(95,215,115), rgb!(250,195,75), rgb!(215,75,75), rgb!(35,35,55)),
    // 61: Phantom
    Theme::new("phantom", "Phantom", rgb!(15,15,30), rgb!(200,200,230), rgb!(100,100,255), rgb!(70,70,200), rgb!(200,100,255), rgb!(55,55,85), rgb!(80,220,150), rgb!(240,200,80), rgb!(220,80,100), rgb!(30,30,60)),
    // 62: Slate
    Theme::new("slate", "Slate", rgb!(28,32,40), rgb!(195,205,220), rgb!(100,140,200), rgb!(75,110,170), rgb!(160,200,240), rgb!(70,80,100), rgb!(100,200,130), rgb!(200,180,80), rgb!(210,90,90), rgb!(45,52,65)),
];

pub fn theme_index_by_id(id: &str) -> usize {
    ALL_THEMES.iter().position(|t| t.id == id).unwrap_or(0)
}

pub fn get_theme(id: &str) -> &'static Theme {
    let idx = theme_index_by_id(id);
    &ALL_THEMES[idx]
}
