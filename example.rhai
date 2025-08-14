fn widget1() {
    return box(#{ class: "widget1", orientation: "h", space_evenly: true, hexpand: true, spacing: 5 }, [label(#{ text: "Hello Ewwii!" }), slider(#{ min: 0, max: 101, value: volume, onchange: "echo hi! you just triggered the slider", dyn_id: "vol" }), button(#{ text: "greet", onclick: "notify-send 'hello there!'" }), checkbox(#{ class: "chckbox" })]);
}
enter([
defwindow("main_window", #{ monitor: 0, windowtype: "dock", geometry: { x: 0 y: 0 width: 10 height: 20 } }, widget1())
]);

