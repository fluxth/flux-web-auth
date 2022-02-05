const webpack = require("webpack");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const path = require("path");
const directories = {
  dist: path.resolve(__dirname, "../static"),
  public: path.resolve(__dirname, "public"),
  prototype: path.resolve(__dirname, "prototype"),
};

const DEBUG = process.env.NODE_ENV === "development" ? true : false;
console.log("DEBUG mode:", DEBUG);

module.exports = {
  mode: DEBUG ? "development" : "production",
  entry: {
    main: "./src",
  },
  output: {
    filename: "js/fluxauth-[name].min.js",
    path: directories.dist,
    uniqueName: "fluxauth",
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "css/fluxauth-[name].min.css",
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: directories.public,
          noErrorOnMissing: true,
          globOptions: { dot: false },
        },
      ],
    }),
    new webpack.DefinePlugin({
      PRODUCTION: !DEBUG,
    }),
  ],
  optimization: {
    minimize: !DEBUG,
    minimizer: [new CssMinimizerPlugin(), new TerserPlugin()],
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
      },
      {
        test: /\.s[ac]ss$/i,
        use: [
          MiniCssExtractPlugin.loader,
          "css-loader",
          "postcss-loader",
          {
            loader: "sass-loader",
            options: {
              sourceMap: DEBUG,
              sassOptions: {
                outputStyle: "expanded",
                quietDeps: true,
              },
            },
          },
        ],
      },
      {
        test: /\.(woff(2)?|ttf|eot|svg)(\?v=\d+\.\d+\.\d+)?$/,
        type: "asset/resource",
        generator: {
          filename: "fonts/[hash][ext][query]",
        },
      },
      {
        test: /images\/.*\.(svg)(\?v=\d+\.\d+\.\d+)?$/,
        type: "asset/resource",
        generator: {
          filename: "img/[hash][ext][query]",
        },
      },
    ],
  },
  devServer: {
    static: {
      directory: directories.prototype,
    },
    headers: {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, PATCH, OPTIONS",
      "Access-Control-Allow-Headers":
        "X-Requested-With, content-type, Authorization",
    },
    allowedHosts: "all",
    compress: false,
    port: 9000,
  },
};
