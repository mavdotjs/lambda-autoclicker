import daisyui from "daisyui"
import themes from "daisyui/src/theming/themes"
import { fontFamily } from "tailwindcss/defaultTheme"

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        appbartop: ['"Bebas Neue"', ...fontFamily.sans],
        sans: ['"Bricolage Grotesque"', ...fontFamily.sans]
      }
    },
  },
  plugins: [daisyui],
  daisyui: {
    themes: [
      {
        lambda: {
          ...themes.black,
          "primary": "#7410f3",
          "secondary": "#2a7faf",
          "--rounded-box": "0.5rem",
          "--rounded-btn": "0.25rem",
          "--animation-btn": undefined,
          "--animation-input": undefined,      
        },
      },
    ],
    base: false,
    utils: true,
    logs: true
  }
}

