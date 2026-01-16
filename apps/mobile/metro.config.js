const { getDefaultConfig } = require('expo/metro-config');
const path = require('path');

const projectRoot = __dirname;
const monorepoRoot = path.resolve(projectRoot, '../..');

const config = getDefaultConfig(projectRoot);

// Watch the monorepo packages
config.watchFolders = [monorepoRoot];

// Let Metro know where to resolve packages
config.resolver.nodeModulesPaths = [
	path.resolve(projectRoot, 'node_modules'),
	path.resolve(monorepoRoot, 'node_modules'),
];

// Disable package haste map for packages folder
config.resolver.disableHierarchicalLookup = true;

module.exports = config;
