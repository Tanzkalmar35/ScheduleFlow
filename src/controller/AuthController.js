import { invoke } from "@tauri-apps/api/core";
import { createErrorToast, createSuccessToast } from "../Toast.js";
import { listen } from "@tauri-apps/api/event";

export async function isValidSession() {
    return await invoke("is_valid_session");
}

export async function submitLoginForm(event) {
    event.preventDefault();
    const loginEmail = document.getElementById("login-modal-email"),
        loginPassword = document.getElementById("login-modal-password"),
        rememberMe = document.getElementById("login-modal-remember-me")

    await invoke("attempt_login", {
        email: loginEmail.value,
        password: loginPassword.value,
        remember: rememberMe.checked
    })
        .then(_ => {
            window.location.href = "../../index.html"
        })
        .catch(e => {
            createErrorToast(e);
        })
}

export async function submitSignupForm(event) {
    event.preventDefault();
    const loginUsername = document.getElementById("signup-modal-username"),
        loginEmail = document.getElementById("signup-modal-email"),
        loginPassword = document.getElementById("signup-modal-password"),
        rememberMe = document.getElementById("signup-modal-remember-me")


    await invoke("attempt_signup", {
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
    document.cookie = `jwt=${event.payload}; expires=${expirationDate.toUTCString()}; path=/; ${window.location.protocol === 'https:' ? 'Secure;' : ''} SameSite=Strict`;
})
    .then(m => createSuccessToast(m))
    .catch(e => createErrorToast(e));

export function invalidateJwtCookie() {
    document.cookie = `jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;`;
}
