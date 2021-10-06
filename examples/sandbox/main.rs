use andromeda::*;

struct ExampleLayer;

impl Layer for ExampleLayer {
    fn get_name(&self) -> String {
        String::from("ExampleLayer")
    }

    fn on_update(&mut self) {
        
    }

    fn on_event(&mut self, event: &mut Event) {
        match event {
            Event::KeyTyped(e) => ad_trace!("{}", e),
            _ => {}
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let (mut app, event_loop) = Application::new()?;

    let layer = create_layer_ref(Box::new(ExampleLayer));

    app.push_layer(&layer);

    Application::run(app, event_loop);

    Ok(())
}