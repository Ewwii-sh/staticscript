use crate::grammar::{DslParser, Rule};
use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;

use super::containers::transpile_container;
use super::props_only::transpile_props_only;

use std::collections::HashSet;

pub fn transpile_code(dsl_code: &str) -> Result<String, Error<Rule>> {
    let pairs = DslParser::parse(Rule::program, dsl_code)?;
    let mut result = String::new();

    for pair in pairs {
        let transpiled = transpile_pair(pair)?;
        if !transpiled.trim().is_empty() {
            result.push_str(&transpiled);
            result.push('\n');
        }
    }
    Ok(result)
}

pub fn transpile_pair(pair: Pair<Rule>) -> Result<String, Error<Rule>> {
    match pair.as_rule() {
        Rule::program => {
            let mut windows = vec![];
            let mut widgets = vec![];
            let mut window_names = HashSet::new();
            let mut widget_names = HashSet::new();

            for inner in pair.into_inner() {
                if inner.as_rule() == Rule::statement {
                    for stmt_inner in inner.into_inner() {
                        match stmt_inner.as_rule() {
                            Rule::widget_def => {
                                let widget_name = stmt_inner.clone().into_inner().next().unwrap().as_str().trim_matches('"');
                                if !widget_names.insert(widget_name.to_string()) {
                                    // Create a Pest error from the span
                                    let span = stmt_inner.as_span();
                                    return Err(Error::new_from_span(
                                        pest::error::ErrorVariant::CustomError {
                                            message: format!("Duplicate widget name: {}", widget_name)
                                        },
                                        span
                                    ));
                                }
                                widgets.push(transpile_pair(stmt_inner)?);
                            }
                            Rule::window_def => {
                                let window_name = stmt_inner.clone().into_inner().next().unwrap().as_str().trim_matches('"');
                                if !window_names.insert(window_name.to_string()) {
                                    // Create a Pest error from the span
                                    let span = stmt_inner.as_span();
                                    return Err(Error::new_from_span(
                                        pest::error::ErrorVariant::CustomError {
                                            message: format!("Duplicate window name: {}", window_name)
                                        },
                                        span
                                    ));
                                }
                                windows.push(transpile_pair(stmt_inner)?);
                            }
                            _ => {}
                        }
                    }
                }
            }

            let mut result = String::new();
            for widget in widgets {
                result.push_str(&widget);
                result.push('\n');
            }
            if !windows.is_empty() {
                result.push_str("enter([\n");
                for w in windows {
                    result.push_str(&w);
                    result.push('\n');
                }
                result.push_str("]);\n");
            }
            Ok(result)
        }

        Rule::window_def => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str();
            let map_entries = inner.next().unwrap();
            let widget_fn_call = inner.next().unwrap().as_str().trim_matches('"');

            let entries: Vec<String> = map_entries
                .into_inner()
                .map(|p| transpile_pair(p))
                .collect::<Result<_, _>>()?;


            Ok(format!(
                "defwindow({}, #{{ {} }}, {}())",
                name,
                entries.join(", "),
                widget_fn_call
            ))
        }

        Rule::widget_def => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().trim_matches('"');
            let box_def = inner.next().unwrap();
            let box_content = transpile_pair(box_def)?;

            Ok(format!("fn {}() {{\n    return {};\n}}", name, box_content))
        }

        // Containers
        Rule::box_def => transpile_container(pair, "box"),
        Rule::center_box_def => transpile_container(pair, "centerbox"),
        Rule::expander_def => transpile_container(pair, "expander"),
        Rule::revealer_def => transpile_container(pair, "revealer"),
        Rule::scroll_def => transpile_container(pair, "scroll"),
        Rule::overlay_def => transpile_container(pair, "overlay"),
        Rule::stack_def => transpile_container(pair, "stack"),
        Rule::event_box_def => transpile_container(pair, "eventbox"),
        Rule::tooltip_def => transpile_container(pair, "tooltip"),

        // Props-only widgets
        Rule::label_def => transpile_props_only(pair, "label"),
        Rule::button_def => transpile_props_only(pair, "button"),
        Rule::image_def => transpile_props_only(pair, "image"),
        Rule::input_def => transpile_props_only(pair, "input"),
        Rule::progress_def => transpile_props_only(pair, "progress"),
        Rule::combo_box_text_def => transpile_props_only(pair, "comboboxtext"),
        Rule::slider_def => transpile_props_only(pair, "slider"),
        Rule::checkbox_def => transpile_props_only(pair, "checkbox"),
        Rule::calendar_def => transpile_props_only(pair, "calendar"),
        Rule::color_button_def => transpile_props_only(pair, "colorbutton"),
        Rule::color_chooser_def => transpile_props_only(pair, "colorchooser"),
        Rule::circular_progress_def => transpile_props_only(pair, "circularprogress"),
        Rule::graph_def => transpile_props_only(pair, "graph"),
        Rule::transform_def => transpile_props_only(pair, "transform"),

        Rule::all_widgets => {
            let entries = pair
                .into_inner()
                .map(|p| transpile_pair(p))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(entries.join("\n"))
        }


        // Map entry
        Rule::map_entry => {
            let mut inner = pair.into_inner();
            let key = inner.next().unwrap().as_str();
            let value_pair = inner.next().unwrap();

            fn render_value(p: Pair<Rule>) -> String {
                let p = if p.as_rule() == Rule::value {
                    p.into_inner().next().unwrap()
                } else {
                    p
                };

                match p.as_rule() {
                    Rule::nested_map => {
                        let entries = p
                            .into_inner()
                            .flat_map(|map_entry_list| map_entry_list.into_inner())
                            .map(|map_entry_pair| {
                                let mut kv_inner = map_entry_pair.into_inner();
                                let k = kv_inner.next().unwrap().as_str();
                                let v = kv_inner.next().unwrap();
                                format!("{}: {}", k, render_value(v))
                            })
                            .collect::<Vec<_>>();
                        format!("#{{ {} }}", entries.join(", "))
                    }

                    Rule::list => {
                        let items = p.into_inner().map(render_value).collect::<Vec<_>>();
                        format!("[{}]", items.join(", "))
                    }
                    _ => p.as_str().to_string(),
                }
            }

            Ok(format!("{}: {}", key, render_value(value_pair)))
        }

        _ => Ok(String::new()),
    }
}
