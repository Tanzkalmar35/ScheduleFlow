import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import {createErrorToast, createSuccessToast} from "./toast.js";

let loginForm = document.getElementById("login-form");

loginForm.addEventListener("submit", submitLoginForm)

export async function submitLoginForm(event) {
    event.preventDefault();
    const loginUsername = document.getElementById("login-modal-username"),
        loginEmail = document.getElementById("login-modal-email"),
        loginPassword = document.getElementById("login-modal-password"),
        rememberMe = document.getElementById("login-modal-remember-me")


    await invoke("attempt_login", {
        username: loginUsername.value,
        email: loginEmail.value,
        password: loginPassword.value,
        remember: rememberMe.checked
    })
        .then(_ => {
            window.location.href = "../index.html";
        })
        .catch(e => createErrorToast(e))
}

listen('setJwtCookie', (event) => {
    let expirationDate = new Date();
    expirationDate.setMonth(expirationDate.getMonth() + 1, expirationDate.getDay());
    document.cookie = `jwt=${event.payload}; expires=${expirationDate.toUTCString()}; path=/; Secure; SameSite=Strict`;
})
    .then(m => createSuccessToast(m))
    .catch(e => createErrorToast(e));
