module.exports = {
  async rewrites() {
    return [
      {
        source: '/api/:slug*',
        destination: 'http://127.0.0.1:8080/:slug*'
      }
    ]
  }
}
