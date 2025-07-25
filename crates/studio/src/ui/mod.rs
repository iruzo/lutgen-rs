use egui::Context;

use crate::App;

mod about;
mod central;
mod left;
mod status;
mod top;

impl App {
    /// Render ui state
    pub fn show(&mut self, ctx: &Context) {
        self.show_about_dialog(ctx);
        self.show_topbar(ctx);
        self.show_statusline(ctx);
        if self.show_sidebar(ctx) {
            self.apply();
        }
        self.show_central_panel(ctx);
    }
}
