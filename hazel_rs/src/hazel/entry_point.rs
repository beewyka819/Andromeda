pub fn start_app<T>(app: T) where T: super::Application {
    app.run();
}