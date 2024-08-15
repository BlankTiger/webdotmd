/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./output/**/*.{html,js,rs}", "./webdotmd/**/*.{html,js,rs}"],
  darkMode: "selector",
  theme: {
    colors: {
      "d-bg": "#0B0C10",
      "d-bg-secondary": "#111111",
      "d-bg-accent": "#000000",
      "d-text": "#B5B6C7",
      "d-text-accent": "#EEEEEE",
      "d-accent": "#66FCF1",
      "d-accent-secondary": "#45A29E",
      "l-bg": "#F6F6F2",
      "l-bg-secondary": "#C2EDCE",
      "l-bg-accent": "#FAFAFA",
      "l-text": "#111111",
      "l-text-accent": "#010101",
      "l-accent": "#6FB3B8",
      "l-accent-secondary": "#388087",
    },
  },
  plugins: [],
};

// dark theme palette
// #0B0C10
// #1F2833
// #C5C6C7
// #66FCF1
// #45A29E
//
// light theme palette
// #388087
// #6FB3B8
// #BADFE7
// #C2EDCE
// #F6F6F2
