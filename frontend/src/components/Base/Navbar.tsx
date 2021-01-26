import React, { ReactElement } from 'react';
import { NavLink } from 'react-router-dom';

import style from './Base.module.scss';

export function Navbar(): ReactElement {
  return (
    <div className={`${style['navbar-wrapper']}`}>
      <NavLink
        exact
        to="/"
        activeClassName="btn btn-danger"
        className="btn btn-primary"
      >
        Home
      </NavLink>
      <NavLink
        exact
        to="/score"
        activeClassName="btn btn-danger"
        className="btn btn-primary"
      >
        Score
      </NavLink>
    </div>
  );
}
