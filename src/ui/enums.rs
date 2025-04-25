#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuMode {
    Basic,
    FillScreen,
    Particle,
    Exit,
}

// impl MenuMode {
//     pub fn from_index(index: usize) -> Option<Self> {
//         match index {
//             0 => Some(MenuMode::Basic),
//             1 => Some(MenuMode::FillScreen),
//             2 => Some(MenuMode::Particle),
//             3 => Some(MenuMode::Exit),
//             _ => None,
//         }
//     }
// }