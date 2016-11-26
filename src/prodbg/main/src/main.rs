extern crate core;
extern crate prodbg_api;
extern crate imgui_sys;
extern crate settings;
extern crate project;

#[macro_use]
extern crate wrui;

use wrui::Ui;
use wrui::Widget;

fn callback(_t: &mut usize) {
    println!("callback");
}

fn main() {
    let t: usize = 0;

    let ui = Ui::new_default();

    let app = ui.application_create();

    let main_window = ui.main_window_create();

    let button = ui.push_button_create(); 
    button.set_title("Foo!");
    button.set_size(150, 150);

    connect_released!(&button, &t, usize, callback);

    main_window.set_central_widget(&button);
    main_window.set_window_title("ProDBG");
    main_window.show();

    app.run();


	/*
    match dir_searcher::find_working_dir() {
        Some(wd) => std::env::set_current_dir(wd).unwrap(),
        None => {
            panic!("\
            Could not find proper working directory. Working directory \
                    should contain accessible \"data\" directory.")
        }
    }

    let mut sessions = Sessions::new();
    let mut windows = Windows::new();
    let mut settings = Settings::new();

    let mut lib_handler = DynamicReload::new(None, Some("t2-output"), Search::Backwards);
    let mut plugins = Plugins::new();

    let view_plugins = Rc::new(RefCell::new(ViewPlugins::new()));
    let backend_plugins = Rc::new(RefCell::new(BackendPlugins::new()));

    let session = sessions.create_instance();

    settings.load_default_settings("data/settings.json").unwrap_or_else(|e| {
        println!("Unable to load data/settings: {}", e);
    });

    let project = Project::load("data/current_project.json").unwrap_or_else(|e| {
        println!("Unable to load current project {}, using defaults", e);
        Project::new()
    });

    plugins.add_handler(&view_plugins);
    plugins.add_handler(&backend_plugins);
    plugins.search_load_plugins(&mut lib_handler);

    let backend = backend_plugins.borrow_mut()
        .create_instance(&project.backend_name, &project.backend_data);

    if let Some(backend) = backend {
        println!("Crated instande {}", project.backend_name);
        if let Some(session) = sessions.get_session(session) {
            println!("set backend");
            session.set_backend(Some(backend));
        }
    }

    windows.create_default(&settings, backend_plugins.borrow_mut().get_plugin_names());
    windows.init(&project.layout_data,
                 &mut view_plugins.borrow_mut(),
                 backend,
                 &mut backend_plugins.borrow_mut());

    loop {
        plugins.update(&mut lib_handler);
        sessions.update(&mut backend_plugins.borrow_mut());
        windows.update(&mut sessions,
                       &mut view_plugins.borrow_mut(),
                       &mut backend_plugins.borrow_mut());

        if windows.should_exit() {
            break;
        }

        // TODO: Proper config and sleep timings
        std::thread::sleep(Duration::from_millis(5));
    }
    */
}

// dummy
#[no_mangle]
pub fn init_plugin() {}
