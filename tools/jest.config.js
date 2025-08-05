module.exports = {
  displayName: 'tools',
  preset: '../jest.preset.js',
  testEnvironment: 'node',
  transform: {
    '^.+\\.js$': ['babel-jest', { presets: ['@babel/preset-env'] }],
  },
  moduleFileExtensions: ['js', 'json'],
  coverageDirectory: '../coverage/tools',
};