use leptos::IntoView;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    let nav_link_class = "grid w-full justify-items-center gap-1 py-2 text-sm font-medium text-gray-400 rounded-xl transition-all duration-200 hover:text-indigo-600 aria-[current=page]:text-indigo-700 aria-[current=page]:font-semibold aria-[current=page]:bg-indigo-50 lg:aria-[current=page]:border-t-0 lg:flex lg:w-full lg:items-center lg:justify-start lg:gap-4 lg:rounded-xl lg:px-4 lg:py-3 lg:text-base lg:hover:bg-indigo-50 lg:aria-[current=page]:bg-indigo-100 lg:aria-[current=page]:shadow-sm";

    view! {
        <nav class = "fixed bottom-0 right-0 left-0 border-t border-gray-200 bg-white shadow-lg lg:sticky lg:top-0 lg:text-left lg:w-[17vw] lg:min-w-fit lg:h-screen lg:self-start lg:border-t-0 lg:border-r lg:py-6 lg:px-4 lg:shadow-md">

            <div class="gap-2 mb-4 hidden lg:flex lg:pl-4 lg:p-3 lg:items-center lg:mb-6">
                <img class="w-auto h-10 rounded-full" src="/assets/logo.png" />
                <img class="w-auto h-8" src="/assets/logo-text.png" alt="Merzah <logo>" />
            </div>

            <ul class="flex justify-around text-foreground p-2 lg:flex-col lg:items-start lg:gap-2 lg:w-full lg:p-0">
                <li class="w-full">
                    <A
                        href="/home"
                        attr:class=nav_link_class
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5 lg:w-6 lg:h-6">
                            <path d="M3 10.5l9-7.5 9 7.5v9.5a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-9.5z"/>
                            <path d="M10 22v-6a2 2 0 1 1 4 0v6"/>
                        </svg>
                        <span>Home</span>
                    </A>
                </li>

                <li class="w-full">
                    <A
                        href="/mosques"
                        attr:class=nav_link_class
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5 lg:w-6 lg:h-6">
                            <path d="M12 8c-2.2 0-4 1.8-4 4v10h8V12c0-2.2-1.8-4-4-4z"/>
                            <path d="M10 22v-4a2 2 0 1 1 4 0v4"/>
                            <path d="M5 22V12H4l2-3 2 3H7v10"/>
                            <path d="M19 22V12h1l-2-3-2 3h1v10"/>
                            <path d="M12 4v4"/>
                            <path d="M12 2a1.5 1.5 0 0 0 0 3"/>
                            <path d="M2 22h20"/>
                        </svg>
                        <span>Mosques</span>
                    </A>
                </li>

                <li class="w-full">
                    <A
                        href="/learn"
                        attr:class=nav_link_class
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5 lg:w-6 lg:h-6">
                            <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/>
                            <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/>
                            <path d="M12 7v14"/>
                        </svg>
                        <span>Learn</span>
                    </A>
                </li>

                <li class="w-full">
                    <A
                        href="/events"
                        attr:class=nav_link_class
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5 lg:w-6 lg:h-6">
                            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                            <line x1="16" y1="2" x2="16" y2="6"></line>
                            <line x1="8" y1="2" x2="8" y2="6"></line>
                            <line x1="3" y1="10" x2="21" y2="10"></line>
                            <path d="m12 13 1 2 2 1-2 1-1 2-1-2-2-1 2-1z"></path>
                        </svg>
                        <span>Events</span>
                    </A>
                </li>
            </ul>
        </nav>
    }
}
