import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from '../../utils/Requester';
import { AnswerlessFlag } from '../../utils/types';
import { Flag } from './Flag';

import style from './CTF.module.scss';

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
    <div className={`${style['content-wrapper']}`}>
      <h1>CTF Title here</h1>
      <div className={style['task-wrapper']}>
        {flags.map((e) => (
          <div key={e.id} className={`${style['flag-box']}`} data-toggle="modal" data-target={`#modal${e.id}`}>
            <p>{e.name}</p>
            <Flag id={e.id.toString()} flag={e} />
          </div>
        ))}
      </div>
    </div>
  );
}
