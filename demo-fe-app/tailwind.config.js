/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        "./src/**/*.{rs,html,css}",
        "./dist/**/*.html",
        "./dist/**/*.js",
    ],
    safelist: [
        "dropdown",
        "dropdown-hover",
        "dropdown-content",
        "btn",
        "btn-sm",
        "btn-outline",
        "btn-primary",
        "m-1",
        "menu",
        "w-52",
        "bg-base-100",
        "rounded-box",
        "z-1", 
        "p-2", 
        "shadow-sm",
    ],
    darkMode: "class",
    theme: {
        extend: {},
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("daisyui")
    ],
    daisyui: {
        themes: ["light ", "dark"],
    },
};
