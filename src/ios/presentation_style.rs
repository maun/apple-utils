use objc2_ui_kit::UIModalPresentationStyle;

#[derive(Debug, Clone, Copy)]
pub enum PresentationStyle {
    FullScreen,
    PageSheet,
    FormSheet,
    CurrentContext,
    Custom,
    OverFullScreen,
    OverCurrentContext,
    Popover,
    BlurOverFullScreen,
    None,
    Automatic,
}

impl Into<UIModalPresentationStyle> for PresentationStyle {
    fn into(self) -> UIModalPresentationStyle {
        match self {
            PresentationStyle::FullScreen => UIModalPresentationStyle::FullScreen,
            PresentationStyle::PageSheet => UIModalPresentationStyle::PageSheet,
            PresentationStyle::FormSheet => UIModalPresentationStyle::FormSheet,
            PresentationStyle::CurrentContext => UIModalPresentationStyle::CurrentContext,
            PresentationStyle::Custom => UIModalPresentationStyle::Custom,
            PresentationStyle::OverFullScreen => UIModalPresentationStyle::OverFullScreen,
            PresentationStyle::OverCurrentContext => UIModalPresentationStyle::OverCurrentContext,
            PresentationStyle::Popover => UIModalPresentationStyle::Popover,
            PresentationStyle::BlurOverFullScreen => UIModalPresentationStyle::BlurOverFullScreen,
            PresentationStyle::None => UIModalPresentationStyle::None,
            PresentationStyle::Automatic => UIModalPresentationStyle::Automatic,
        }
    }
}
