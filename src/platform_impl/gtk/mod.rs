// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod accelerator;
mod icon;

use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use accelerator::to_gtk_mnemonic;
use dpi::Position;
use gtk4::{gio, glib::VariantTy, prelude::*};
pub(crate) use icon::PlatformIcon;

use crate::{
    accelerator::Accelerator,
    util::{AddOp, Counter},
    Icon, IsMenuItem, MenuEvent, MenuId, MenuItemKind, MenuItemType, NativeIcon,
    PredefinedMenuItemType,
};

static COUNTER: Counter = Counter::new();

const DEFAULT_ACTION: &str = "_internal_sendEvent";
const DEFAULT_ACTION_GROUP: &str = "muda";
const DEFAULT_DETAILED_ACTION: &str = "muda._internal_sendEvent";

struct GtkMenuBar(gtk4::PopoverMenuBar, gio::Menu);

impl GtkMenuBar {
    fn new() -> Self {
        let menu = gio::Menu::new();
        Self(gtk4::PopoverMenuBar::from_model(Some(&menu)), menu)
    }

    fn widget(&self) -> &gtk4::PopoverMenuBar {
        &self.0
    }

    fn menu(&self) -> &gio::Menu {
        &self.1
    }
}

pub struct Menu {
    id: MenuId,
    instances: HashMap<u32, GtkMenuBar>,
    action_group: Option<gio::SimpleActionGroup>,
    children: Vec<Rc<RefCell<MenuChild>>>,
}

impl Menu {
    pub fn new(id: Option<MenuId>) -> Self {
        Self {
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            instances: HashMap::new(),
            action_group: None,
            children: Vec::new(),
        }
    }

    pub fn id(&self) -> &MenuId {
        &self.id
    }

    pub fn add_menu_item(&mut self, item: &dyn IsMenuItem, op: AddOp) -> crate::Result<()> {
        match op {
            AddOp::Append => self.children.push(item.child()),
            AddOp::Insert(i) => self.children.insert(i, item.child()),
        }

        for (menu_id, menu_bar) in &self.instances {
            let gtk_item = item.make_gtk_menu_item(*menu_id, self.action_group.as_ref())?;
            match op {
                AddOp::Append => menu_bar.menu().append_item(&gtk_item),
                AddOp::Insert(position) => menu_bar.menu().insert_item(position as i32, &gtk_item),
            }
        }

        Ok(())
    }

    pub fn add_menu_item_with_id(&mut self, item: &dyn IsMenuItem, id: u32) -> crate::Result<()> {
        for (menu_id, menu_bar) in self.instances.iter().filter(|m| *m.0 == id) {
            let gtk_item = item.make_gtk_menu_item(*menu_id, self.action_group.as_ref())?;
            menu_bar.menu().append_item(&gtk_item);
        }

        Ok(())
    }

    pub fn remove(&self, item: &dyn IsMenuItem) -> crate::Result<()> {
        todo!()
    }

    pub fn items(&self) -> Vec<MenuItemKind> {
        self.children
            .iter()
            .map(|c| c.borrow().kind(c.clone()))
            .collect()
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
            let action_group = gtk4::gio::SimpleActionGroup::new();

            let action = gtk4::gio::SimpleAction::new(DEFAULT_ACTION, Some(&VariantTy::STRING));
            action.connect_activate(|_, v| {
                if let Some(v) = v {
                    MenuEvent::send(MenuEvent {
                        id: MenuId(v.as_ref().to_string()),
                    });
                }
            });
            action_group.add_action(&action);

            self.action_group = Some(action_group);
        }

        // This is the first time this method has been called on this window
        // so we need to create the menubar
        if let Entry::Vacant(e) = self.instances.entry(id) {
            e.insert(GtkMenuBar::new());
        } else {
            return Err(crate::Error::AlreadyInitialized);
        }

        window.insert_action_group(DEFAULT_ACTION_GROUP, self.action_group.as_ref());

        for item in self.items() {
            self.add_menu_item_with_id(item.as_ref(), id)?;
        }

        let menu_bar = self.instances[&id].widget();

        // add the menubar to the specified widget, otherwise to the window
        if let Some(container) = container {
            if container.type_().name() == "GtkBox" {
                let gtk_box = container.dynamic_cast_ref::<gtk4::Box>().unwrap();
                gtk_box.prepend(menu_bar);
            } else if container.type_().name() == "GtkFixed" {
                let gtk_box = container.dynamic_cast_ref::<gtk4::Fixed>().unwrap();
                gtk_box.put(menu_bar, 0., 0.);
            } else if container.type_().name() == "GtkStack" {
                let gtk_box = container.dynamic_cast_ref::<gtk4::Stack>().unwrap();
                gtk_box.add_child(menu_bar);
            }
        } else {
            window.set_child(Some(menu_bar));
        }

        // show the menu bar
        menu_bar.set_visible(true);

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

#[derive(Clone)]
enum GtkChild {
    Item(gio::MenuItem),
    Submenu {
        id: u32,
        item: gio::MenuItem,
        menu: gio::Menu,
        action_group: Option<gio::SimpleActionGroup>,
    },
}

impl GtkChild {
    fn id(&self) -> u32 {
        match self {
            GtkChild::Item(_) => {
                unreachable!("This is a bug report to https://github.com/tauri-apps/muda")
            }
            GtkChild::Submenu { id, .. } => *id,
        }
    }

    fn menu(&self) -> &gio::Menu {
        match self {
            GtkChild::Item(_) => {
                unreachable!("This is a bug report to https://github.com/tauri-apps/muda")
            }
            GtkChild::Submenu { menu, .. } => menu,
        }
    }

    fn action_group(&self) -> Option<&gio::SimpleActionGroup> {
        match self {
            GtkChild::Item(_) => None,
            GtkChild::Submenu { action_group, .. } => action_group.as_ref(),
        }
    }
}

pub struct MenuChild {
    id: MenuId,
    text: String,
    enabled: bool,
    accelerator: Option<Accelerator>,

    checked: bool,

    type_: MenuItemType,

    instances: HashMap<u32, Vec<GtkChild>>,
    children: Vec<Rc<RefCell<MenuChild>>>,
}

impl MenuChild {
    pub fn new_submenu(text: &str, enabled: bool, id: Option<MenuId>) -> Self {
        Self {
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            text: text.to_string(),
            enabled,
            checked: false,
            accelerator: None,
            type_: MenuItemType::Submenu,
            instances: HashMap::new(),
            children: Vec::new(),
        }
    }

    fn create_gtk_item_for_submenu(
        &mut self,
        menu_id: u32,
        action_group: Option<&gio::SimpleActionGroup>,
    ) -> crate::Result<gio::MenuItem> {
        let menu = gio::Menu::new();
        let item = gio::MenuItem::new_submenu(Some(&to_gtk_mnemonic(&self.text)), &menu);

        let id = COUNTER.next();

        let child = GtkChild::Submenu {
            item: item.clone(),
            menu,
            id,
            action_group: action_group.cloned(),
        };

        self.instances.entry(menu_id).or_default().push(child);

        for item in self.items() {
            self.add_menu_item_with_id(item.as_ref(), id)?;
        }

        Ok(item)
    }

    pub fn add_menu_item(&mut self, item: &dyn IsMenuItem, op: AddOp) -> crate::Result<()> {
        match op {
            AddOp::Append => self.children.push(item.child()),
            AddOp::Insert(i) => self.children.insert(i, item.child()),
        }

        for menus in self.instances.values() {
            for gtk_child in menus {
                let gtk_item = item.make_gtk_menu_item(gtk_child.id(), gtk_child.action_group())?;

                match op {
                    AddOp::Append => gtk_child.menu().append_item(&gtk_item),
                    AddOp::Insert(position) => {
                        gtk_child.menu().insert_item(position as i32, &gtk_item)
                    }
                }
            }
        }

        Ok(())
    }

    pub fn add_menu_item_with_id(&self, item: &dyn IsMenuItem, id: u32) -> crate::Result<()> {
        for menus in self.instances.values() {
            for gtk_child in menus.iter().filter(|m| m.id() == id) {
                let gtk_item = item.make_gtk_menu_item(gtk_child.id(), gtk_child.action_group())?;
                gtk_child.menu().append_item(&gtk_item);
            }
        }

        Ok(())
    }

    pub fn remove(&self, item: &dyn IsMenuItem) -> crate::Result<()> {
        todo!()
    }

    pub fn items(&self) -> Vec<MenuItemKind> {
        self.children
            .iter()
            .map(|c| c.borrow().kind(c.clone()))
            .collect()
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
    pub fn new(
        text: &str,
        enabled: bool,
        accelerator: Option<Accelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            text: text.to_string(),
            enabled,
            accelerator,
            checked: false,
            type_: MenuItemType::MenuItem,
            instances: HashMap::new(),
            children: Vec::new(),
        }
    }

    fn create_gtk_item_for_menu_item(
        &mut self,
        menu_id: u32,
        _action_group: Option<&gio::SimpleActionGroup>,
    ) -> crate::Result<gio::MenuItem> {
        let item = gio::MenuItem::new(
            Some(&to_gtk_mnemonic(&self.text)),
            Some(&format!("{DEFAULT_DETAILED_ACTION}::{}", self.id.as_ref())),
        );

        let child = GtkChild::Item(item.clone());
        self.instances.entry(menu_id).or_default().push(child);

        Ok(item)
    }

    pub fn id(&self) -> &MenuId {
        &self.id
    }

    pub fn item_type(&self) -> &MenuItemType {
        &self.type_
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
    pub fn new_predefined(item_type: PredefinedMenuItemType, text: Option<String>) -> Self {
        Self {
            id: MenuId(COUNTER.next().to_string()),
            text: text.unwrap_or_else(|| item_type.text().to_string()),
            enabled: true,
            accelerator: None,
            checked: false,
            type_: MenuItemType::Predefined,
            instances: HashMap::new(),
            children: Vec::new(),
        }
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
        Self {
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            text: text.to_string(),
            enabled,
            accelerator,
            checked,
            type_: MenuItemType::Check,
            instances: HashMap::new(),
            children: Vec::new(),
        }
    }

    fn create_gtk_item_for_check_menu_item(
        &mut self,
        menu_id: u32,
        action_group: Option<&gio::SimpleActionGroup>,
    ) -> crate::Result<gio::MenuItem> {
        let item = gio::MenuItem::new(
            Some(&to_gtk_mnemonic(&self.text)),
            Some(&format!("{DEFAULT_ACTION_GROUP}.{}", self.id.as_ref())),
        );

        if let Some(action_group) = action_group {
            let state = &self.checked.to_variant();
            let action = gio::SimpleAction::new_stateful(self.id.as_ref(), None, state);
            let id = self.id.clone();
            action.connect_state_notify(move |_| {
                MenuEvent::send(MenuEvent { id: id.clone() });
            });
            action_group.add_action(&action);
        }

        let child = GtkChild::Item(item.clone());
        self.instances.entry(menu_id).or_default().push(child);

        Ok(item)
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
        Self {
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            text: text.to_string(),
            enabled,
            accelerator,
            checked: false,
            type_: MenuItemType::Icon,
            instances: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn new_native_icon(
        text: &str,
        enabled: bool,
        icon: Option<NativeIcon>,
        accelerator: Option<Accelerator>,
        id: Option<MenuId>,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(|| MenuId(COUNTER.next().to_string())),
            text: text.to_string(),
            enabled,
            accelerator,
            checked: false,
            type_: MenuItemType::Submenu,
            instances: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn set_icon(&self, icon: Option<Icon>) {}
}

impl dyn IsMenuItem + '_ {
    fn make_gtk_menu_item(
        &self,
        menu_id: u32,
        action_group: Option<&gio::SimpleActionGroup>,
    ) -> crate::Result<gio::MenuItem> {
        let kind = self.kind();
        let mut child = kind.child_mut();
        match child.item_type() {
            MenuItemType::Submenu => child.create_gtk_item_for_submenu(menu_id, action_group),
            MenuItemType::MenuItem => child.create_gtk_item_for_menu_item(menu_id, action_group),
            MenuItemType::Check => child.create_gtk_item_for_check_menu_item(menu_id, action_group),
            _ => todo!(),
            // MenuItemType::Predefined => {
            //     child.create_gtk_item_for_predefined_menu_item(menu_id, action_group)
            // }
            // MenuItemType::Icon => child.create_gtk_item_for_icon_menu_item(
            //     menu_id,
            //     action_group,
            // ),
        }
    }
}
