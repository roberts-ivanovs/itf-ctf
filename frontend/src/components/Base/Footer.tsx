import React, { ReactElement } from 'react';

import style from './Base.module.scss';

export function Footer(): ReactElement {
  return (
    <div className={`${style['footer-wrapper']}`}>
      <p>Problēmu gadījumā sazinieties ar robertsivanovs1999@gmail.com</p>
    </div>
  );
}
