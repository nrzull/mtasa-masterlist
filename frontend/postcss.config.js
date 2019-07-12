const config = {
  plugins: {
    "postcss-preset-env": {}
  }
};

if (process.env.NODE_ENV === "production") {
  config.plugins.cssnano = {
    preset: "default"
  };
}

module.exports = config;
