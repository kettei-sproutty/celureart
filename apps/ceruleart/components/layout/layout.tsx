import Head from 'next/head';
import { Fragment } from 'react';
import type { FC, PropsWithChildren } from 'react';
import styles from './layout.module.css';

const Layout: FC<PropsWithChildren> = ({ children }) => {
  return (
    <Fragment>
      <Head>
        <title>Celureart</title>
      </Head>
      <header className={styles['header']} />
      <main className={styles['main']}>{children}</main>
      <footer className={styles['footer']} />
    </Fragment>
  );
};

export default Layout;
