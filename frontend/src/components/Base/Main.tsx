import React, { ReactElement } from 'react';
import { Route, Switch } from 'react-router-dom';
import { Scoreboard } from '../Scoreboard/Scoreboard';
import { CTF } from '../CTF/CTF';
import { Admin } from '../Admin/Admin';
import { About } from '../Adout/About';
import { Event } from '../Adout/Event';

export function Main(): ReactElement {
  return (
    <main id="#content">
      <Switch>
        <Route path="/veryobfuscatedadminpanel" component={Admin} />
        <Route exact path="/score" component={Scoreboard} />
        <Route exact path="/about" component={About} />
        <Route exact path="/linktoevent" component={Event} />
        <Route exact path="/" component={CTF} />
      </Switch>
    </main>
  );
}
