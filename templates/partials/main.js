setTheme(getTheme(), false);

function setTheme(theme, save = true) {
  if (save) localStorage.setItem("theme", theme);
  document.documentElement.className = "theme-" + theme;

  const content = document.getElementById("content");
  if (content) content.className = "bg-" + theme;

  const selectTheme = document.getElementById("select-theme");
  if (selectTheme) selectTheme.value = theme;

  const accentColor = window
    .getComputedStyle(document.documentElement)
    .getPropertyValue("--accent");
  const themeColor = document.querySelector('meta[name="theme-color"]');
  if (themeColor) themeColor.setAttribute("content", accentColor);
}

function getTheme() {
  const theme = localStorage.getItem("theme");
  if (!theme) {
    return "evergarden";
  }

  const selectTheme = document.getElementById("select-theme");
  if (selectTheme && selectTheme.options) {
    if (Array.from(selectTheme.options).some((option) => option.innerText === theme)) {
      return theme;
    }
  } else {
    return theme;
  }

  return "evergarden";
}

function setDynamicClasses() {
  document.querySelectorAll(".target-blank").forEach((link) => {
    link.setAttribute("target", "_blank");
  });
}

document.addEventListener("DOMContentLoaded", function () {
  const selectTheme = document.getElementById("select-theme");

  setTheme(getTheme(), false);

  selectTheme.addEventListener("change", (e) => {
    setTheme(e.target.value);
  });

  document.querySelectorAll(".sidebar-link").forEach((link) => {
    if (link.href === window.location.href) {
      link.classList.add("active");
    }
  });

  setDynamicClasses();
});
