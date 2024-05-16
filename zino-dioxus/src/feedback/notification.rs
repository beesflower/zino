use crate::class::Class;
use dioxus::prelude::*;
use zino_core::SharedString;

/// A simple colored block meant to draw the attention to the user about something.
pub fn Notification(props: NotificationProps) -> Element {
    rsx! {
        div {
            class: props.class,
            class: if !props.color.is_empty() { "is-{props.color}" },
            class: if !props.theme.is_empty() { "is-{props.theme}" },
            class: if !props.visible { "is-hidden" },
            position: "fixed",
            top: "4rem",
            right: "0.75rem",
            z_index: 99,
            if props.on_close.is_some() {
                button {
                    class: props.close_class,
                    onclick: move |event| {
                        if let Some(handler) = props.on_close.as_ref() {
                            handler.call(event);
                        }
                    }
                }
            }
            { props.children }
        }
    }
}

/// The [`Notification`] properties struct for the configuration of the component.
#[derive(Clone, PartialEq, Props)]
pub struct NotificationProps {
    /// The class attribute for the component.
    #[props(into, default = "notification".into())]
    pub class: Class,
    /// A class to apply to the `close` button element.
    #[props(into, default = "delete".into())]
    pub close_class: Class,
    /// The color of the notification: `primary` | `link` | `info` | `success` | `warning` | `danger`.
    #[props(into, default)]
    pub color: SharedString,
    /// The theme of the notification: `light`.
    #[props(into, default)]
    pub theme: SharedString,
    /// An event handler to be called when the `close` button is clicked.
    pub on_close: Option<EventHandler<MouseEvent>>,
    /// A flag to determine whether the modal is visible or not.
    #[props(default)]
    pub visible: bool,
    /// The children to render within the component.
    children: Element,
}
