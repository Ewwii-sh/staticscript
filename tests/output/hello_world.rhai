fn widget1() {	
    return box(#{
		 class: "widget1",
		 orientation: "h",
		 space_evenly: true,
		 hexpand: true,
		 spacing: 5 
	},
	 [		label(#{
			 text: "Hello World!" 
		})
	]);
	

}

enter([	
defwindow("main_window",
	 #{
		 monitor: 0,
		 windowtype: "dock",
		 geometry: #{
			 x: "0",
			 y: "0",
			 width: "20px",
			 height: "20px" 
		} 
	},
	 widget1())

]);


