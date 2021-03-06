const { DefinePlugin } = require("webpack");
const HtmlPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin");
const ForkTsCheckerPlugin = require("fork-ts-checker-webpack-plugin");
const path = require("path");

const [DEV, PROD] = ["development", "production"];
const { NODE_ENV = DEV } = process.env;
const [IS_DEV, IS_PROD] = [NODE_ENV === DEV, NODE_ENV === PROD];

const SRC = path.resolve(__dirname, "src");
const DIST = path.resolve(__dirname, "dist");

const config = {
  mode: NODE_ENV,
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
    plugins: [new TsconfigPathsPlugin()]
  },
  entry: SRC,
  output: {
    path: DIST
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        include: SRC,
        loader: "ts-loader",
        options: {
          transpileOnly: true
        }
      },

      {
        test: /\.css$/,
        use: [
          MiniCssExtractPlugin.loader,
          { loader: "css-loader", options: { importLoaders: 1 } },
          "postcss-loader"
        ]
      },

      {
        test: /\.(png|ttf)$/,
        include: SRC,
        use: "file-loader"
      },

      {
        test: /\.svg$/,
        include: SRC,
        use: "react-svg-loader"
      }
    ]
  },
  plugins: [
    new ForkTsCheckerPlugin(),
    new HtmlPlugin({
      template: path.resolve(SRC, "index.html"),
      favicon: path.resolve(SRC, "assets", "favicon.png")
    }),
    new MiniCssExtractPlugin(),
    new DefinePlugin({
      "process.env.NODE_ENV": JSON.stringify(NODE_ENV)
    })
  ],
  devServer: {
    stats: {
      modules: false,
      chunks: false,
      children: false
    },
    proxy: {
      "/api": "http://localhost:8081"
    }
  }
};

module.exports = config;
