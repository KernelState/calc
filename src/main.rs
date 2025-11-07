mod lexer;
mod parser;

use gtk::{Application, ApplicationWindow, Button, Grid, Label};
use gtk::{Box as GBox, prelude::*};
use lexer::Lexer;
use parser;
use std::cell::RefCell;
use std::rc::Rc;

fn compute(input: String) -> f64 {
    let mut lx = Lexer::from_string(input);
    lx.lex();
    let toks = lx.toks;
    // let mut prsr = parser::Eqt::from_toks(toks);
    // prsr.parse();
    // let eqt = prsr.eqt;
    // if let Some(val) = eqt.value {
    //     return val;
    // }
    // TODO: Implement proper computation parsing, I'm going outside for now
    return 0 as f64;
}

fn main() {
    let app = Application::builder()
        .application_id("org.example.calculator")
        .build();

    app.connect_activate(|app| {
        // Shared input
        let input = Rc::new(RefCell::new(String::new()));

        // Create window
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(200)
            .default_height(300)
            .title("TESTING")
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
        grid.set_row_spacing(5);
        grid.set_column_spacing(5);

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

                // Clone Rc for this button
                let input_clone = Rc::clone(&input);
                let out_label_clone = Rc::clone(&out_label);
                let mut eql = Rc::new(RefCell::new(false));
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
                            inpl.set_text(format!("{}={}", inp.as_str(), res).as_str());
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
