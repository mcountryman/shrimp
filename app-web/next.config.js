const withCss = require("@zeit/next-css");
const withLess = require("@zeit/next-less");
const withImages = require("next-images");

module.exports = withImages(
  withLess({
    paths: ["src/theme"],
    cssModules: true,
    cssLoaderOptions: {
      auto: /^\.\/src\/theme/,
      paths: ["src/theme"],
      minimize: true,
      sourceMap: true,
      importLoaders: 1,
      localIdentName: "[local]-[hash:base64:5]",
      postcssLoaderOptions: {
        plugins: [require("cssnano")({ preset: "advanced" })],
      },
    },
  })
);
