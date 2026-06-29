# egui-text-animation

NOTE: all readme is written by AI

`egui-text-animation` provides simple text animation utilities for the [egui](https://github.com/emilk/egui) library.

This library offers the `TextAnimator` struct to create various text animations. It's designed to be easy to integrate
into your existing egui applications. The core of the library is the `TextAnimator` struct, which manages the state of
the animation, and the `AnimationType` enum, which determines the type of animation to perform.

![Crates.io Version](https://img.shields.io/crates/v/egui_text_animation)

## Showcase

![animations_ebPt6SYaHj](https://github.com/user-attachments/assets/977247f8-84fb-43d6-ac88-a3bc86f9faac)

## Features

- **Fade-in Animation:** Animate text to gradually appear, character by character. See `AnimationType::FadeIn`.
- **Typewriter Animation:** Animate text to appear as if it's being typed. See `AnimationType::Typewriter`.
- **Hacker Animation:** Animate text to appear as if it's being decoded. See `AnimationType::Hacker`.
- **Customizable Speed:** Control the speed of the animation with `TextAnimator::set_speed`.
- **Easy Integration:** Simply create a `TextAnimator`, call `TextAnimator::process_animation` each frame, and then
  render with `TextAnimator::render`.
- **Automatic Repainting:** Call `ctx.request_repaint()` inside your update loop to ensure smooth animation.
- **Animation Control:** You can check if the animation is finished with `TextAnimator::is_animation_finished` and reset
  it with `TextAnimator::reset`.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
egui-text-animation = "0.1.4" # Replace with the actual version (or use a git dependency)
eframe = "0.35.0" # Or the latest version that suits your needs.
```

Replace `"0.1.4"` with the actual released version (if any), you also can use a
git dependency:

```toml
[dependencies]
egui-text-animation = { git = "https://github.com/dest4590/egui-text-animation" }
eframe = "0.35.0"
```

## Animation Types

The `AnimationType` enum provides the following animation types:

- `AnimationType::FadeIn`: Characters gradually fade in from transparent to fully opaque.
- `AnimationType::Typewriter`: Characters appear one by one, simulating a typewriter effect.
- `AnimationType::Hacker`: Characters cycle through random characters before settling on the final character.

## API Reference

See the [docs.rs documentation](https://docs.rs/egui-text-animation) for a complete API reference.

## Notes

- The `unstable_dt` value from `ctx.input(|i| i.unstable_dt)` is used for frame-independent animation timing. This
  ensures the animation runs at the correct speed regardless of the application's frame rate.
- It's crucial to call `ctx.request_repaint()` during the animation to ensure that egui re-renders the UI, thus updating
  the animation.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
