Wayland IME demo via `zwp_input_method_v2` in Rust.

It doesn't work in many apps by default - notably, xwayland and electron apps.
Probably some workarounds that people use for fcitx5 or ibus would be needed to make it work everywhere.

useful resources:
- https://dorotac.eu/posts/input_method/
- https://gitlab.freedesktop.org/wayland/wayland-protocols/-/issues/39 - differences between text input protocols