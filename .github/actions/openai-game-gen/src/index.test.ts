import * as core from '@actions/core';
import * as fs from 'fs/promises';
import { interpolatePrompt } from './index';

// Mock modules
jest.mock('@actions/core');
jest.mock('fs/promises');

describe('OpenAI Game Gen Action', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('interpolatePrompt', () => {
    it('should interpolate config values', () => {
      const template = 'Generate a {config.game.title} with {config.theme.setting}';
      const config = {
        game: { title: 'Test Game' },
        theme: { setting: 'fantasy world' },
      };
      const params = {};

      const result = interpolatePrompt(template, config, params);
      expect(result).toBe('Generate a Test Game with fantasy world');
    });

    it('should interpolate direct parameters', () => {
      const template = 'Zone: {zone_name}, Size: {map_size}';
      const config = {};
      const params = {
        zone_name: 'Verdant Flats',
        map_size: '12x12',
      };

      const result = interpolatePrompt(template, config, params);
      expect(result).toBe('Zone: Verdant Flats, Size: 12x12');
    });

    it('should handle missing values gracefully', () => {
      const template = 'Missing: {config.missing.value} and {unknown}';
      const config = {};
      const params = {};

      const result = interpolatePrompt(template, config, params);
      expect(result).toBe('Missing: {config.missing.value} and {unknown}');
    });
  });

  describe('parseGeneratedContent', () => {
    it('should parse JSON content', () => {
      const content = '{"key": "value"}';
      const result = parseGeneratedContent(content, 'json');
      expect(result).toEqual({ key: 'value' });
    });

    it('should extract Rust code from markdown', () => {
      const content = 'Some text\n```rust\nfn main() {}\n```\nMore text';
      const result = parseGeneratedContent(content, 'rust');
      expect(result).toBe('fn main() {}');
    });

    it('should return raw content for unknown formats', () => {
      const content = 'Raw content';
      const result = parseGeneratedContent(content, 'unknown');
      expect(result).toBe('Raw content');
    });
  });
});