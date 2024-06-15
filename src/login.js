import {submitLoginForm} from "./controller/auth_controller.js";

let loginForm = document.getElementById("login-form");

loginForm.addEventListener("submit", submitLoginForm)
