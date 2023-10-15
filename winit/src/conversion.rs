//! Convert [`winit`] types into [`iced_runtime`] types, and viceversa.
//!
//! [`winit`]: https://github.com/rust-windowing/winit
//! [`iced_runtime`]: https://github.com/iced-rs/iced/tree/0.10/runtime
use winit::event::MouseButton;
use winit::keyboard::Key;
use crate::core::keyboard;
use crate::core::mouse;
use crate::core::touch;
use crate::core::window;
use crate::core::{Event, Point};
use crate::Position;

/// Converts a winit window event into an iced event.
pub fn window_event(
    event: &winit::event::WindowEvent,
    scale_factor: f64,
    modifiers: winit::keyboard::ModifiersState,
) -> Option<Event> {
    use winit::event::WindowEvent;

    match event {
        WindowEvent::Resized(new_size) => {
            let logical_size = new_size.to_logical(scale_factor);

            Some(Event::Window(window::Event::Resized {
                width: logical_size.width,
                height: logical_size.height,
            }))
        }
        WindowEvent::ScaleFactorChanged { inner_size_writer, .. } => {
            println!("Scale factor changed");
 /*           let logical_size = new_inner_size.to_logical(scale_factor);

            Some(Event::Window(window::Event::Resized {
                width: logical_size.width,
                height: logical_size.height,
            }))*/
            None
        }
        WindowEvent::CloseRequested => {
            Some(Event::Window(window::Event::CloseRequested))
        }
        WindowEvent::CursorMoved { position, .. } => {
            let position = position.to_logical::<f64>(scale_factor);

            Some(Event::Mouse(mouse::Event::CursorMoved {
                position: Point::new(position.x as f32, position.y as f32),
            }))
        }
        WindowEvent::CursorEntered { .. } => {
            Some(Event::Mouse(mouse::Event::CursorEntered))
        }
        WindowEvent::CursorLeft { .. } => {
            Some(Event::Mouse(mouse::Event::CursorLeft))
        }
        WindowEvent::MouseInput { button, state, .. } => {
            let button = mouse_button(*button);

            Some(Event::Mouse(match state {
                winit::event::ElementState::Pressed => {
                    mouse::Event::ButtonPressed(button)
                }
                winit::event::ElementState::Released => {
                    mouse::Event::ButtonReleased(button)
                }
            }))
        }
        WindowEvent::MouseWheel { delta, .. } => match delta {
            winit::event::MouseScrollDelta::LineDelta(delta_x, delta_y) => {
                Some(Event::Mouse(mouse::Event::WheelScrolled {
                    delta: mouse::ScrollDelta::Lines {
                        x: *delta_x,
                        y: *delta_y,
                    },
                }))
            }
            winit::event::MouseScrollDelta::PixelDelta(position) => {
                Some(Event::Mouse(mouse::Event::WheelScrolled {
                    delta: mouse::ScrollDelta::Pixels {
                        x: position.x as f32,
                        y: position.y as f32,
                    },
                }))
            }
        },
        /*WindowEvent::ReceivedCharacter(c) if !is_private_use_character(*c) => {
            Some(Event::Keyboard(keyboard::Event::CharacterReceived(*c)))
        }*/
        WindowEvent::KeyboardInput {
            event:
                winit::event::KeyEvent {
                    logical_key,
                    state,
                    ..
                },
            ..
        } => Some(Event::Keyboard({
            let key_code = logical_key.clone();
            let modifiers = self::modifiers(modifiers);

            match state {
                winit::event::ElementState::Pressed => {
                    keyboard::Event::KeyPressed {
                        key_code,
                        modifiers,
                    }
                }
                winit::event::ElementState::Released => {
                    keyboard::Event::KeyReleased {
                        key_code,
                        modifiers,
                    }
                }
            }
        })),
        WindowEvent::ModifiersChanged(new_modifiers) => Some(Event::Keyboard(
            keyboard::Event::ModifiersChanged(self::modifiers(new_modifiers.state())),
        )),
        WindowEvent::Focused(focused) => Some(Event::Window(if *focused {
            window::Event::Focused
        } else {
            window::Event::Unfocused
        })),
        WindowEvent::HoveredFile(path) => {
            Some(Event::Window(window::Event::FileHovered(path.clone())))
        }
        WindowEvent::DroppedFile(path) => {
            Some(Event::Window(window::Event::FileDropped(path.clone())))
        }
        WindowEvent::HoveredFileCancelled => {
            Some(Event::Window(window::Event::FilesHoveredLeft))
        }
        WindowEvent::Touch(touch) => {
            Some(Event::Touch(touch_event(*touch, scale_factor)))
        }
        WindowEvent::Moved(position) => {
            let winit::dpi::LogicalPosition { x, y } =
                position.to_logical(scale_factor);

            Some(Event::Window(window::Event::Moved { x, y }))
        }
        _ => None,
    }
}

/// Converts a [`window::Level`] to a [`winit`] window level.
///
/// [`winit`]: https://github.com/rust-windowing/winit
pub fn window_level(level: window::Level) -> winit::window::WindowLevel {
    match level {
        window::Level::Normal => winit::window::WindowLevel::Normal,
        window::Level::AlwaysOnBottom => {
            winit::window::WindowLevel::AlwaysOnBottom
        }
        window::Level::AlwaysOnTop => winit::window::WindowLevel::AlwaysOnTop,
    }
}

/// Converts a [`Position`] to a [`winit`] logical position for a given monitor.
///
/// [`winit`]: https://github.com/rust-windowing/winit
pub fn position(
    monitor: Option<&winit::monitor::MonitorHandle>,
    (width, height): (u32, u32),
    position: Position,
) -> Option<winit::dpi::Position> {
    match position {
        Position::Default => None,
        Position::Specific(x, y) => {
            Some(winit::dpi::Position::Logical(winit::dpi::LogicalPosition {
                x: f64::from(x),
                y: f64::from(y),
            }))
        }
        Position::Centered => {
            if let Some(monitor) = monitor {
                let start = monitor.position();

                let resolution: winit::dpi::LogicalSize<f64> =
                    monitor.size().to_logical(monitor.scale_factor());

                let centered: winit::dpi::PhysicalPosition<i32> =
                    winit::dpi::LogicalPosition {
                        x: (resolution.width - f64::from(width)) / 2.0,
                        y: (resolution.height - f64::from(height)) / 2.0,
                    }
                    .to_physical(monitor.scale_factor());

                Some(winit::dpi::Position::Physical(
                    winit::dpi::PhysicalPosition {
                        x: start.x + centered.x,
                        y: start.y + centered.y,
                    },
                ))
            } else {
                None
            }
        }
    }
}

/// Converts a [`window::Mode`] to a [`winit`] fullscreen mode.
///
/// [`winit`]: https://github.com/rust-windowing/winit
pub fn fullscreen(
    monitor: Option<winit::monitor::MonitorHandle>,
    mode: window::Mode,
) -> Option<winit::window::Fullscreen> {
    match mode {
        window::Mode::Windowed | window::Mode::Hidden => None,
        window::Mode::Fullscreen => {
            Some(winit::window::Fullscreen::Borderless(monitor))
        }
    }
}

/// Converts a [`window::Mode`] to a visibility flag.
pub fn visible(mode: window::Mode) -> bool {
    match mode {
        window::Mode::Windowed | window::Mode::Fullscreen => true,
        window::Mode::Hidden => false,
    }
}

/// Converts a [`winit`] fullscreen mode to a [`window::Mode`].
///
/// [`winit`]: https://github.com/rust-windowing/winit
pub fn mode(mode: Option<winit::window::Fullscreen>) -> window::Mode {
    match mode {
        None => window::Mode::Windowed,
        Some(_) => window::Mode::Fullscreen,
    }
}

/// Converts a [`mouse::Interaction`] to a [`winit`] cursor icon.
///
/// [`winit`]: https://github.com/rust-windowing/winit
pub fn mouse_interaction(
    interaction: mouse::Interaction,
) -> winit::window::CursorIcon {
    use mouse::Interaction;

    match interaction {
        Interaction::Idle => winit::window::CursorIcon::Default,
        Interaction::Pointer => winit::window::CursorIcon::Pointer,
        Interaction::Working => winit::window::CursorIcon::Progress,
        Interaction::Grab => winit::window::CursorIcon::Grab,
        Interaction::Grabbing => winit::window::CursorIcon::Grabbing,
        Interaction::Crosshair => winit::window::CursorIcon::Crosshair,
        Interaction::Text => winit::window::CursorIcon::Text,
        Interaction::ResizingHorizontally => {
            winit::window::CursorIcon::EwResize
        }
        Interaction::ResizingVertically => winit::window::CursorIcon::NsResize,
        Interaction::NotAllowed => winit::window::CursorIcon::NotAllowed,
    }
}

/// Converts a `MouseButton` from [`winit`] to an [`iced`] mouse button.
///
/// [`winit`]: https://github.com/rust-windowing/winit
/// [`iced`]: https://github.com/iced-rs/iced/tree/0.10
pub fn mouse_button(mouse_button: winit::event::MouseButton) -> mouse::Button {
    match mouse_button {
        winit::event::MouseButton::Left => mouse::Button::Left,
        winit::event::MouseButton::Right => mouse::Button::Right,
        winit::event::MouseButton::Middle => mouse::Button::Middle,
        winit::event::MouseButton::Other(other) => mouse::Button::Other(other),
        MouseButton::Back => mouse::Button::Back,
        MouseButton::Forward => mouse::Button::Forward,
    }
}

/// Converts some `ModifiersState` from [`winit`] to an [`iced`] modifiers
/// state.
///
/// [`winit`]: https://github.com/rust-windowing/winit
/// [`iced`]: https://github.com/iced-rs/iced/tree/0.10
pub fn modifiers(
    modifiers: winit::keyboard::ModifiersState,
) -> keyboard::Modifiers {
    let mut result = keyboard::Modifiers::empty();

    result.set(keyboard::Modifiers::SHIFT, modifiers.shift_key());
    result.set(keyboard::Modifiers::CTRL, modifiers.control_key());
    result.set(keyboard::Modifiers::ALT, modifiers.alt_key());
    result.set(keyboard::Modifiers::LOGO, modifiers.super_key());

    result
}

/// Converts a physical cursor position to a logical `Point`.
pub fn cursor_position(
    position: winit::dpi::PhysicalPosition<f64>,
    scale_factor: f64,
) -> Point {
    let logical_position = position.to_logical(scale_factor);

    Point::new(logical_position.x, logical_position.y)
}

/// Converts a `Touch` from [`winit`] to an [`iced`] touch event.
///
/// [`winit`]: https://github.com/rust-windowing/winit
/// [`iced`]: https://github.com/iced-rs/iced/tree/0.10
pub fn touch_event(
    touch: winit::event::Touch,
    scale_factor: f64,
) -> touch::Event {
    let id = touch::Finger(touch.id);
    let position = {
        let location = touch.location.to_logical::<f64>(scale_factor);

        Point::new(location.x as f32, location.y as f32)
    };

    match touch.phase {
        winit::event::TouchPhase::Started => {
            touch::Event::FingerPressed { id, position }
        }
        winit::event::TouchPhase::Moved => {
            touch::Event::FingerMoved { id, position }
        }
        winit::event::TouchPhase::Ended => {
            touch::Event::FingerLifted { id, position }
        }
        winit::event::TouchPhase::Cancelled => {
            touch::Event::FingerLost { id, position }
        }
    }
}

/// Converts some [`UserAttention`] into it's `winit` counterpart.
///
/// [`UserAttention`]: window::UserAttention
pub fn user_attention(
    user_attention: window::UserAttention,
) -> winit::window::UserAttentionType {
    match user_attention {
        window::UserAttention::Critical => {
            winit::window::UserAttentionType::Critical
        }
        window::UserAttention::Informational => {
            winit::window::UserAttentionType::Informational
        }
    }
}

/// Converts some [`Icon`] into it's `winit` counterpart.
///
/// Returns `None` if there is an error during the conversion.
pub fn icon(icon: window::Icon) -> Option<winit::window::Icon> {
    let (pixels, size) = icon.into_raw();

    winit::window::Icon::from_rgba(pixels, size.width, size.height).ok()
}

// As defined in: http://www.unicode.org/faq/private_use.html
pub(crate) fn is_private_use_character(c: char) -> bool {
    matches!(
        c,
        '\u{E000}'..='\u{F8FF}'
        | '\u{F0000}'..='\u{FFFFD}'
        | '\u{100000}'..='\u{10FFFD}'
    )
}
