import {submitLoginForm} from "./controller/auth_controller.js";
import {invoke} from "@tauri-apps/api/tauri";

let loginForm = document.getElementById("login-form");

invoke("set_current_window")
    .then(_ => console.log("WINDOW SET"))
    .catch(e => console.log("Error setting the window: ", e));

loginForm.addEventListener("submit", submitLoginForm)
