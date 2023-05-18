/*!
# LoadingBar
ANSI terminal progress bars. Bars fill all the available space.

## Usage
```
use loadingbar::Bar;
let mut bar = Bar::new(0.5, false, None);
println!("{}", bar);
bar.progress = 41.0 / 82.0;
println!("{}", bar);
```

## Shrinking
The bar is built by adding components until there is no space left for them.
The minimum size for a bar is 5, which is enough space for `[100%]`.
*/

extern crate termsize;
use std::fmt;

/// Text-Incomplete
const TI: &str = "\u{27F3} ";
/// Text-Complete
const TC: &str = "\u{2713} ";
/// Cap-Left
const CL: &str = "[";
/// Cap-Right
const CR: &str = "]";
/// Progress-Incomplete
const PI: &str = "\u{2592}";
/// Progress-Complete
const PC: &str = "\u{2588}";
/// Line-End
const LE: &str = "\u{001b}[1F";
/// Right-to-left modifier
const RTL: bool = false;
/// Default initial progress
const PROGRESS: f32 = 0.0;

const DEFAULT_WIDTH: u16 = 80;
const WIDTH: Option<usize> = Some(DEFAULT_WIDTH as usize);
const MIN_WIDTH: usize = 7;

/// The only export from loadingbar, implements the fmt::Display trait.
pub struct Bar {
    /// A number between 0 and 1
    pub progress: f32,
    /// Right-to-left modifier
    pub rtl: bool,
    /// Manually set the available space, set to None for a dynamic bar
    pub width: Option<usize>,
}

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // work out the size we have to work with
        let size: usize = match self.width {
            // the programmer set the size
            Some(size) => size,
            // we need to calculate it dynamically
            None => match termsize::get()
                .unwrap_or(termsize::Size {
                    rows: 0,
                    cols: DEFAULT_WIDTH,
                })
                .cols as usize
            {
                0..=MIN_WIDTH => MIN_WIDTH,
                size => size,
            },
        };

        // the smallest number of components is 4 -> '[', '50%', ']', LE
        let mut components: Vec<&str> = Vec::with_capacity(4);

        let percent = format!("{}%", ((self.progress * 100.0).floor() as usize));
        let indicator = match &percent as &str {
            "100%" => TC,
            _ => TI,
        };
        components.push(indicator);

        // The won't exceed the max size, so we avoid allocations
        let mut progress: Vec<&str> = Vec::with_capacity(size);

        progress.push(CL);

        if size == 5 {
            progress.push(&percent);
        } else {
            let c = ((size - 4) as f32 * self.progress).floor() as usize;
            let i = (size - 4) - c;

            for _ in 0..c {
                progress.push(PC);
            }
            for _ in 0..i {
                progress.push(PI);
            }
        }

        progress.push(CR);

        // We have to reverse the bar twice to get it to appear normally on RTL
        if self.rtl {
            progress.reverse();
        }
        components.append(&mut progress);

        if self.rtl {
            components.reverse();
        }

        // Line ender always goes at the end
        components.push(LE);

        write!(f, "{}", components.join(""))
    }
}

impl Bar {
    pub fn new(progress: f32, rtl: bool, width: Option<usize>) -> Bar {
        Bar {
            progress,
            rtl,
            width,
        }
    }
}

impl From<bool> for Bar {
    fn from(rtl: bool) -> Bar {
        Bar {
            progress: PROGRESS,
            rtl,
            width: WIDTH,
        }
    }
}

impl From<f32> for Bar {
    fn from(progress: f32) -> Bar {
        Bar {
            progress,
            rtl: RTL,
            width: WIDTH,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ltr_40_i() {
        let bar = Bar::new(0.5, false, Some(40));
        assert_eq!(
            format!("{}", bar),
            "⟳ [██████████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒]\u{001b}[1F"
        )
    }

    #[test]
    fn new_rtl_40_i() {
        let bar = Bar::new(0.5, true, Some(40));
        assert_eq!(
            format!("{}", bar),
            "[██████████████████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒]⟳ \u{001b}[1F"
        )
    }

    #[test]
    fn new_ltr_5_i() {
        let bar = Bar::new(0.8, false, Some(5));
        assert_eq!(format!("{}", bar), "⟳ [80%]\u{001b}[1F")
    }

    #[test]
    fn new_rtl_5_i() {
        let bar = Bar::new(0.8, true, Some(5));
        assert_eq!(format!("{}", bar), "[80%]⟳ \u{001b}[1F")
    }

    #[test]
    fn new_ltr_40_c() {
        let bar = Bar::new(1.0, false, Some(40));
        assert_eq!(
            format!("{}", bar),
            "✓ [████████████████████████████████████]\u{001b}[1F"
        )
    }

    #[test]
    fn new_rtl_40_c() {
        let bar = Bar::new(1.0, true, Some(40));
        assert_eq!(
            format!("{}", bar),
            "[████████████████████████████████████]✓ \u{001b}[1F"
        )
    }

    #[test]
    fn new_ltr_5_c() {
        let bar = Bar::new(1.0, false, Some(5));
        assert_eq!(format!("{}", bar), "✓ [100%]\u{001b}[1F")
    }

    #[test]
    fn new_rtl_5_c() {
        let bar = Bar::new(1.0, true, Some(5));
        assert_eq!(format!("{}", bar), "[100%]✓ \u{001b}[1F")
    }

    #[test]
    #[ignore]
    /// Run this test with --nocapture, there should be one bar, scaled to your screen
    fn visual_test() {
        println!("\n");

        // this test is shown in the module docs
        let mut bar = Bar::new(0.5, false, None);
        println!("{}", bar);
        bar.progress = 41.0 / 42.0;
        println!("{}", bar);

        println!("\n");
    }
}
