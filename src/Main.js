import { loadUserCalendarData } from "./calendar/CalendarUtil.js";
import { isValidSession } from "./controller/AuthController.js";
import { invoke } from "@tauri-apps/api/tauri";

async function init() {
    if (!await isValidSession()) {
        window.location.href = '../src/html/login.html'
        return;
    }

    invoke("set_current_window")
        .catch(e => console.log("Error updating the current window: ", e));

    loadUserCalendarData();
}

await init()
    .then(_ => console.log("Initialized successfully"))
    .catch(e => console.log("Initialization failed: " + e))
