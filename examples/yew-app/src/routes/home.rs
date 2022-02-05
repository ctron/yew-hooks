use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="app">
            <header class="app-header">
                <h1>{ "Yew Hooks" }</h1>
                <div class="hooks">
                    <h2>{ "State" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseToggle} classes="app-link" >{ "use_toggle" }</Link<AppRoute>> { " - tracks state of counterparts." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseBoolToggle} classes="app-link">{ "use_bool_toggle" }</Link<AppRoute>> { " - tracks state of a boolean." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseCounter} classes="app-link">{ "use_counter" }</Link<AppRoute>> { " - tracks state of a number." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLatest} classes="app-link">{ "use_latest" }</Link<AppRoute>> { " - returns the latest immutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMutLatest} classes="app-link">{ "use_mut_latest" }</Link<AppRoute>> { " - returns the latest mutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UsePrevious} classes="app-link">{ "use_previous" }</Link<AppRoute>> { " - returns the previous immutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseList} classes="app-link">{ "use_list" }</Link<AppRoute>> { " - tracks state of a list." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMap} classes="app-link">{ "use_map" }</Link<AppRoute>> { " - tracks state of a hash map." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSet} classes="app-link">{ "use_set" }</Link<AppRoute>> { " - tracks state of a hash set." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseQueue} classes="app-link">{ "use_queue" }</Link<AppRoute>> { " - tracks state of a queue." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseRafState} classes="app-link">{ "use_raf_state" }</Link<AppRoute>> { " - creates set method which only updates after requestAnimationFrame." }</li>
                    </ul>

                    <h2>{ "Side-effects" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseAsync} classes="app-link" >{ "use_async" }</Link<AppRoute>> { " - resolves an async future, like fetch REST api." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseTitle} classes="app-link" >{ "use_title" }</Link<AppRoute>> { " - sets title of the page." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLocalStorage} classes="app-link" >{ "use_local_storage" }</Link<AppRoute>> { " - manages a value in localStorage." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSessionStorage} classes="app-link" >{ "use_session_storage" }</Link<AppRoute>> { " - manages a value in sessionStorage." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseBeforeUnload} classes="app-link" >{ "use_before_unload" }</Link<AppRoute>> { " - shows browser alert when user try to reload or close the page." }</li>
                    </ul>

                    <h2>{ "Lifecycles" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseEffectOnce} classes="app-link" >{ "use_effect_once" }</Link<AppRoute>> { " - a modified use_effect hook that only runs once." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMount} classes="app-link">{ "use_mount" }</Link<AppRoute>> { " - calls mount callbacks." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseUnmount} classes="app-link">{ "use_unmount" }</Link<AppRoute>> { " - calls unmount callbacks." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseIsFirstMount} classes="app-link">{ "use_is_first_mount" }</Link<AppRoute>> { " - checks if current render is first." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseIsMounted} classes="app-link">{ "use_is_mounted" }</Link<AppRoute>> { " - tracks if component is mounted." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseEvent} classes="app-link">{ "use_event" }</Link<AppRoute>> { " - subscribe to events." }</li>
                    </ul>

                    <h2>{ "Animations" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseTimeout} classes="app-link" >{ "use_timeout" }</Link<AppRoute>> { " - schedules a timeout to invoke callback." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseInterval} classes="app-link">{ "use_interval" }</Link<AppRoute>> { " - schedules an interval to invoke callback." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseUpdate} classes="app-link">{ "use_update" }</Link<AppRoute>> { " - returns a callback, which re-renders component when called." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseRaf} classes="app-link">{ "use_raf" }</Link<AppRoute>> { " - re-renders component on each requestAnimationFrame." }</li>
                    </ul>

                    <h2>{ "Sensors" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseWindowSize} classes="app-link" >{ "use_window_size" }</Link<AppRoute>> { " - tracks Window dimensions." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseWindowScroll} classes="app-link" >{ "use_window_scroll" }</Link<AppRoute>> { " - tracks Window scroll position." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseScroll} classes="app-link" >{ "use_scroll" }</Link<AppRoute>> { " - tracks an HTML element's scroll position." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseScrolling} classes="app-link" >{ "use_scrolling" }</Link<AppRoute>> { " - tracks whether HTML element is scrolling." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLocation} classes="app-link" >{ "use_location" }</Link<AppRoute>> { " - tracks brower's location value." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseHash} classes="app-link" >{ "use_hash" }</Link<AppRoute>> { " - tracks brower's location hash value." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSearchParam} classes="app-link" >{ "use_search_param" }</Link<AppRoute>> { " - tracks brower's location search param value." }</li>
                    </ul>
                </div>
                <p>
                    { "More is coming.." }
                </p>
                <a
                    class="app-logo"
                    href="https://yew.rs"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                </a>
            </header>
        </div>
    }
}
