const path = require("path");

module.exports = {
  entry: './js/index.js',
  output: {
    filename: 'bootstrap.js',
    path: path.resolve(__dirname, 'dist')
  },
  mode: 'development'
};