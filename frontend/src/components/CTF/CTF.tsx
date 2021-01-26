import { log } from 'console';
import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from '../../utils/Requester';
import { AnswerlessFlag } from '../../utils/types';
import { Flag } from './Flag';

async function getFlags(callback: (arg0: Array<AnswerlessFlag>) => void): Promise<void> {
  const res = await Requester.getAllFlags();
  callback(res.data);
}

export function CTF(): ReactElement {
  const [flags, setFlags] = useState<Array<AnswerlessFlag>>([]);

  useEffect(() => {
    void getFlags(setFlags);
  }, []);

  return (
    <div>
      <h2>CTF Title here</h2>
      {flags.map((e) => <Flag key={e.id} flag={e} />)}
    </div>
  );
}
