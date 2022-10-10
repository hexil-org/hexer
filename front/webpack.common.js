const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: "./src/index.ts",
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: "ts-loader",
                include: path.resolve(__dirname, "src/"),
            },
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader"],
                include: path.resolve(__dirname, "src/"),
            },
        ],
    },
    plugins: [
        new HtmlWebpackPlugin({ template: "./src/index.html" }),
        new CopyWebpackPlugin({
            patterns: [{ from: "./assets", to: "assets" }],
        }),
    ],
    resolve: {
        extensions: [".ts", ".js", ".json"],
    },
    output: {
        filename: "main.js",
        path: path.resolve(__dirname, "dist"),
        clean: true,
    },
};
