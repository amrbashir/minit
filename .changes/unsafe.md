---
"muda": "minor"
---

**Breaking change** Marked a few methods with `unsafe` to better represent the safety guarantees:

- `ContextMenu::show_context_menu_for_hwnd`
- `ContextMenu::attach_menu_subclass_for_hwnd`
- `ContextMenu::detach_menu_subclass_from_hwnd`
- `Menu::init_for_hwnd`
- `Menu::init_for_hwnd_with_theme`
- `Menu::set_theme_for_hwnd`
- `Menu::remove_for_hwnd`
- `Menu::hide_for_hwnd`
- `Menu::show_for_hwnd`
- `Menu::is_visible_on_hwnd`
