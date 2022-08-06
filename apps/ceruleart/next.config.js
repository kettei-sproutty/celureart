const nextTranslate = require('next-translate')
const withPlugins = require('next-compose-plugins');
const withTM = require('next-transpile-modules')(['wasm']);

/**
 * @type {import('next').NextConfig}
 */
const config = {
  webpack(config) {
    config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm'

    // Since Webpack 5 doesn't enable WebAssembly by default, we should do it manually
    config.experiments = { ...config.experiments, asyncWebAssembly: true }

    return config
  },
}

module.exports = withPlugins([nextTranslate, withTM], config);
