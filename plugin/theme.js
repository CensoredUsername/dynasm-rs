var themes = document.getElementById("theme-choices");
var themePicker = document.getElementById("theme-picker");

function showThemeButtonState() {
    themes.style.display = "block";
    themePicker.style.borderBottomRightRadius = "0";
    themePicker.style.borderBottomLeftRadius = "0";
}

function hideThemeButtonState() {
    themes.style.display = "none";
    themePicker.style.borderBottomRightRadius = "3px";
    themePicker.style.borderBottomLeftRadius = "3px";
}

function switchThemeButtonState() {
    if (themes.style.display === "block") {
        hideThemeButtonState();
    } else {
        showThemeButtonState();
    }
};

function handleThemeButtonsBlur(e) {
    var active = document.activeElement;
    var related = e.relatedTarget;

    if (active.id !== "themePicker" &&
        (!active.parentNode || active.parentNode.id !== "theme-choices") &&
        (!related ||
         (related.id !== "themePicker" &&
          (!related.parentNode || related.parentNode.id !== "theme-choices")))) {
        hideThemeButtonState();
    }
}

themePicker.onclick = switchThemeButtonState;
themePicker.onblur = handleThemeButtonsBlur;
["dark","light"].forEach(function(item) {
    var but = document.createElement('button');
    but.innerHTML = item;
    but.onclick = function(el) {
        switchTheme(currentTheme, mainTheme, item, true);
    };
    but.onblur = handleThemeButtonsBlur;
    themes.appendChild(but);
});