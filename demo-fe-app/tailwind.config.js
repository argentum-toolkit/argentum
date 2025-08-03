/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        "./src/**/*.{rs,html,css}",
        "./dist/**/*.html",
        "./dist/**/*.js",
    ],
    safelist: [
        //authbar
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
        //theme toggle
        "swap",
        "swap-rotate",
        "theme-controller",
        "swap-on",
        "swap-off",
        "fill-current",
        "h-10",
        "w-10",
        "h-7",
        "w-7",
        "md:h-14", 
        "md:w-14",
    ],
    darkMode: "class",
    theme: {
        extend: {},
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("daisyui")
    ],
};
