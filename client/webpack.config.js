const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const RqrrWasmPlugin = require('rqrr-wasm/dist/webpack-plugins');

const distPath = path.resolve(__dirname, "../assets");
module.exports = (env, argv) => {
    return {
        devServer: {
            contentBase: distPath,
            compress: argv.mode === 'production',
            port: 8000
        },
        entry: './bootstrap.js',
        output: {
            path: distPath,
            filename: "bundle.js",
            webassemblyModuleFilename: "bundle.wasm"
        },
        module: {
            rules: [
                {
                    test: /\.s[ac]ss$/i,
                    use: [
                        'style-loader',
                        'css-loader',
                        'sass-loader',
                    ],
                },
            ],
        },
        plugins: [
            new CopyWebpackPlugin({
                patterns: [
                    {from: './static', to: distPath},
                ],
            }),
            new WasmPackPlugin({
                crateDirectory: ".",
                extraArgs: "--no-typescript",
            }),
            new RqrrWasmPlugin()
        ],
        watch: argv.mode !== 'production'
    };
};