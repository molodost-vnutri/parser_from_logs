#![windows_subsystem = "windows"]

use std::path::PathBuf;
mod data;
mod reading;
use std::thread;
use druid::widget::{BackgroundBrush, Button, Flex, Label};
use druid::{AppLauncher, Color, Data, Env, Lens, Widget, WidgetExt, WindowDesc};


#[derive(Clone, Data, Lens)]
struct AppData {
    num_lines: usize,
    folder: String,
    count_psw: usize,
}
impl AppData {
    fn new() -> Self {
        Self {
            num_lines: 0,
            count_psw: 0,
            folder: "".to_owned(),
        }
    }
}
fn build_ui() -> impl Widget<AppData> {

    let num_lines_label: Label<AppData> = Label::new("Строк нашёл:");
    let num_psw_label: Label<AppData> = Label::new("Файлов passwords найдено:");
    let folder_label: Label<AppData> = Label::new("Папка сессии:");

    let num_lines_input = Label::new(|data: &AppData, _env: &Env| data.num_lines.to_string());
    let num_psw_input = Label::new(|data: &AppData, _env: &Env| data.count_psw.to_string());
    let folder_input = Label::new(|data: &AppData, _env: &Env | data.folder.clone());


    let start_button = Button::new("На чек")
        .on_click(|_, data: &mut AppData, _: &Env| {
            on_start_click(data);
        })
        .fix_width(100.0);


    let num_lines_row: Flex<AppData> = Flex::row()
        .with_child(num_lines_label)
        .with_child(num_lines_input);

    let num_psw_row: Flex<AppData> = Flex::row()
        .with_child(num_psw_label)
        .with_child(num_psw_input);

    let folder_row: Flex<AppData> = Flex::row()
        .with_child(folder_label)
        .with_child(folder_input);
    
    
    
    Flex::column()
        .with_child(num_lines_row)
        .with_child(num_psw_row)
        .with_child(folder_row)
        .with_spacer(20.0)
        .with_child(start_button)
        .center()

    
}


fn main() {

    let main_window = WindowDesc::new(build_ui().background(BackgroundBrush::Color(Color::BLACK)))
        .title("ulp extractor by molodost_vnutri v0.1")
        .window_size((400.0, 300.0))
        .with_min_size((400.0, 300.0))
        .transparent(true);

    let data = AppData::new();
    let launcher = AppLauncher::with_window(main_window);
    launcher.launch(data).expect("Failed to launch application");
}
fn start_processing(data: &mut AppData) {

    if let Some(file) = tinyfiledialogs::select_folder_dialog("Папка с логами", "./") {

        let handle = thread::spawn(move || {
            let word = vec!["psw", "pass", "cred", "pwd", "word"];
            let parse = reading::reading_folder(&PathBuf::new().join(&file), &word);
            let result = reading::workdata(&parse);
            (result, parse.len(), file)
        });

        if let Ok((found, psw, file)) = handle.join() {
            data.count_psw += psw;
            data.num_lines += found;
            data.folder = file;
        }
    }
}

fn on_start_click(data: &mut AppData) {
    start_processing(data);
}