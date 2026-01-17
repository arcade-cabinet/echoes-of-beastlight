import React from 'react';
import renderer from 'react-test-renderer';
import { View } from 'react-native';

describe('mobile placeholder', () => {
  it('renders correctly', () => {
    const tree = renderer.create(<View />).toJSON();
    expect(tree).toBeTruthy();
    expect(tree).toMatchObject({ type: 'View' });
  });
});
