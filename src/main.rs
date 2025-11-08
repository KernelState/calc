mod lexer;
mod parser;

use gtk::{Application, ApplicationWindow, Button, CssProvider, Grid, Label, StyleContext};
use gtk::{Box as GBox, gdk::Screen as GScreen, prelude::*};
use lexer::Lexer;
use std::cell::RefCell;
use std::rc::Rc;

fn compute(input: String) -> Result<f64, parser::ParseError> {
    let mut lx = Lexer::from_string(input);
    lx.lex();
    let toks = lx.toks;
    let mut prsr = parser::Parser::from_toks(toks);
    return prsr.eval();
}

fn main() {
    let app = Application::builder()
        .application_id("org.example.calculator")
        .build();

    app.connect_activate(|app| {
        let css_provider = CssProvider::new();
        css_provider.load_from_path("style.css").unwrap();
        // Shared input
        let input = Rc::new(RefCell::new(String::new()));

        // Create window
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(200)
            .default_height(300)
            .title("TESTING")
            .resizable(false)
            .build();

        let contents = GBox::new(gtk::Orientation::Vertical, 10);

        let inp_ = Rc::clone(&input);
        let input_val = inp_.borrow();
        let out_label = Rc::new(RefCell::new(Label::new(Some(input_val.as_str()))));
        let lbl = Rc::clone(&out_label);
        let out = lbl.borrow();
        contents.pack_start(&out as &Label, true, false, 20);

        // Grid layout
        let grid = Grid::new();
        grid.set_row_spacing(10);
        grid.set_column_spacing(10);

        contents.pack_start(&grid, true, true, 10);

        // Set grid as window's child
        window.add(&contents);

        // Button labels (rows)
        let labels = vec![
            vec!["(", ")", "%", "AC"],
            vec!["7", "8", "9", "*"],
            vec!["4", "5", "6", "/"],
            vec!["1", "2", "3", "+"],
            vec!["0", ".", "=", "-"],
        ];

        for (row_idx, row) in labels.iter().enumerate() {
            for (col_idx, &label) in row.iter().enumerate() {
                let button = Button::with_label(label);

                let button_sc = button.style_context();

                match label {
                    "AC" => button_sc.add_class("clear"),
                    "=" => button.set_widget_name("equal"),
                    "+" | "-" | "*" | "/" | "%" => button_sc.add_class("operator"),
                    _ => button_sc.add_class("normal"),
                }

                if let Some(screen) = GScreen::default() {
                    StyleContext::add_provider_for_screen(
                        &screen,
                        &css_provider,
                        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                    );
                }

                // Clone Rc for this button
                let input_clone = Rc::clone(&input);
                let out_label_clone = Rc::clone(&out_label);
                let eql = Rc::new(RefCell::new(false));
                let eql_clone = Rc::clone(&eql);
                button.connect_clicked(move |_| {
                    let mut inp = input_clone.borrow_mut();
                    let inpl = out_label_clone.borrow_mut();
                    let mut eql = eql_clone.borrow_mut();
                    match label {
                        "AC" => inp.clear(),
                        "=" => {
                            *eql = true;
                            let res = compute(inp.clone());
                            inpl.set_text(
                                format!(
                                    "{}={}",
                                    inp.as_str(),
                                    match res {
                                        Ok(v) => v.to_string(),
                                        Err(e) => format!("{e}"),
                                    }
                                )
                                .as_str(),
                            );
                        }
                        _ => inp.push_str(label),
                    }
                    println!("[CALC_DEBUG] Current input: {}", *inp);
                    if !*eql {
                        inpl.set_text(inp.as_str());
                    } else {
                        *eql = false;
                    }
                });

                grid.attach(&button, col_idx as i32, row_idx as i32, 1, 1);
            }
        }

        window.show_all();
    });

    app.run();
}
