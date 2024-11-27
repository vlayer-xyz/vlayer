import daisyUI from 'daisyui'
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    'node_modules/daisyui/dist/**/*.js',
    'node_modules/react-daisyui/dist/**/*.js',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    daisyUI
  ],
  daisyui: {
    themes: [
      {
        vlayer: {
          primary: "#915bf8",
          secondary: "#3219d7",
          accent: "#00ffff",
          neutral: "#3219d7",
          "base-100": "#050610",
          info: "#0000ff",
          success: "#00ff00",
          warning: "#00ff00",
          error: "#ff0000",
        },
      },
    ],
  },
}