import { View } from 'react-native';
import renderer from 'react-test-renderer';

describe('mobile placeholder', () => {
	it('renders correctly', () => {
		const tree = renderer.create(<View />).toJSON();
		expect(tree).toBeTruthy();
		expect(tree).toMatchObject({ type: 'View' });
	});
});
