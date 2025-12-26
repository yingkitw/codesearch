//! Theme Support Module
//!
//! Provides customizable output formatting with multiple themes.

use colored::*;

/// Theme trait for customizable output formatting
#[allow(dead_code)]
pub trait Theme: Send + Sync {
    fn name(&self) -> &'static str;
    fn format_header(&self, text: &str) -> ColoredString;
    fn format_success(&self, text: &str) -> ColoredString;
    fn format_error(&self, text: &str) -> ColoredString;
    fn format_warning(&self, text: &str) -> ColoredString;
    fn format_info(&self, text: &str) -> ColoredString;
    fn format_highlight(&self, text: &str) -> ColoredString;
    fn format_dim(&self, text: &str) -> ColoredString;
    fn format_file_path(&self, text: &str) -> ColoredString;
    fn format_line_number(&self, text: &str) -> ColoredString;
    fn format_match(&self, text: &str) -> ColoredString;
}

/// Default theme with balanced colors
#[derive(Clone, Copy)]
pub struct DefaultTheme;

impl Theme for DefaultTheme {
    fn name(&self) -> &'static str { "default" }
    fn format_header(&self, text: &str) -> ColoredString { text.bold().cyan() }
    fn format_success(&self, text: &str) -> ColoredString { text.green() }
    fn format_error(&self, text: &str) -> ColoredString { text.red() }
    fn format_warning(&self, text: &str) -> ColoredString { text.yellow() }
    fn format_info(&self, text: &str) -> ColoredString { text.blue() }
    fn format_highlight(&self, text: &str) -> ColoredString { text.bold() }
    fn format_dim(&self, text: &str) -> ColoredString { text.dimmed() }
    fn format_file_path(&self, text: &str) -> ColoredString { text.cyan() }
    fn format_line_number(&self, text: &str) -> ColoredString { text.yellow() }
    fn format_match(&self, text: &str) -> ColoredString { text.red().bold() }
}

/// Dark theme optimized for dark terminals
#[derive(Clone, Copy)]
pub struct DarkTheme;

impl Theme for DarkTheme {
    fn name(&self) -> &'static str { "dark" }
    fn format_header(&self, text: &str) -> ColoredString { text.bold().bright_cyan() }
    fn format_success(&self, text: &str) -> ColoredString { text.bright_green() }
    fn format_error(&self, text: &str) -> ColoredString { text.bright_red() }
    fn format_warning(&self, text: &str) -> ColoredString { text.bright_yellow() }
    fn format_info(&self, text: &str) -> ColoredString { text.bright_blue() }
    fn format_highlight(&self, text: &str) -> ColoredString { text.bold().white() }
    fn format_dim(&self, text: &str) -> ColoredString { text.bright_black() }
    fn format_file_path(&self, text: &str) -> ColoredString { text.bright_magenta() }
    fn format_line_number(&self, text: &str) -> ColoredString { text.bright_cyan() }
    fn format_match(&self, text: &str) -> ColoredString { text.bright_yellow().bold() }
}

/// Light theme optimized for light terminals
#[derive(Clone, Copy)]
pub struct LightTheme;

impl Theme for LightTheme {
    fn name(&self) -> &'static str { "light" }
    fn format_header(&self, text: &str) -> ColoredString { text.bold().blue() }
    fn format_success(&self, text: &str) -> ColoredString { text.green() }
    fn format_error(&self, text: &str) -> ColoredString { text.red() }
    fn format_warning(&self, text: &str) -> ColoredString { text.truecolor(200, 100, 0) }
    fn format_info(&self, text: &str) -> ColoredString { text.blue() }
    fn format_highlight(&self, text: &str) -> ColoredString { text.bold().black() }
    fn format_dim(&self, text: &str) -> ColoredString { text.truecolor(128, 128, 128) }
    fn format_file_path(&self, text: &str) -> ColoredString { text.purple() }
    fn format_line_number(&self, text: &str) -> ColoredString { text.blue() }
    fn format_match(&self, text: &str) -> ColoredString { text.red().bold().underline() }
}

/// Monochrome theme for minimal output
#[derive(Clone, Copy)]
pub struct MonoTheme;

impl Theme for MonoTheme {
    fn name(&self) -> &'static str { "mono" }
    fn format_header(&self, text: &str) -> ColoredString { text.bold() }
    fn format_success(&self, text: &str) -> ColoredString { text.normal() }
    fn format_error(&self, text: &str) -> ColoredString { text.bold() }
    fn format_warning(&self, text: &str) -> ColoredString { text.italic() }
    fn format_info(&self, text: &str) -> ColoredString { text.normal() }
    fn format_highlight(&self, text: &str) -> ColoredString { text.bold() }
    fn format_dim(&self, text: &str) -> ColoredString { text.dimmed() }
    fn format_file_path(&self, text: &str) -> ColoredString { text.underline() }
    fn format_line_number(&self, text: &str) -> ColoredString { text.dimmed() }
    fn format_match(&self, text: &str) -> ColoredString { text.bold().underline() }
}

/// Ocean theme with blue tones
#[derive(Clone, Copy)]
pub struct OceanTheme;

impl Theme for OceanTheme {
    fn name(&self) -> &'static str { "ocean" }
    fn format_header(&self, text: &str) -> ColoredString { text.bold().truecolor(0, 191, 255) }
    fn format_success(&self, text: &str) -> ColoredString { text.truecolor(64, 224, 208) }
    fn format_error(&self, text: &str) -> ColoredString { text.truecolor(255, 99, 71) }
    fn format_warning(&self, text: &str) -> ColoredString { text.truecolor(255, 215, 0) }
    fn format_info(&self, text: &str) -> ColoredString { text.truecolor(100, 149, 237) }
    fn format_highlight(&self, text: &str) -> ColoredString { text.bold().truecolor(255, 255, 255) }
    fn format_dim(&self, text: &str) -> ColoredString { text.truecolor(119, 136, 153) }
    fn format_file_path(&self, text: &str) -> ColoredString { text.truecolor(0, 206, 209) }
    fn format_line_number(&self, text: &str) -> ColoredString { text.truecolor(135, 206, 250) }
    fn format_match(&self, text: &str) -> ColoredString { text.truecolor(50, 205, 50).bold() }
}

/// Forest theme with green tones
#[derive(Clone, Copy)]
pub struct ForestTheme;

impl Theme for ForestTheme {
    fn name(&self) -> &'static str { "forest" }
    fn format_header(&self, text: &str) -> ColoredString { text.bold().truecolor(34, 139, 34) }
    fn format_success(&self, text: &str) -> ColoredString { text.truecolor(50, 205, 50) }
    fn format_error(&self, text: &str) -> ColoredString { text.truecolor(220, 20, 60) }
    fn format_warning(&self, text: &str) -> ColoredString { text.truecolor(218, 165, 32) }
    fn format_info(&self, text: &str) -> ColoredString { text.truecolor(107, 142, 35) }
    fn format_highlight(&self, text: &str) -> ColoredString { text.bold().truecolor(240, 255, 240) }
    fn format_dim(&self, text: &str) -> ColoredString { text.truecolor(85, 107, 47) }
    fn format_file_path(&self, text: &str) -> ColoredString { text.truecolor(143, 188, 143) }
    fn format_line_number(&self, text: &str) -> ColoredString { text.truecolor(144, 238, 144) }
    fn format_match(&self, text: &str) -> ColoredString { text.truecolor(255, 215, 0).bold() }
}

/// Get theme by name
#[allow(dead_code)]
pub fn get_theme(name: &str) -> Box<dyn Theme> {
    match name.to_lowercase().as_str() {
        "dark" => Box::new(DarkTheme),
        "light" => Box::new(LightTheme),
        "mono" | "monochrome" => Box::new(MonoTheme),
        "ocean" => Box::new(OceanTheme),
        "forest" => Box::new(ForestTheme),
        _ => Box::new(DefaultTheme),
    }
}

/// List all available theme names
#[allow(dead_code)]
pub fn list_theme_names() -> Vec<&'static str> {
    vec!["default", "dark", "light", "mono", "ocean", "forest"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = DefaultTheme;
        assert_eq!(theme.name(), "default");
        let _ = theme.format_header("test");
        let _ = theme.format_success("test");
        let _ = theme.format_error("test");
    }

    #[test]
    fn test_get_theme() {
        assert_eq!(get_theme("default").name(), "default");
        assert_eq!(get_theme("dark").name(), "dark");
        assert_eq!(get_theme("light").name(), "light");
        assert_eq!(get_theme("mono").name(), "mono");
        assert_eq!(get_theme("ocean").name(), "ocean");
        assert_eq!(get_theme("forest").name(), "forest");
        assert_eq!(get_theme("unknown").name(), "default");
    }

    #[test]
    fn test_list_theme_names() {
        let names = list_theme_names();
        assert!(names.contains(&"default"));
        assert!(names.contains(&"dark"));
        assert!(names.len() >= 6);
    }
}

