/* window.rs
 *
 * Copyright 2024 pixl_xip
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/app/drey/Chameleon/window.ui")]
    pub struct ChameleonWindow {
        // Template widgets
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChameleonWindow {
        const NAME: &'static str = "ChameleonWindow";
        type Type = super::ChameleonWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ChameleonWindow {}
    impl WidgetImpl for ChameleonWindow {}
    impl WindowImpl for ChameleonWindow {}
    impl ApplicationWindowImpl for ChameleonWindow {}
    impl AdwApplicationWindowImpl for ChameleonWindow {}
}

glib::wrapper! {
    pub struct ChameleonWindow(ObjectSubclass<imp::ChameleonWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl ChameleonWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
