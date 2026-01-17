import App from '../App';
import renderer from 'react-test-renderer';

describe('mobile placeholder', () => {
	it('renders App component without crashing', () => {
		const tree = renderer.create(<App />);
		const root = tree.root;
		expect(root).toBeDefined();
	});
});
