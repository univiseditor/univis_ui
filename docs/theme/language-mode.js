(function () {
    function configureLanguageMode() {
        var path = window.location.pathname;
        var isArabic = path.indexOf("/ar/") !== -1;
        var isEnglish = path.indexOf("/en/") !== -1;

        if (!isArabic && !isEnglish) {
            return;
        }

        var html = document.documentElement;
        var body = document.body;
        var main = document.querySelector("main");
        var direction = isArabic ? "rtl" : "ltr";
        var language = isArabic ? "ar" : "en";

        html.lang = language;
        html.dir = direction;
        html.classList.toggle("language-ar", isArabic);
        html.classList.toggle("language-en", isEnglish);

        if (body) {
            body.dir = direction;
        }

        if (main) {
            main.dir = direction;

            var switcher = document.createElement("div");
            switcher.className = "language-switch";

            var label = document.createElement("span");
            label.textContent = isArabic ? "اللغة الحالية: العربية" : "Current language: English";
            switcher.appendChild(label);

            var link = document.createElement("a");
            link.href = isArabic ? path.replace("/ar/", "/en/") : path.replace("/en/", "/ar/");
            link.textContent = isArabic ? "English" : "العربية";
            switcher.appendChild(link);

            main.insertBefore(switcher, main.firstChild);
        }
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", configureLanguageMode);
    } else {
        configureLanguageMode();
    }
})();
