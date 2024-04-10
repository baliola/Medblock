/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./App.{js,jsx,ts,tsx}', './src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: {
          normal: '#242DA8',
          light: '#7178E1',
        },
        secondary: {
          normal: '#D30837',
          light: '#FED8E0',
        },
      },
    },
  },
  plugins: [],
};
