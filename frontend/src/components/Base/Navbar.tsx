import React, { ReactElement } from 'react';

import style from './Base.module.scss';

export function Navbar(): ReactElement {
  return (
    <div className={`${style['navbar-wrapper']}`} />
  );
}
