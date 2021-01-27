import React, { ReactElement } from 'react';
import { Route, Switch } from 'react-router-dom';
import { Scoreboard } from '../Scoreboard/Scoreboard';
import { CTF } from '../CTF/CTF';
import { Admin } from '../Admin/Admin';

export function Main(): ReactElement {
  return (
    <main id="#content">
      <Switch>
        <Route path="/veryobfuscatedadminpanel" component={Admin} />
        <Route exact path="/score" component={Scoreboard} />
        <Route exact path="/" component={CTF} />
      </Switch>
    </main>
  );
}
