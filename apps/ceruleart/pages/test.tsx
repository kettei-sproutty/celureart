import type { NextPage } from 'next';
import dynamic from 'next/dynamic';

const BevyComponent = dynamic({
  loader: async () => {
    try {
      const wasm = await import('wasm');
      wasm.Glc.new('test');

      return null;
    } catch (error) {
      console.debug(error);
      return null;
    }
  },
  ssr: false,
});

const TestPage: NextPage = () => {
  return (
    <div>
      <canvas id='test' width={800} height={600} />
      <BevyComponent />
    </div>
  );
};

export default TestPage;
