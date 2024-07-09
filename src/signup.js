import {submitSignupForm} from "./controller/auth_controller.js";
import {invoke} from "@tauri-apps/api/tauri";

let signupForm = document.getElementById("signup-form");

signupForm.addEventListener("submit", submitSignupForm)

invoke("set_current_window")
    .catch(e => console.log("Error updating the current window: ", e));
