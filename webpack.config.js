const path = require('path');

module.exports = {
  entry: './src/index.js',
  output: {
    filename: 'index.js',
    path: path.resolve(__dirname, 'dist')
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.rs$/,
        use: [{
            loader: 'babel-loader',
            options: {
              compact: true,
            }
          },
          {
            loader: 'rust-native-wasm-loader',
            options: {
              release: true,
              gc: true,
              wasmBindgen: {
                debug: false
              },
            }
          }]
      }
    ]
  }
};