
function navigateTo(url: string) {
    window.location.href = url;
}

function navigateToIndexPage() {
    navigateTo("/");
}

function navigateToLoginPage() {
    navigateTo("../src/login.html")
}
