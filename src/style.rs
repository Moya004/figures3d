use crossterm::style::Color;

pub static COLOR_MAP: [(&str, Color); 19] = [
	("black", Color::Black),
	("red", Color::Red),
	("darkred", Color::DarkRed),
	("green", Color::Green),
	("darkgreen", Color::DarkGreen),
	("yellow", Color::Yellow),
	("darkyellow", Color::DarkYellow),
	("blue", Color::Blue),
	("darkblue", Color::DarkBlue),
	("magenta", Color::Magenta),
	("darkmagenta", Color::DarkMagenta),
	("cyan", Color::Cyan),
	("darkcyan", Color::DarkCyan),
	("white", Color::White),
	("grey", Color::Grey),
	("gray", Color::Grey),
	("darkgrey", Color::DarkGrey),
	("darkgray", Color::DarkGrey),
	("reset", Color::Reset),
];

pub fn parse_color(input: &str) -> Option<Color> {
	let key = input.trim().to_lowercase();
	COLOR_MAP.iter().find(|(name, _)| *name == key).map(|(_, color)| *color)
}
