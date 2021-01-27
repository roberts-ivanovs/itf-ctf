import React, { ReactElement } from 'react';
import { NavLink } from 'react-router-dom';

import style from '../Base/Base.module.scss';

export function AdminNavbar(): ReactElement {
  return (
    <div className={`${style['navbar-wrapper']}`}>
      <NavLink
        exact
        to="/veryobfuscatedadminpanel"
      >
        Admin Home
      </NavLink>
      <NavLink
        exact
        to="/veryobfuscatedadminpanel/score"
      >
        Score with emails
      </NavLink>
    </div>
  );
}
