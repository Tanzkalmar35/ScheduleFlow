import { submitLoginForm } from "./controller/AuthController.js";
import { invoke } from "@tauri-apps/api/core";

let loginForm = document.getElementById("login-form");

invoke("set_current_window")
    .catch(e => console.log("Error updating the current window: ", e));

loginForm.addEventListener("submit", submitLoginForm)
