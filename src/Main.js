import { isValidSession } from "./controller/AuthController.js";
import { invoke } from "@tauri-apps/api/tauri";

if (!await isValidSession()) {
    window.location.href = '../src/html/login.html'
}

invoke("set_current_window")
    .catch(e => console.log("Error updating the current window: ", e));
