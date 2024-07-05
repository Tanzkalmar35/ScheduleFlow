import {submitSignupForm} from "./controller/auth_controller.js";

let signupForm = document.getElementById("signup-form");

signupForm.addEventListener("submit", submitSignupForm)
