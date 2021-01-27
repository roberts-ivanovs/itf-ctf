import React, { ReactElement } from 'react';
import { Route, Switch } from 'react-router-dom';
import { AdminNavbar } from './AdminNavbar';
import { EditFlag } from './EditFlag';
import { NewFlag } from './NewFlag';
import { RootAdmin } from './RootAdmin';
import { ScoreboardWithEmails } from './ScoreWithEmails';

export function Admin(): ReactElement {
  return (
    <div>

      <AdminNavbar />
      <Switch>
        <Route exact path="/veryobfuscatedadminpanel" component={RootAdmin} />
        <Route exact path="/veryobfuscatedadminpanel/new" component={NewFlag} />
        <Route exact path="/veryobfuscatedadminpanel/edit/:id" component={EditFlag} />
        <Route exact path="/veryobfuscatedadminpanel/score" component={ScoreboardWithEmails} />
      </Switch>
    </div>
  );
}
