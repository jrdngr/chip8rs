const path = require("path");

module.exports = {
  entry: './bootstrap.js',
  output: {
    filename: 'bootstrap.js',
    path: path.resolve(__dirname, 'dist')
  },
  module: {
    rules: [
      {
        test: /\.worker\.js$/,
        use: { loader: 'worker-loader' }
      }
    ]
  },
  mode: 'development'
};
