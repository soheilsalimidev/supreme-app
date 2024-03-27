/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {},
    fontFamily: {
      'sans': ['AnjomanMaxVF', {
          fontVariationSettings: '"wght" 400'
        },],
    }
  },
  plugins: [require('@tailwindcss/forms')],
}

