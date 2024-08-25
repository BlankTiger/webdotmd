class HLJSLanguageDisplayPlugin {
  constructor(options = {}) {
    self.hook = options.hook;
    self.callback = options.callback;
  }

  "after:highlightElement"({ el, text }) {
    let languageClass = Array.from(el.classList).find((cls) =>
      cls.startsWith("language-"),
    );

    let languageKey = languageClass
      ? languageClass.replace("language-", "")
      : "Unknown";
    let languageName =
      languageKey && hljs.getLanguage(languageKey)
        ? hljs.getLanguage(languageKey).name
        : "Unknown";
    let language = languageName + " code";

    let languageDiv = Object.assign(document.createElement("div"), {
      className:
        "bg-gray-850 text-gray-100 border-b border-l text-xs rounded-bl-lg p-1.5 absolute top-0 right-0",
      innerText: language,
    });

    el.insertBefore(languageDiv, el.childNodes[0]);
  }
}
