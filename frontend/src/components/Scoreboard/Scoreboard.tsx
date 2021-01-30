import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from '../../utils/Requester';
import { BasicAPI, Score } from '../../utils/types';

export function Scoreboard(): ReactElement {
  const [entries, setEntries] = useState<Array<Score>>([]);

  useEffect(() => {
    async function getEntries(): Promise<void> {
      const res = await Requester.getAllScores();
      setEntries(res.data);
    }
    void getEntries();
  }, []);

  return (
    <table className="table container">
      <thead>
        <tr>
          <th scope="col">#</th>
          <th scope="col">Name</th>
          <th scope="col">Score</th>
          <th scope="col">Answered</th>
        </tr>
      </thead>
      <tbody>
        {entries.sort((a, b) => b.score - a.score).map((e, num) => (
          <tr key={e.user.id}>
            <th scope="row">{num + 1}</th>
            <td>{e.user.name}</td>
            <td>
              {(Math.round(e.score)).toString(2)}
            </td>
            <td>

              {e.flags.map((i) => <span key={i.id} className="badge badge-secondary">{i.name}</span>)}

            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
