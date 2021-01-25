import React from 'react';
import ReactDOM from 'react-dom';

import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';

import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap/dist/js/bootstrap.min';
import { App } from './App';

ReactDOM.render(
  <Router>
    <Switch>
      <Route
        exact
        path=""
        component={App}
      />
    </Switch>
  </Router>,
  document.getElementById('root'),
);
