module.exports = {
    darkMode: 'class',
    mode: "jit",
    content: [
        './src/**/*.{js,ts,jsx,tsx}',
        '../core/src/**/*.{js,ts,jsx,tsx}',
    ],
    presets: [require('../core/tailwind.config.js')],
}
