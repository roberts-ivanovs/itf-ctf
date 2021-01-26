import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from '../../utils/Requester';
import { BasicAPI, ScoreRaw } from '../../utils/types';

export function Scoreboard(): ReactElement {
  const [entries, setEntries] = useState<Array<ScoreRaw>>([]);

  useEffect(() => {
    async function getEntries(): Promise<void> {
      const res = await Requester.getAllScores();
      setEntries(res.data);
    }
    void getEntries();
  }, []);

  return (
    <table className="table">
      <thead>
        <tr>
          <th scope="col">#</th>
          <th scope="col">Name</th>
          <th scope="col">Score</th>
        </tr>
      </thead>
      <tbody>
        {entries.map((e, num) => (
          <tr key={e.id}>
            <th scope="row">{num + 1}</th>
            <td>{e.userId}</td>
            <td>{e.flagId}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
