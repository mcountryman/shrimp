const withCss = require("@zeit/next-css");
const withLess = require("@zeit/next-less");

module.exports = withCss(withLess({
  webpack(config, options) {
    return config;
  }
}));