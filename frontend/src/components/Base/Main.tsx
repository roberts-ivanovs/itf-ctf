import React, { ReactElement } from 'react';
import { Route, Switch } from 'react-router-dom';
import { Scoreboard } from '../Scoreboard/Scoreboard';
import { CTF } from '../CTF/CTF';
import { About } from '../Adout/About';

export function Main(): ReactElement {
  return (
    <main id="#content">
      <Switch>
        <Route exact path="/score" component={Scoreboard} />
        <Route exact path="/about" component={About} />
        <Route exact path="/" component={CTF} />
      </Switch>
    </main>
  );
}
