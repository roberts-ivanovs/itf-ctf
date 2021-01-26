import React, { ReactElement } from 'react';
import { NavLink } from 'react-router-dom';

import style from './Base.module.scss';

export function Navbar(): ReactElement {
  return (
    <div className={`${style['navbar-wrapper']}`}>
      <NavLink
        exact
        to="/"
      >
        Home
      </NavLink>
      <NavLink
        exact
        to="/score"
      >
        Score
      </NavLink>
    </div>
  );
}
