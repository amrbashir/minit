// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod accelerator;
mod icon;

use std::collections::{hash_map::Entry, HashMap};

use dpi::Position;
use gtk4::{gio::SimpleActionGroup, prelude::*};
pub(crate) use icon::PlatformIcon;

use crate::{
    accelerator::Accelerator,
    util::{AddOp, Counter},
    Icon, IsMenuItem, MenuId, MenuItemKind, MenuItemType, NativeIcon, PredefinedMenuItemType,
};

static COUNTER: Counter = Counter::new();

struct GtkMenuBar(gtk4::PopoverMenuBar, gtk4::gio::Menu);

impl GtkMenuBar {
    fn new() -> Self {
        let menu = gtk4::gio::Menu::new();
        Self(gtk4::PopoverMenuBar::from_model(Some(&menu)), menu)
    }

    fn widget(&self) -> &gtk4::PopoverMenuBar {
        &self.0
    }
}

pub struct Menu {
    id: MenuId,
    gtk_menubars: HashMap<u32, GtkMenuBar>,
    action_group: Option<SimpleActionGroup>,
}

impl Menu {
    pub fn new(id: Option<MenuId>) -> Self {
        Self {
            id: id.unwrap_or_else(|| MenuId::new_owned(COUNTER.next().to_string())),
            gtk_menubars: HashMap::new(),
            action_group: None,
        }
    }

    pub fn id(&self) -> &MenuId {
        &self.id
    }

    pub fn add_menu_item(&self, item: &dyn IsMenuItem, op: AddOp) -> crate::Result<()> {
        todo!()
    }

    pub fn remove(&self, item: &dyn IsMenuItem) -> crate::Result<()> {
        todo!()
    }

    pub fn items(&self) -> Vec<MenuItemKind> {
        todo!()
    }

    pub fn init_for_gtk_window<W, C>(
        &mut self,
        window: &W,
        container: Option<&C>,
    ) -> crate::Result<()>
    where
        W: gtk4::prelude::IsA<gtk4::Window>,
        W: gtk4::prelude::IsA<gtk4::Widget>,
        C: gtk4::prelude::IsA<gtk4::Widget>,
    {
        let id = window.as_ptr() as u32;

        if self.action_group.is_none() {
            self.action_group = Some(gtk4::gio::SimpleActionGroup::new());
        }

        // This is the first time this method has been called on this window
        // so we need to create the menubar and its parent box
        if let Entry::Vacant(e) = self.gtk_menubars.entry(id) {
            e.insert(GtkMenuBar::new());
        } else {
            return Err(crate::Error::AlreadyInitialized);
        }

        // Construct the entries of the menubar
        let menu_bar = &self.gtk_menubars[&id];

        window.insert_action_group(self.id().as_ref(), self.action_group.as_ref());

        // TODO:
        // for item in self.items() {
        //     self.add_menu_item_with_id(item.as_ref(), id)?;
        // }

        // add the menubar to the specified widget, otherwise to the window
        if let Some(container) = container {
            if container.type_().name() == "GtkBox" {
                let gtk_box = container.dynamic_cast_ref::<gtk4::Box>().unwrap();
                gtk_box.prepend(menu_bar.widget());
            } else if container.type_().name() == "GtkFixed" {
                let gtk_box = container.dynamic_cast_ref::<gtk4::Fixed>().unwrap();
                gtk_box.put(menu_bar.widget(), 0., 0.);
            } else if container.type_().name() == "GtkStack" {
                let gtk_box = container.dynamic_cast_ref::<gtk4::Stack>().unwrap();
                gtk_box.add_child(menu_bar.widget());
            }
        } else {
            window.set_child(Some(menu_bar.widget()));
        }

        // show the menu bar
        menu_bar.widget().set_visible(true);

        Ok(())
    }

    pub fn remove_for_gtk_window<W>(&self, window: &W) -> crate::Result<()>
    where
        W: gtk4::prelude::IsA<gtk4::Window>,
    {
        todo!()
    }

    pub fn hide_for_gtk_window<W>(&self, window: &W) -> crate::Result<()>
    where
        W: gtk4::prelude::IsA<gtk4::Window>,
    {
        todo!()
    }

    pub fn show_for_gtk_window<W>(&self, window: &W) -> crate::Result<()>
    where
        W: gtk4::prelude::IsA<gtk4::Window>,
    {
        todo!()
    }

    #[cfg(target_os = "linux")]
    pub fn is_visible_on_gtk_window<W>(&self, window: &W) -> bool
    where
        W: gtk4::prelude::IsA<gtk4::Window>,
    {
        todo!()
    }

    pub fn gtk_menubar_for_gtk_window<W>(&self, window: &W) -> Option<gtk4::PopoverMenuBar>
    where
        W: gtk4::prelude::IsA<gtk4::Window>,
    {
        todo!()
    }

    pub fn show_context_menu_for_gtk_window(
        &self,
        window: &gtk4::Window,
        position: Option<Position>,
    ) -> bool {
        todo!()
    }
}

pub struct MenuChild {}

impl MenuChild {
    pub fn new(
        text: &str,
        enabled: bool,
        accelerator: Option<Accelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {}
    }

    pub fn id(&self) -> &MenuId {
        todo!()
    }

    pub fn item_type(&self) -> &MenuItemType {
        todo!()
    }

    pub fn text(&self) -> String {
        todo!()
    }

    pub fn set_text(&self, text: &str) {
        todo!()
    }

    pub fn is_enabled(&self) -> bool {
        todo!()
    }

    pub fn set_enabled(&self, enabled: bool) {
        todo!()
    }

    pub fn set_accelerator(&self, accelerator: Option<Accelerator>) -> crate::Result<()> {
        todo!()
    }
}

impl MenuChild {
    pub fn new_submenu(text: &str, enabled: bool, id: Option<MenuId>) -> Self {
        Self {}
    }

    pub fn add_menu_item(&self, item: &dyn IsMenuItem, op: AddOp) -> crate::Result<()> {
        todo!()
    }

    pub fn remove(&self, item: &dyn IsMenuItem) -> crate::Result<()> {
        todo!()
    }

    pub fn items(&self) -> Vec<MenuItemKind> {
        todo!()
    }

    pub fn show_context_menu_for_gtk_window(
        &self,
        w: &gtk4::Window,
        position: Option<Position>,
    ) -> bool {
        todo!()
    }
}

impl MenuChild {
    pub fn new_predefined(item: PredefinedMenuItemType, text: Option<String>) -> Self {
        Self {}
    }
}

impl MenuChild {
    pub fn new_check(
        text: &str,
        enabled: bool,
        checked: bool,
        accelerator: Option<Accelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {}
    }

    pub fn is_checked(&self) -> bool {
        todo!()
    }

    pub fn set_checked(&self, checked: bool) {
        todo!()
    }
}

impl MenuChild {
    pub fn new_icon(
        text: &str,
        enabled: bool,
        icon: Option<Icon>,
        accelerator: Option<Accelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {}
    }

    pub fn new_native_icon(
        text: &str,
        enabled: bool,
        icon: Option<NativeIcon>,
        accelerator: Option<Accelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {}
    }

    pub fn set_icon(&self, icon: Option<Icon>) {}
}
