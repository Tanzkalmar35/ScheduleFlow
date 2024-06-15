import {isValidSession} from "./controller/auth_controller.js";

if (!await isValidSession()) {
    window.location.href = '../src/html/login.html'
}
