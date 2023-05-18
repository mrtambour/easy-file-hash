use iced::widget::container;
use iced::Color;
use iced_native::color;

pub struct DropFileContainer;

impl container::StyleSheet for DropFileContainer {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let appearance = style.appearance(&iced::theme::Container::default());

        container::Appearance {
            border_radius: 15.0,
            border_width: 2.0,
            border_color: color!(90, 90, 90),
            ..appearance
        }
    }
}

impl From<DropFileContainer> for iced::theme::Container {
    fn from(style: DropFileContainer) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}
