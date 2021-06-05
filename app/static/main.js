document.getElementById("identity").onclick = function () {
    location.href = "/profile";
};

document.getElementById("scan").onclick = function () {
    location.href = "/scan";
};


if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('static/service_worker.js')
    .then((reg) => console.log('Registered', reg))
    .catch((err) => console.log('Not registered', err))
    };