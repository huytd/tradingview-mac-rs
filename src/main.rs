extern crate web_view;

use web_view::*;

fn main() {
    let size = (1024, 680);
    let resizable = true;
    let debug = false;
    let titlebar_transparent = true;
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();
    
    run(
        "",
        Content::Url("https://tradingview.com/chart/"),
        Some(size),
        resizable,
        debug,
        titlebar_transparent,
        move |mut webview| {
            webview.set_background_color(1.0, 1.0, 1.0, 1.0);
        },
        frontend_cb,
        userdata
    );
}
