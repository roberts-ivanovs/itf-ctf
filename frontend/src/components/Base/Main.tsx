import React, { ReactElement } from 'react';
import { Route, Switch } from 'react-router-dom';
import { Admin } from '../Admin/Admin';
import { CTF } from '../CTF/CTF';

export function Main(): ReactElement {
  return (
    <main id="#content">
      <Switch>
        <Route path="/admin" component={Admin} />
        <Route path="/" component={CTF} />
      </Switch>
    </main>
  );
}
