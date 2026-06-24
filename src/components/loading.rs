use iced::widget::{container, container::rounded_box, Float};

use crate::components::centered;

pub fn loading<'a, T>() -> Float<'a, T>
where
    T: 'a,
{
    centered(container("Loading...").style(rounded_box).padding(15))
}
