---
"muda": patch
---

Return `bool` in `ContextMenu::show_context_menu_for_hwnd` on Windows and `ContextMenu::show_context_menu_for_nsview` on macOS to indicate why the context menu was closed.