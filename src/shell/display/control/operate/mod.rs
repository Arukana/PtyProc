pub mod bold;
pub mod dim;
pub mod underlined;
pub mod blink;
pub mod reverse;
pub mod hidden;
pub mod foreground;
pub mod background;

use self::bold::Bold;
use self::dim::Dim;
use self::underlined::Underlined;
use self::blink::Blink;
use self::reverse::Reverse;
use self::hidden::Hidden;
use self::foreground::Foreground;
use self::background::Background;

#[derive(Clone, Copy, Debug)]
pub struct Operate {
    bold: Option<Bold>,
    dim: Option<Dim>,
    underlined: Option<Underlined>,
    blink: Option<Blink>,
    reserve: Option<Reverse>,
    hidden: Option<Hidden>,
    foreground: Option<Foreground>,
    background: Option<Background>,
}

impl Default for Operate {
    fn default() -> Operate {
        Operate {
            bold: None,
            dim: None,
            underlined: None,
            blink: None,
            reserve: None,
            hidden: None,
            foreground: None,
            background: None,
        }
    }
}
