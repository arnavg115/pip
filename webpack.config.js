const path = require('path');
// const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
    entry: './index.js',
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'dist'),
    },
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ["css-loader"],
            },

            // ...
            // {
            //     test: /\.wasm$/,
            //     loader: 'raw-loader',
            // },
        ],

    },
    experiments: {
        asyncWebAssembly: true
    }
};