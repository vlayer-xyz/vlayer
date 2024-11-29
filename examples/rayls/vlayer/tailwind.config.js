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
          primary: "#7235E5",
          secondary: "#3219d7",
          accent: "#00ffff",
          neutral: "white",
          "base-100": "white",
          info: "#0000ff",
          success: "#35E572",
          warning: "#00ff00",
          error: "#ff0000",
        },
      },
    ],
  },
}