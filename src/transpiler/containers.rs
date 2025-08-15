use super::core::transpile_pair;
use crate::grammar::Rule;
use pest::iterators::Pair;

pub fn transpile_container(pair: Pair<Rule>, widget_type: &str) -> String {
    let mut map_entries = vec![];
    let mut children = vec![];

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::map_entry_list => {
                for entry in inner.into_inner() {
                    map_entries.push(transpile_pair(entry));
                }
            }
            Rule::map_entry => {
                map_entries.push(transpile_pair(inner));
            }
            Rule::all_widgets | Rule::widget_def => {
                children.push(transpile_pair(inner));
            }
            _ => {}
        }
    }

    format!(
        "{}(#{{ {} }}, [{}])",
        widget_type,
        map_entries.join(", "),
        children.join(", ")
    )
}
