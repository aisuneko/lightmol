mod calc;
mod utils;
use fltk::{
    app, button::Button, button::RadioRoundButton, button::ReturnButton, dialog::message_default,
    enums::Color, frame::Frame, group::Flex, image::SvgImage, input::Input, prelude::*,
    window::Window,
};
use fltk_theme::{widget_schemes::fluent::colors::*, SchemeType, WidgetScheme};
fn main() {
    let app = app::App::default();
    let widget_scheme = WidgetScheme::new(SchemeType::Fluent);
    widget_scheme.apply();
    let mut wind = Window::default().with_size(400, 160).with_label("Lightmol");
    let image = SvgImage::load("./icon.svg").expect("Icon file not found");
    wind.set_icon(Some(image));
    wind.set_color(Color::from_rgb(255, 255, 255));
    let mut flex = Flex::default()
        .with_size(380, 140)
        .center_of_parent()
        .column();
    let mut flex2 = Flex::default().row();
    flex.set_size(&flex2, 50);
    let mut edit = Input::default();
    edit.set_text_size(20);
    flex2.set_size(&edit, 210);
    let mut button = ReturnButton::default().with_label("Calculate");
    button.set_color(Color::from_rgba_tuple(ACCENT_COLOR));
    button.set_selection_color(button.color().darker());
    flex2.set_size(&button, 110);
    let mut about_button = Button::default().with_label("About");
    flex2.set_size(&about_button, 50);
    flex2.end();
    let flex3 = Flex::default().row().center_of_parent();
    flex.set_size(&flex3, 25);
    let _normal_radiobutton = RadioRoundButton::default()
        .with_label("Normal Mode")
        .toggle(true);
    // let spacer = Frame::default();
    // let mut normal_spinner = Spinner::default().with_label("Decimals: ");
    // normal_spinner.set_value(2.0);
    // normal_spinner.set_range(0.0, 10.0);
    // normal_spinner.set_step(1.0);
    // normal_spinner.set_label_size(10);
    let school_radiobutton = RadioRoundButton::default().with_label("School Mode");
    flex3.end();
    let mut frame = Frame::default().with_label("See results here");
    frame.set_label_size(20);
    flex.end();
    wind.end();
    wind.show();
    button.set_callback(move |_| {
        frame.set_label_size(36);
        frame.set_label(
            &calc::parse(
                edit.value().replace("·", "-").as_str(),
                school_radiobutton.is_toggled(),
            )
            .to_string(),
        );
    });
    about_button.set_callback(move |_| message_default(utils::ABOUT_MSG));
    app.run().unwrap();
}
