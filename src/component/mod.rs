pub mod blank_space;
pub mod current_comparison;
pub mod current_pace;
pub mod delta;
pub mod detailed_timer;
pub mod graph;
pub mod possible_time_save;
pub mod previous_segment;
pub mod separator;
pub mod splits;
pub mod sum_of_best;
pub mod text;
pub mod timer;
pub mod title;
pub mod total_playtime;

use settings::{Gradient, Color};
use palette::{Alpha, Rgb};

const DEFAULT_INFO_TEXT_GRADIENT: Gradient = Gradient::Vertical(
    Color {
        rgba: Alpha {
            alpha: 0.06,
            color: Rgb {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
        },
    },
    Color {
        rgba: Alpha {
            alpha: 0.005,
            color: Rgb {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
        },
    },
);
