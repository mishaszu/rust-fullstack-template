const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = (env, argv) => {
  let distPath = path.resolve(__dirname, "../backend/static");
  if (argv.workspace && argv.workspace == "true") {
    distPath = path.resolve(__dirname, "public");
  }
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000,
      proxy: {
        '/api': 'http://localhost:8081',
        '/test': 'http://localhost:8081'
      },
      historyApiFallback: true
    },
    entry: './index.js',
    output: {
      path: distPath,
      filename: 'app.js',
      webassemblyModuleFilename: 'app.wasm',
      publicPath: '/'
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            {
              loader: MiniCssExtractPlugin.loader,
              options: {
                hmr: argv.mode === 'development',
              },
            },
            'css-loader',
            'sass-loader',
          ],
        },
      ],
    },
    plugins: [
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
      new MiniCssExtractPlugin({
        filename: argv.mode === "development" ? '[name].css' : '[name].[hash].css',
      }),
      new HtmlWebpackPlugin({
        template: './static/index.html'
      })
    ],
    watch: argv.mode !== 'production'
  };
};
