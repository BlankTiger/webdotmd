let theme = window.localStorage.getItem("theme");
if (theme !== null) {
  let html = document.getElementById("html");
  html.className = theme;
  if (theme === "dark") {
    let toggle = document.getElementById("themeToggle");
    toggle.toggleAttribute("checked");
  }
}
