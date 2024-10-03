import { loadUserCalendarData } from "./calendar/CalendarUtil.js";
import { isValidSession } from "./controller/AuthController.js";
import { invoke } from "@tauri-apps/api/core";

async function init() {
    if (!(await isValidSession())) {
        window.location.href = "../src/html/login.html";
        return;
    }

    invoke("set_app_handle").catch((e) =>
        console.log("Error updating the app handle: ", e),
    );

    await loadUserCalendarData();

    console.log("Calendar data Loaded");
}

await init()
    .then(() => console.log("Initialized successfully"))
    .catch((e) => console.log("Initialization failed: " + e));
