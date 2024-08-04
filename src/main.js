import { isValidSession } from "./controller/auth_controller.js";
import { invoke } from "@tauri-apps/api/tauri";

if (!await isValidSession()) {
    console.log("Well well well, no valid session...")
    window.location.href = '../src/html/login.html'
}

invoke("set_current_window")
    .catch(e => console.log("Error updating the current window: ", e));
