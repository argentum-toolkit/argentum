/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./assets/login/*.ftl",
        "./assets/login/theme.properties",
    ],
    safelist: [

    ],
    theme: {
        extend: {},
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("daisyui"),
    ],
};
