import React, { ReactElement } from 'react';
import { Footer } from './components/Base/Footer';
import { Header } from './components/Base/Header';
import { Main } from './components/Base/Main';

export function App(): ReactElement {
  return (
    <>
      <Header />
      <Main />
      <Footer />
    </>
  );
}
