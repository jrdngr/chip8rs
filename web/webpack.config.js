const path = require("path");

module.exports = {
  entry: './bootstrap.js',
  output: {
    filename: 'bootstrap.js',
    path: path.resolve(__dirname, 'dist')
  },
  mode: 'development'
};
