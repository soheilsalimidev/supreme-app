/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {},
    fontFamily: {
      sans: []
    }
  },
  plugins: [require('@tailwindcss/forms'), plugin(function({ matchUtilities, theme }) {
    matchUtilities(
      {
        'bg-gradient': (angle) => ({
          'background-image': `linear-gradient(${angle}, var(--tw-gradient-stops))`,
        }),
      },
      {
        values: Object.assign(
          theme('bgGradientDeg', {}),           {
            135: '135deg',          }
        )
      }
    )
  })],
}

