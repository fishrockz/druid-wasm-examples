pub mod anim;
pub mod calc;
pub mod custom_widget;
pub mod either;
//pub mod ext_event; // No thread support on wasm
pub mod flex;
pub mod game_of_life;
pub mod hello;
pub mod identity;
pub mod image; // Can't load an image from a local file
pub mod layout;
pub mod lens;
pub mod list;
pub mod multiwin;
pub mod panels;
pub mod parse;
pub mod scroll_colors;
pub mod scroll;
pub mod split_demo;
pub mod styled_text;
//pub mod svg; // usvg doesn't compile on usvg at the time of this writing
pub mod switch_demo;
pub mod timer;
pub mod view_switcher;

//TODO:
// Load images
// Open file dialog
// Clipboard
