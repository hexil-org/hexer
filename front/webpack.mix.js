const mix = require("laravel-mix");
require("mix-html-builder");

const mixOptions = {
    hmrOptions: {
        host: "localhost",
        port: "9451",
    },
};

const webpackOptions = {
    devtool: "inline-source-map",
};

mix.options(mixOptions)
    .webpackConfig(webpackOptions)
    .setPublicPath("dist")
    .ts("src/index.ts", "assets/js/index.js")
    .html({ htmlRoot: "./src/index.html", output: "", inject: true });

if (mix.inProduction()) {
    mix.disableNotifications();
    mix.version();
}
