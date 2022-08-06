import type { AppProps } from 'next/app';
import '../styles/globals.css';

/**
 * @fixes https://github.com/aralroca/next-translate/issues/486
 */
// eslint-disable-next-line import/no-anonymous-default-export, react/display-name
export default ({ Component, pageProps }: AppProps) => {
  return <Component {...pageProps} />;
};
