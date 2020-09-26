use yew::prelude::*;


pub fn add () -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" style="stroke: currentColor; stroke-linecap: square; stroke-linejoin: round; stroke-width: 32px;">
			<line x1="256" y1="112" x2="256" y2="400" />

			<line x1="400" y1="256" x2="112" y2="256" />
		</svg>
	}
}

pub fn back () -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" style="fill: none; stroke: currentColor; stroke-linecap: square; stroke-miterlimit: 10; stroke-width: 48px;">
			<polyline points="244 400 100 256 244 112" />

			<line x1="120" y1="256" x2="412" y2="256" />
		</svg>
	}
}

pub fn link () -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" style="fill: none; stroke: currentColor; stroke-linecap: square; stroke-linejoin: round; stroke-width: 48px;">
			<path d="M200.66,352H144a96,96,0,0,1,0-192h55.41" />

			<path d="M312.59,160H368a96,96,0,0,1,0,192H311.34" />

			<line x1="169.07" y1="256" x2="344.93" y2="256" />
		</svg>
	}
}

pub fn open () -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
			<polygon points="201.37 288 377.37 112 48 112 48 464 400 464 400 134.63 224 310.63 201.37 288" />

			<polygon points="320 48 320 80 409.37 80 377.37 112 400 134.63 432 102.63 432 192 464 192 464 48 320 48" />
		</svg>
	}
}

pub fn settings () -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
			<path d="M256,176a80,80,0,1,0,80,80A80.24,80.24,0,0,0,256,176Zm172.72,80a165.53,165.53,0,0,1-1.64,22.34l48.69,38.12a11.59,11.59,0,0,1,2.63,14.78l-46.06,79.52a11.64,11.64,0,0,1-14.14,4.93l-57.25-23a176.56,176.56,0,0,1-38.82,22.67l-8.56,60.78A11.93,11.93,0,0,1,302.06,486H209.94a12,12,0,0,1-11.51-9.53l-8.56-60.78A169.3,169.3,0,0,1,151.05,393L93.8,416a11.64,11.64,0,0,1-14.14-4.92L33.6,331.57a11.59,11.59,0,0,1,2.63-14.78l48.69-38.12A174.58,174.58,0,0,1,83.28,256a165.53,165.53,0,0,1,1.64-22.34L36.23,195.54a11.59,11.59,0,0,1-2.63-14.78l46.06-79.52A11.64,11.64,0,0,1,93.8,96.31l57.25,23a176.56,176.56,0,0,1,38.82-22.67l8.56-60.78A11.93,11.93,0,0,1,209.94,26h92.12a12,12,0,0,1,11.51,9.53l8.56,60.78A169.3,169.3,0,0,1,361,119L418.2,96a11.64,11.64,0,0,1,14.14,4.92l46.06,79.52a11.59,11.59,0,0,1-2.63,14.78l-48.69,38.12A174.58,174.58,0,0,1,428.72,256Z" />
		</svg>
	}
}

pub fn star () -> Html {
	html! {
		<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
			<path d="M496,203.3H312.36L256,32,199.64,203.3H16L166.21,308.7,107.71,480,256,373.84,404.29,480,345.68,308.7Z" />
		</svg>
	}
}
