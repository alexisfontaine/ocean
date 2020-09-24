use ocean::{Anchor, Stack};
use std::lazy::SyncLazy;
use web_sys::{window, ScrollBehavior, ScrollToOptions};
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::agent::RouteRequest;

use super::route::{ApplicationAnchor, ApplicationRoute};


pub enum Message {
	Navigation(Route),
}


pub struct Application {
	router: RouteAgentBridge,
	route: Option<ApplicationRoute>,
}


impl Component for Application {
	type Message = Message;

	type Properties = ();


	fn create (_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut router = RouteAgentBridge::new(link.callback(Self::Message::Navigation));

		router.send(RouteRequest::GetCurrentRoute);

		Application {
			router,
			route: None,
		}
	}

	fn update (&mut self, message: Self::Message) -> ShouldRender {
		match message {
			Self::Message::Navigation(route) => {
				let route = ApplicationRoute::switch(route.clone()).or_else(|| {
					let redirection = ApplicationRoute::redirect(route);

					self.router.send(RouteRequest::ReplaceRouteNoBroadcast(redirection.clone().into()));
					Some(redirection)
				});

				if route != self.route {
					self.route = route;
					SCROLL_OPTIONS.with(|options| window().unwrap().scroll_with_scroll_to_options(options));

					return true
				}
			}
		}

		false
	}

	fn change (&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn view (&self) -> Html {
		let in_presentation_mode = self.route.as_ref().map(ApplicationRoute::in_presentation_mode).unwrap_or_default();

		html! {
			<>
				<header class="application__header" data-stretched=in_presentation_mode data-translucent=in_presentation_mode>
					<ApplicationAnchor route=ApplicationRoute::Home>
						<img class="application__logo" src="/logo.svg" alt="Ocean" />
					</ApplicationAnchor>

					<nav>
						<ApplicationAnchor route=ApplicationRoute::Documentation>{"Documentation"}</ApplicationAnchor>
						<ApplicationAnchor route=ApplicationRoute::Components>{"Components"}</ApplicationAnchor>
						<ApplicationAnchor route=ApplicationRoute::About>{"About"}</ApplicationAnchor>
					</nav>

					<Anchor to="https://github.com/alexisfontaine/ocean" open_in_new_context=true>
						{"GitHub"}
						<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
							<path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" />
						</svg>
					</Anchor>
				</header>

				{for self.route.as_ref().map(ApplicationRoute::render)}

				<Stack tag="footer" class="application__footer">
					<nav />
				</Stack>
			</>
		}
	}
}


thread_local! {
	static SCROLL_OPTIONS: SyncLazy<ScrollToOptions> = SyncLazy::new(|| {
		let mut options = ScrollToOptions::new();

		options.top(0.).left(0.).behavior(ScrollBehavior::Smooth);
		options
	});
}
