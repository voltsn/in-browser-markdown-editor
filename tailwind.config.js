/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "src/**/*.rs",
    "./*.html",
  ],
  theme: {
    colors: {
      black: '#151619',
      gray: '#C1C4CB',
      white: '#FFFFFF',
      "medium-black": '#1D1F22',
      "semi-medium-black": '#2B2D31',
      "light-black": '#2B2D31',
      "very-dark-gray": '#35393F',
      "dark-gray": '#5A6069',
      "medium-gray": '#7C8187',
      "light-gray": '#E4E4E4',
      "white-smoke":  '#F5F5F5',
      "light-orange": '#F39765',
      "dark-orange": '#E46643',
    },
    extend: {
      fontFamily: {
        roboto: ["Roboto", "sans-serif"]
      },
    },
  },
  plugins: [],
}

