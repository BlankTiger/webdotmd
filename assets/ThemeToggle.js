let toggle = document.getElementById("themeToggle");
toggle.onclick = (e) => {
  let html = document.getElementById("html");
  if (html.className === "") {
    html.className = "dark";
  } else {
    html.className = "";
  }
  window.localStorage.setItem("theme", html.className);
};
