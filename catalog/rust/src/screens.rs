//! The catalog's screens, mirroring material-components-android's own list.
//! Each is Octoscript DSL, evaluated on device.

pub const KIT: &str = include_str!("../octoscript/kit.octoscript");

macro_rules! screens {
    ($($route:literal => $title:literal , $file:literal);* $(;)?) => {
        /// (route, display title) — drives the table of contents.
        pub const ROUTES: &[(&str, &str)] = &[ $(($route, $title)),* ];

        pub fn body(route: &str) -> &'static str {
            match route {
                $($route => include_str!(concat!("../octoscript/", $file)),)*
                _ => "{t:\"col\", pad: 24, c:[{t:\"text\", variant:\"titleMedium\", text:\"Unknown route\"}]}",
            }
        }
    };
}

screens! {
    "allcomponents"     => "All components",        "allcomponents.octoscript";
    "adaptive"          => "Adaptive layouts",      "adaptive.octoscript";
    "badge"             => "Badge",                 "badge.octoscript";
    "bottomappbar"      => "Bottom app bar",        "bottomappbar.octoscript";
    "bottomnav"         => "Bottom navigation",     "bottomnav.octoscript";
    "bottomsheet"       => "Bottom sheet",          "bottomsheet.octoscript";
    "button"            => "Button",                "button.octoscript";
    "card"              => "Card",                  "card.octoscript";
    "carousel"          => "Carousel",              "carousel.octoscript";
    "checkbox"          => "Checkbox",              "checkbox.octoscript";
    "chip"              => "Chip",                  "chip.octoscript";
    "color"             => "Color palette",         "color.octoscript";
    "datepicker"        => "Date picker",           "datepicker.octoscript";
    "dialog"            => "Dialog",                "dialog.octoscript";
    "divider"           => "Divider",               "divider.octoscript";
    "dockedtoolbar"     => "Docked toolbar",        "dockedtoolbar.octoscript";
    "elevation"         => "Elevation",             "elevation.octoscript";
    "fab"               => "Floating action button","fab.octoscript";
    "floatingtoolbar"   => "Floating toolbar",      "floatingtoolbar.octoscript";
    "font"              => "Typography",            "font.octoscript";
    "imageview"         => "Image view",            "imageview.octoscript";
    "listitem"          => "List item",             "listitem.octoscript";
    "loadingindicator"  => "Loading indicator",     "loadingindicator.octoscript";
    "materialswitch"    => "Switch",                "materialswitch.octoscript";
    "menu"              => "Menu",                  "menu.octoscript";
    "musicplayer"       => "Music player",          "musicplayer.octoscript";
    "navigationdrawer"  => "Navigation drawer",     "navigationdrawer.octoscript";
    "octoswidgets"      => "octos-one widgets",     "octoswidgets.octoscript";
    "navigationrail"    => "Navigation rail",       "navigationrail.octoscript";
    "preferences"       => "Preferences",           "preferences.octoscript";
    "progressindicator" => "Progress indicator",    "progressindicator.octoscript";
    "radiobutton"       => "Radio button",          "radiobutton.octoscript";
    "search"            => "Search",                "search.octoscript";
    "shapetheming"      => "Shape theming",         "shapetheming.octoscript";
    "sidesheet"         => "Side sheet",            "sidesheet.octoscript";
    "slider"            => "Slider",                "slider.octoscript";
    "snackbar"          => "Snackbar",              "snackbar.octoscript";
    "tabs"              => "Tabs",                  "tabs.octoscript";
    "textfield"         => "Text field",            "textfield.octoscript";
    "timepicker"        => "Time picker",           "timepicker.octoscript";
    "topappbar"         => "Top app bar",           "topappbar.octoscript";
    "transition"        => "Transition",            "transition.octoscript";
}
