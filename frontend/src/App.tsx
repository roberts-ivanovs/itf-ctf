import React, { ReactElement } from 'react';
import { Footer } from './components/Base/Footer';
import { Navbar } from './components/Base/Navbar';
import { Main } from './components/Base/Main';

export function App(): ReactElement {
  return (
    <>
      <Navbar />
      <Main />
      <Footer />
    </>
  );
}
