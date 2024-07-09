import {submitLoginForm} from "./controller/auth_controller.js";
import {invoke} from "@tauri-apps/api/tauri";

let loginForm = document.getElementById("login-form");

invoke("set_current_window")
    .catch(e => console.log("Error updating the current window: ", e));

loginForm.addEventListener("submit", submitLoginForm)
